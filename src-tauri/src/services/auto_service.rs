use sqlx::{SqlitePool, Row};
use chrono::Timelike;
use crate::errors::Result;
use crate::models::dto::*;
use crate::models::entity::{Plan, Exam};
use crate::db::repositories::knowledge_point_repo::KnowledgePointRepository;
use crate::db::repositories::review_session_repo::ReviewSessionRepository;

/// 每日签到：生成今日概览
pub async fn daily_checkin(pool: &SqlitePool) -> Result<DailyCheckin> {
    let now = chrono::Utc::now().timestamp();
    let today_start = now - (now % 86400);
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    // 今日已复习知识点数
    let today_reviews: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM review_sessions WHERE actual_date >= ? AND was_skipped = 0"
    ).bind(today_start).fetch_one(pool).await?;

    // 今日已学习分钟数
    let today_minutes: i64 = sqlx::query_scalar(
        "SELECT COALESCE(CAST(SUM(time_spent_seconds) AS INTEGER), 0) FROM review_sessions WHERE actual_date >= ?"
    ).bind(today_start).fetch_one(pool).await?;

    // 今日完成执行数
    let today_completed: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM executions WHERE end_time >= ?"
    ).bind(today_start).fetch_one(pool).await?;

    // 待复习数
    let due_review_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM review_sessions WHERE scheduled_date <= ? AND actual_date IS NULL AND was_skipped = 0"
    ).bind(now).fetch_one(pool).await?;

    // 待办计划数
    let pending_plan_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM plans WHERE status IN ('pending', 'in_progress')"
    ).fetch_one(pool).await?;

    // 生成问候和建议
    let hour = chrono::Utc::now().with_timezone(&chrono::Local).time().hour();
    let greeting = if hour < 12 { "早上好 ☀️" }
        else if hour < 18 { "下午好 🌤️" }
        else { "晚上好 🌙" };

    let suggestion = if due_review_count > 0 {
        format!("今天有 {} 个知识点待复习，建议先从艾宾浩斯看板开始。", due_review_count)
    } else if pending_plan_count > 0 {
        format!("有 {} 个计划待完成，选一个开始吧！", pending_plan_count)
    } else {
        "今天还没有学习计划，要不要创建一个？".into()
    };

    Ok(DailyCheckin {
        date: today,
        greeting: greeting.to_string(),
        due_review_count,
        pending_plan_count,
        today_completed,
        today_minutes: today_minutes / 60,
        suggestion,
    })
}

/// 番茄钟（执行）结束后的全自动联动
///
/// 1. 更新计划状态 pending/in_progress → completed
/// 2. 如果有反馈则创建轻量复盘
/// 3. 如果计划启用了 auto_review_enabled，生成复习计划
/// 4. 更新关联知识点掌握度 + 创建下次复习 session
pub async fn on_execution_complete(
    pool: &SqlitePool,
    execution_id: &str,
    feedback: Option<ExecutionFeedback>,
) -> Result<ExecutionCompleteResult> {
    let now = chrono::Utc::now().timestamp();

    // 查找执行记录
    let exec = sqlx::query("SELECT * FROM executions WHERE id = ?")
        .bind(execution_id).fetch_optional(pool).await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("执行记录 {} 未找到", execution_id)))?;

    let plan_id: String = exec.get("plan_id");
    let actual_hours: Option<f64> = exec.get("actual_hours");
    let completion_rate: Option<f64> = exec.get("completion_rate");

    let mut result = ExecutionCompleteResult {
        plan_updated: false,
        review_created: false,
        kp_updated: false,
        message: String::new(),
        next_review_date: None,
    };

    // 1. 更新计划状态
    let plan: Option<Plan> = sqlx::query_as::<_, Plan>(
        "SELECT * FROM plans WHERE id = ?"
    ).bind(&plan_id).fetch_optional(pool).await?;

    if let Some(_plan) = &plan {
        sqlx::query("UPDATE plans SET status = 'completed', updated_at = ? WHERE id = ?")
            .bind(now).bind(&plan_id).execute(pool).await?;
        result.plan_updated = true;
    }

    // 2. 创建轻量复盘（如果有反馈）
    if let Some(ref fb) = feedback {
        if fb.mood_score.is_some() || fb.notes.is_some() {
            let review_id = uuid::Uuid::new_v4().to_string();
            let mood = fb.mood_score.unwrap_or(3);
            sqlx::query(
                r#"INSERT INTO reviews (id, plan_id, review_date, what_went_well, what_to_improve,
                   action_items, mood_score, energy_level, created_at, updated_at)
                   VALUES (?, ?, ?, '', '', '', ?, 3, ?, ?)"#
            ).bind(&review_id).bind(&plan_id).bind(now).bind(mood).bind(now).bind(now)
            .execute(pool).await?;
            result.review_created = true;
        }
    }

    // 3. 更新关联知识点掌握度（通过 source 字段查找所有关联到本计划的 KP）
    if let Some(ref p) = plan {
        if let Some(ref fb) = feedback {
            if let Some(mastery) = fb.mastery_score {
                // 查找所有 source = plan_id 的知识点
                let linked_kps = sqlx::query(
                    "SELECT id FROM knowledge_points WHERE source = ?"
                ).bind(&plan_id).fetch_all(pool).await?;

                // 如果没有通过 source 关联的，尝试用 source_kp_id（向后兼容）
                let kp_ids: Vec<String> = if linked_kps.is_empty() {
                    if let Some(ref kp_id) = p.source_kp_id {
                        vec![kp_id.clone()]
                    } else {
                        vec![]
                    }
                } else {
                    linked_kps.iter().map(|r| r.get::<String, _>("id")).collect()
                };

                for kp_id in &kp_ids {
                    let kp = KnowledgePointRepository::find_by_id(pool, kp_id).await?;

                    // 连续正确计数：≥ 0.65 算一次有效学习
                    let new_consecutive = if mastery >= 0.65 {
                        kp.consecutive_correct + 1
                    } else { kp.consecutive_correct };

                    let new_interval = if mastery >= 0.65 {
                        let ef = (2.5 - 0.15 * (kp.difficulty_level as f64) + 0.1 * (kp.consecutive_correct as f64)).clamp(1.3, 3.0);
                        std::cmp::max(1, ((kp.review_interval_days as f64) * ef).round() as i32)
                    } else {
                        std::cmp::max(1, ((kp.review_interval_days as f64) * 0.5).round() as i32)
                    };

                    let next_review_at = now + new_interval as i64 * 86400;
                    // 掌握门槛：「很扎实」1次掌握，「掌握了」需连续2次
                    let is_mastered = (mastery >= 0.9 && new_consecutive >= 1)
                        || (mastery >= 0.65 && new_consecutive >= 2);

                    KnowledgePointRepository::update_mastery(
                        pool, kp_id, mastery, new_interval as i64,
                        kp.ease_factor, new_consecutive, next_review_at, is_mastered,
                    ).await?;

                    if !is_mastered {
                        // 检查是否已有待处理的复习会话
                        let pending: i64 = sqlx::query_scalar(
                            "SELECT COUNT(*) FROM review_sessions WHERE knowledge_point_id = ? AND actual_date IS NULL AND was_skipped = 0"
                        ).bind(kp_id).fetch_one(pool).await?;
                        if pending == 0 {
                            ReviewSessionRepository::create(
                                pool, kp_id, next_review_at, Some(&plan_id),
                            ).await?;
                        }
                    }

                    result.kp_updated = true;
                    result.next_review_date = chrono::NaiveDateTime::from_timestamp_opt(next_review_at, 0)
                        .map(|dt| dt.format("%m月%d日").to_string());
                }
            }
        }
    }

    // 5. 生成结果消息
    let mut msgs = Vec::new();
    if result.plan_updated { msgs.push("已标记计划为完成".to_string()); }
    if result.review_created { msgs.push("已创建轻量复盘".to_string()); }
    if result.kp_updated {
        if let Some(ref date) = result.next_review_date {
            msgs.push(format!("已更新知识点掌握度，下次复习：{}", date));
        } else {
            msgs.push("已更新知识点掌握度".to_string());
        }
    }

    result.message = if msgs.is_empty() {
        "番茄钟已完成。".to_string()
    } else {
        msgs.join("\n")
    };

    Ok(result)
}

/// 获取今日 AI 推荐学习顺序
///
/// 合并待复习 + 今日计划，按优先级排序：
/// 1. 逾期复习（按逾期天数降序）
/// 2. 今日到期复习（按掌握度升序）
/// 3. 今日计划（按优先级 + 考试关联度）
pub async fn get_today_recommendations(pool: &SqlitePool) -> Result<TodayRecommendation> {
    let now = chrono::Utc::now().timestamp();
    let today_start = now - (now % 86400);
    let today_end = today_start + 86400;
    log::info!("[今日推荐] 查询范围: today_start={}, today_end={}, now={}", today_start, today_end, now);

    // 1. 获取待复习
    let due_reviews = ReviewSessionRepository::find_due(pool).await?;
    log::info!("[今日推荐] 待复习任务数: {}", due_reviews.len());

    // 2. 获取今日计划（pending/in_progress 且在今天的范围内）
    let today_plans = sqlx::query_as::<_, Plan>(
        "SELECT * FROM plans WHERE status IN ('pending', 'in_progress') AND start_date < ? AND end_date >= ? ORDER BY priority ASC"
    ).bind(today_end).bind(today_start).fetch_all(pool).await?;
    log::info!("[今日推荐] 今日计划数: {}", today_plans.len());

    // 3. 获取今日已完成分钟数
    let completed_minutes: i64 = sqlx::query_scalar(
        "SELECT COALESCE(CAST(SUM(COALESCE(actual_hours,0) * 60) AS INTEGER), 0) FROM executions WHERE end_time >= ?"
    ).bind(today_start).fetch_one(pool).await?;
    log::info!("[今日推荐] 已完成分钟数: {}", completed_minutes);

    // 4. 获取最近考试（用于优先级加成）
    let exams = sqlx::query_as::<_, Exam>(
        "SELECT * FROM exams WHERE is_active = 1 AND target_date >= ? ORDER BY target_date ASC LIMIT 5"
    ).bind(now).fetch_all(pool).await?;

    let nearest_exam_days = exams.first()
        .map(|e| std::cmp::max(0, (e.target_date - now) / 86400))
        .unwrap_or(999);

    // 5. 构建推荐项
    let mut items: Vec<RecommendedItem> = Vec::new();

    for review in &due_reviews {
        let overdue_days = std::cmp::max(0, (now - review.scheduled_date) / 86400) as i32;
        let estimated = 25; // 复习默认 25 分钟

        let priority = (overdue_days * 100)
            + ((1.0 - review.mastery_level) * 50.0) as i32;

        let reason = if overdue_days > 0 {
            format!("已逾期 {} 天", overdue_days)
        } else {
            "今日到期".to_string()
        };

        items.push(RecommendedItem {
            id: review.session_id.clone(),
            item_type: "review".to_string(),
            title: review.kp_name.clone(),
            subject_name: review.subject_name.clone(),
            subject_color: review.subject_color.clone(),
            mastery_level: review.mastery_level,
            overdue_days,
            estimated_minutes: estimated,
            priority,
            reason,
        });
    }

    for plan in &today_plans {
        let estimated = (plan.estimated_hours * 60.0) as i32;
        let subject_name = sqlx::query_scalar::<_, String>(
            "SELECT COALESCE(name, '未归类') FROM subjects WHERE id = ?"
        ).bind(&plan.subject_id).fetch_optional(pool).await?
        .unwrap_or_else(|| "未归类".to_string());

        let subject_color = sqlx::query_scalar::<_, String>(
            "SELECT COALESCE(color, '#94a3b8') FROM subjects WHERE id = ?"
        ).bind(&plan.subject_id).fetch_optional(pool).await?
        .unwrap_or_else(|| "#94a3b8".to_string());

        // 关联考试加分：如果该科目出现在考试中，按考试临近程度加分
        let exam_bonus = if nearest_exam_days < 30 { 50 } else if nearest_exam_days < 90 { 20 } else { 0 };
        let priority = (5 - plan.priority) * 30 + exam_bonus;

        let priority_label = ["", "紧急", "高", "中", "低"]
            .get(plan.priority as usize)
            .copied()
            .unwrap_or("未知");
        let reason = if exam_bonus > 0 {
            format!("优先级 {} · 距考试 {} 天", priority_label, nearest_exam_days)
        } else {
            format!("优先级 {}", priority_label)
        };

        items.push(RecommendedItem {
            id: plan.id.clone(),
            item_type: "plan".to_string(),
            title: plan.title.clone(),
            subject_name,
            subject_color,
            mastery_level: 0.0,
            overdue_days: 0,
            estimated_minutes: estimated,
            priority,
            reason,
        });
    }

    // 6. 按优先级降序排列
    items.sort_by(|a, b| b.priority.cmp(&a.priority));

    // 7. 计算今日总预估时长
    let total_estimated_minutes: i64 = items.iter().map(|i| i.estimated_minutes as i64).sum();

    // 8. 生成建议
    let suggestion = if items.is_empty() {
        "今天没有待办任务，要不要创建一个计划？".to_string()
    } else if let Some(top) = items.first() {
        if top.item_type == "review" && top.overdue_days > 0 {
            format!("建议优先复习「{}」已逾期 {} 天，掌握度仅 {}%", top.title, top.overdue_days, (top.mastery_level * 100.0) as i32)
        } else {
            format!("建议从「{}」开始", top.title)
        }
    } else {
        "开始今天的学习吧！".to_string()
    };

    let result = TodayRecommendation {
        total_estimated_minutes,
        completed_minutes,
        items,
        suggestion,
    };
    log::info!("[今日推荐] 最终结果: items={}, total_min={}, completed_min={}", result.items.len(), result.total_estimated_minutes, result.completed_minutes);
    Ok(result)
}
