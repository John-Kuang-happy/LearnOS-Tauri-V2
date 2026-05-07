use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::Insight;
use crate::db::repositories::insight_repo::InsightRepository;

/// 基于规则的学习建议引擎
///
/// 通过分析现有数据生成个性化学习建议，保存到 insights 表。
/// 无需网络连接，始终可用。
pub async fn generate_suggestions(pool: &SqlitePool) -> Result<Vec<Insight>> {
    // 清空旧的建议类 insights
    InsightRepository::delete_by_type(pool, "suggestion").await?;

    let now = chrono::Utc::now().timestamp();
    let week_ago = now - 7 * 86400;
    let mut suggestions = Vec::new();

    // ---- 规则 1：薄弱科目建议 ----
    let weak_subjects = super::analysis_service::analyze_weak_subjects(pool).await?;
    for ws in &weak_subjects {
        if ws.score > 0.5 {
            let title = format!("加强「{}」的学习力度", ws.subject_name);
            let content = if ws.score > 0.7 {
                format!(
                    "「{}」综合薄弱评分 {:.0}%，建议：\n1. 每天额外安排 30 分钟专项练习\n2. 优先复习该科目的未掌握知识点\n3. 每周做一次错题回顾",
                    ws.subject_name, ws.score * 100.0
                )
            } else {
                format!(
                    "「{}」综合薄弱评分 {:.0}%，建议适当增加学习时间和知识点复习频次。",
                    ws.subject_name, ws.score * 100.0
                )
            };
            let severity = if ws.score > 0.7 { "warning" } else { "info" };
            let insight = InsightRepository::create(
                pool, "suggestion", &title, &content,
                Some(&ws.subject_id), severity, "{}",
            ).await?;
            suggestions.push(insight);
        }
    }

    // ---- 规则 2：学习时长趋势 ----
    let this_week_hours: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(actual_hours), 0.0) FROM executions WHERE start_time >= ?"
    ).bind(week_ago).fetch_one(pool).await?;

    let last_week_hours: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(actual_hours), 0.0) FROM executions WHERE start_time >= ? AND start_time < ?"
    ).bind(week_ago - 7 * 86400).bind(week_ago).fetch_one(pool).await?;

    if this_week_hours < last_week_hours * 0.5 && last_week_hours > 1.0 {
        let insight = InsightRepository::create(
            pool, "suggestion",
            "本周学习时长明显下降",
            &format!(
                "本周 {:.1}h vs 上周 {:.1}h，下降 {:.0}%。建议检查是否有未完成的学习计划，或重新调整本周目标。",
                this_week_hours, last_week_hours,
                (1.0 - this_week_hours / last_week_hours.max(0.01)) * 100.0
            ),
            None::<&str>, "warning", "{}",
        ).await?;
        suggestions.push(insight);
    }

    if this_week_hours > last_week_hours * 1.5 && last_week_hours > 1.0 {
        let insight = InsightRepository::create(
            pool, "suggestion",
            "本周学习状态良好！",
            &format!("本周 {:.1}h 相比上周 {:.1}h 有明显提升，保持这个节奏！", this_week_hours, last_week_hours),
            None::<&str>, "success", "{}",
        ).await?;
        suggestions.push(insight);
    }

    // ---- 规则 3：复习逾期提醒 ----
    let today_start = now - (now % 86400);
    let overdue: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM review_sessions WHERE scheduled_date < ? AND actual_date IS NULL AND was_skipped = 0"
    ).bind(today_start).fetch_one(pool).await?;

    if overdue > 5 {
        let insight = InsightRepository::create(
            pool, "suggestion",
            &format!("{} 个复习任务已逾期", overdue),
            "逾期复习会导致遗忘曲线加速下降。建议今天集中时间完成逾期复习，或在艾宾浩斯看板中跳过非重点项。",
            None::<&str>, "warning", "{}",
        ).await?;
        suggestions.push(insight);
    }

    // ---- 规则 4：知识点掌握里程碑 ----
    let total_kps: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM knowledge_points")
        .fetch_one(pool).await?;
    let mastered_kps: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM knowledge_points WHERE is_mastered = 1"
    ).fetch_one(pool).await?;

    if total_kps > 0 && mastered_kps as f64 / total_kps as f64 > 0.6 {
        let insight = InsightRepository::create(
            pool, "suggestion",
            &format!("已掌握 {}/{} 个知识点，表现优秀！", mastered_kps, total_kps),
            "超过 60% 的知识点已标记为掌握。建议开始做综合模拟题，检验真实应试水平。",
            None::<&str>, "success", "{}",
        ).await?;
        suggestions.push(insight);
    } else if total_kps > 0 && mastered_kps == 0 {
        let insight = InsightRepository::create(
            pool, "suggestion",
            "开始你的第一个复习周期",
            "所有知识点尚未开始复习。每天复习 2-3 个知识点并提交掌握评分，系统会自动调整复习间隔。",
            None::<&str>, "info", "{}",
        ).await?;
        suggestions.push(insight);
    }

    // ---- 规则 5：科目学习分布 ----
    let dist = super::analysis_service::get_time_distribution(pool, 30).await?;
    if dist.len() >= 3 {
        let max_h = dist[0].total_hours;
        if dist.len() >= 2 {
            let min_h = dist[dist.len() - 1].total_hours;
            if max_h > 0.0 && min_h < max_h * 0.15 {
                let least_subject = &dist[dist.len() - 1].subject_name;
                let insight = InsightRepository::create(
                    pool, "suggestion",
                    &format!("「{}」近 30 天学习时间偏少", least_subject),
                    "学习时间过于集中在少数科目，建议均衡分配，确保每个科目都有足够的复习时间。",
                    None::<&str>, "info", "{}",
                ).await?;
                suggestions.push(insight);
            }
        }
    }

    log::info!("生成了 {} 条学习建议", suggestions.len());
    Ok(suggestions)
}
