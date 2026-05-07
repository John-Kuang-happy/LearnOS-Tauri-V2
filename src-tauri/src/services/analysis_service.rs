use sqlx::{SqlitePool, Row};
use crate::errors::Result;
use crate::models::dto::{WeakSubjectAlert, WeakSubjectFactor, SubjectDistribution, WeeklyTrend};
use crate::db::repositories::insight_repo::InsightRepository;

/// 薄弱科目多因子加权分析
///
/// 对每个科目计算弱点评分，分数越高越薄弱
pub async fn analyze_weak_subjects(pool: &SqlitePool) -> Result<Vec<WeakSubjectAlert>> {
    let now = chrono::Utc::now().timestamp();
    let month_ago = now - 30 * 86400;
    let week_ago = now - 7 * 86400;

    // 获取所有活跃科目（含KP目标）
    let subjects = sqlx::query(
        "SELECT id, name, color, weekly_goal_hours, weekly_goal_kps FROM subjects WHERE is_active = 1"
    ).fetch_all(pool).await?;

    let mut alerts = Vec::new();

    for subject_row in &subjects {
        let subj_id: String = subject_row.get("id");
        let subj_name: String = subject_row.get("name");
        let subj_color: String = subject_row.get("color");
        let weekly_goal: f64 = subject_row.get("weekly_goal_hours");
        let weekly_goal_kps: i32 = subject_row.get("weekly_goal_kps");

        // 因子 1：近 30 天计划完成率（权重 0.30）
        let completion_rate: f64 = sqlx::query_scalar::<_, Option<f64>>(
            r#"SELECT AVG(COALESCE(e.completion_rate, 0))
               FROM plans p
               LEFT JOIN executions e ON e.plan_id = p.id AND e.start_time >= ?
               WHERE p.subject_id = ? AND p.status = 'completed'"#
        ).bind(month_ago).bind(&subj_id).fetch_one(pool).await?.unwrap_or(0.0);
        let low_completion = 1.0 - completion_rate.clamp(0.0, 1.0);

        // 因子 2：知识点平均掌握度（权重 0.25）
        let avg_mastery: f64 = sqlx::query_scalar(
            "SELECT COALESCE(AVG(mastery_level), 0.0) FROM knowledge_points WHERE subject_id = ?"
        ).bind(&subj_id).fetch_one(pool).await?;
        let low_mastery = 1.0 - avg_mastery.clamp(0.0, 1.0);

        // 因子 3：近 30 天复盘情绪/精力评分（权重 0.20）
        let avg_mood: f64 = sqlx::query_scalar::<_, Option<f64>>(
            r#"SELECT AVG((COALESCE(r.mood_score, 3) + COALESCE(r.energy_level, 3)) / 2.0)
               FROM reviews r
               JOIN plans p ON r.plan_id = p.id
               WHERE p.subject_id = ? AND r.review_date >= ?"#
        ).bind(&subj_id).bind(month_ago).fetch_one(pool).await?.unwrap_or(3.0);
        let poor_mood = 1.0 - (avg_mood / 5.0).clamp(0.0, 1.0);

        // 因子 4：周目标达成率（权重 0.15）—— 方案B：时间+KP复合
        let weekly_hours: f64 = sqlx::query_scalar::<_, Option<f64>>(
            r#"SELECT COALESCE(SUM(e.actual_hours), 0.0)
               FROM executions e
               JOIN plans p ON e.plan_id = p.id
               WHERE p.subject_id = ? AND e.start_time >= ?"#
        ).bind(&subj_id).bind(week_ago).fetch_one(pool).await?.unwrap_or(0.0);
        let time_rate = if weekly_goal > 0.0 {
            (weekly_hours / weekly_goal).min(1.0)
        } else { 0.0 };
        // 本周新掌握KP数（mastered_at 精确记录首次掌握时间，不会被后续复习刷新）
        let new_kps: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM knowledge_points WHERE subject_id = ? AND is_mastered = 1 AND mastered_at >= ?"
        ).bind(&subj_id).bind(week_ago).fetch_one(pool).await?;
        let kp_rate = if weekly_goal_kps > 0 {
            (new_kps as f64 / weekly_goal_kps as f64).min(1.0)
        } else { 0.0 };
        // 综合达成率：有KP目标时50:50
        let composite_rate = if weekly_goal_kps > 0 {
            time_rate * 0.5 + kp_rate * 0.5
        } else {
            time_rate
        };
        let goal_gap = (1.0 - composite_rate).clamp(0.0, 1.0);

        // 因子 5：复习逾期率（权重 0.10）
        let overdue_count: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM review_sessions rs
               JOIN knowledge_points kp ON rs.knowledge_point_id = kp.id
               WHERE kp.subject_id = ? AND rs.scheduled_date <= ? AND rs.actual_date IS NULL AND rs.was_skipped = 0"#
        ).bind(&subj_id).bind(now).fetch_one(pool).await?;
        let total_due: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM review_sessions rs
               JOIN knowledge_points kp ON rs.knowledge_point_id = kp.id
               WHERE kp.subject_id = ? AND rs.scheduled_date <= ?"#
        ).bind(&subj_id).bind(now + 7 * 86400).fetch_one(pool).await?;
        let overdue_rate = if total_due > 0 {
            overdue_count as f64 / total_due as f64
        } else { 0.0 };

        // 加权总分（0=最强，1=最弱）
        let score = 0.30 * low_completion + 0.25 * low_mastery + 0.20 * poor_mood
            + 0.15 * goal_gap + 0.10 * overdue_rate;

        let factors = vec![
            WeakSubjectFactor { name: "完成率低".into(), weight: 0.30, value: low_completion },
            WeakSubjectFactor { name: "掌握度低".into(), weight: 0.25, value: low_mastery },
            WeakSubjectFactor { name: "学习情绪".into(), weight: 0.20, value: poor_mood },
            WeakSubjectFactor { name: "未达周目标(综合)".into(), weight: 0.15, value: goal_gap },
            WeakSubjectFactor { name: "复习逾期".into(), weight: 0.10, value: overdue_rate },
        ];

        alerts.push(WeakSubjectAlert {
            subject_id: subj_id,
            subject_name: subj_name,
            color: subj_color,
            score,
            factors,
        });
    }

    // 按弱点评分降序排列
    alerts.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    Ok(alerts)
}

/// 近 N 天时间分布（按科目）
pub async fn get_time_distribution(pool: &SqlitePool, days: i64) -> Result<Vec<SubjectDistribution>> {
    let since = chrono::Utc::now().timestamp() - days * 86400;
    let rows = sqlx::query_as::<_, (String, String, String, f64, i64)>(
        r#"SELECT s.id, s.name, s.color,
           COALESCE(SUM(e.actual_hours), 0.0) as total_hours,
           COUNT(e.id) as execution_count
           FROM subjects s
           LEFT JOIN plans p ON p.subject_id = s.id
           LEFT JOIN executions e ON e.plan_id = p.id AND e.start_time >= ?
           WHERE s.is_active = 1
           GROUP BY s.id
           ORDER BY total_hours DESC"#
    ).bind(since).fetch_all(pool).await?;

    Ok(rows.into_iter().map(|(subject_id, subject_name, color, total_hours, execution_count)| {
        SubjectDistribution { subject_id, subject_name, color, total_hours, execution_count }
    }).collect())
}

/// 近 N 周效率趋势
pub async fn get_efficiency_trend(pool: &SqlitePool, weeks: i64) -> Result<Vec<WeeklyTrend>> {
    let now = chrono::Utc::now().timestamp();
    let since = now - weeks * 7 * 86400;

    let rows = sqlx::query(
        r#"SELECT CAST((e.start_time / 86400) AS INTEGER) as day_idx,
           COALESCE(SUM(e.actual_hours), 0.0) as hours,
           COUNT(e.id) as exec_count
           FROM executions e
           WHERE e.start_time >= ? AND e.actual_hours IS NOT NULL
           GROUP BY day_idx
           ORDER BY day_idx ASC"#
    ).bind(since).fetch_all(pool).await?;

    Ok(rows.iter().map(|r| {
        let day_idx: i64 = r.get("day_idx");
        WeeklyTrend {
            date: day_idx * 86400,
            hours: r.get("hours"),
            execution_count: r.get("exec_count"),
        }
    }).collect())
}

/// 生成复习提醒并保存到 insights
pub async fn generate_review_reminders(pool: &SqlitePool) -> Result<i64> {
    let now = chrono::Utc::now().timestamp();

    // 清空旧提醒
    InsightRepository::delete_by_type(pool, "review_reminder").await?;

    // 今日到期数（只统计今天排期、尚未完成的）
    let today_start = now - (now % 86400);
    let due_today: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM review_sessions WHERE scheduled_date >= ? AND scheduled_date <= ? AND actual_date IS NULL AND was_skipped = 0"
    ).bind(today_start).bind(now).fetch_one(pool).await?;

    // 逾期数（今天 0 点之前到期的）
    let overdue: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM review_sessions WHERE scheduled_date < ? AND actual_date IS NULL AND was_skipped = 0"
    ).bind(today_start).fetch_one(pool).await?;

    let mut count = 0i64;

    if due_today > 0 {
        InsightRepository::create(
            pool, "review_reminder",
            &format!("今日 {} 个知识点待复习", due_today),
            "打开艾宾浩斯看板查看待复习清单，按时复习可大幅提升记忆保留率。",
            None::<&str>, if overdue > 3 { "warning" } else { "info" }, "{}",
        ).await?;
        count += 1;
    }

    if overdue > 0 {
        InsightRepository::create(
            pool, "review_reminder",
            &format!("{} 个复习任务已逾期", overdue),
            "建议立即安排时间补上逾期复习，避免遗忘曲线加速下降。",
            None::<&str>, "warning", "{}",
        ).await?;
        count += 1;
    }

    // 薄弱科目提醒
    let weak = analyze_weak_subjects(pool).await?;
    if let Some(weakest) = weak.first() {
        if weakest.score > 0.5 {
            InsightRepository::create(
                pool, "review_reminder",
                &format!("「{}」需要更多关注", weakest.subject_name),
                &format!("综合薄弱评分 {:.0}%，建议增加该科目的学习时间。", weakest.score * 100.0),
                Some(&weakest.subject_id), "warning", "{}",
            ).await?;
            count += 1;
        }
    }

    Ok(count)
}

/// 运行完整分析：薄弱科目 + 提醒生成 + 清理过期
pub async fn run_full_analysis(pool: &SqlitePool) -> Result<Vec<WeakSubjectAlert>> {
    // 清理过期 insights
    InsightRepository::delete_expired(pool).await?;

    // 生成复习提醒
    generate_review_reminders(pool).await?;

    // 返回薄弱科目分析
    analyze_weak_subjects(pool).await
}
