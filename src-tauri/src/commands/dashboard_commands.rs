use tauri::State;
use crate::state::AppState;
use crate::models::dto::*;
use crate::db::repositories::{plan_repo::PlanRepository, review_repo::ReviewRepository};
use sqlx::Row;

#[tauri::command]
pub async fn get_dashboard_stats(
    state: State<'_, AppState>,
) -> Result<DashboardStats, crate::errors::AppError> {
    let pool = &*state.db;
    let now = chrono::Utc::now().timestamp();
    let week_ago = now - 7 * 86400; // 近7天

    // 本周学习总时长
    let weekly_hours: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(actual_hours), 0.0) FROM executions WHERE start_time >= ?"
    ).bind(week_ago).fetch_one(pool).await?;

    // 计划完成数
    let completed = PlanRepository::count_by_status(pool, "completed").await?;

    // 平均完成率（最近30天的执行记录）
    let avg_completion_rate: f64 = sqlx::query_scalar::<_, Option<f64>>(
        "SELECT AVG(completion_rate) FROM executions WHERE start_time >= ? AND completion_rate IS NOT NULL"
    ).bind(now - 30 * 86400).fetch_one(pool).await?.unwrap_or(0.0);

    // 复盘总数
    let reviews = ReviewRepository::count_all(pool).await?;

    // 今日待复习数
    let due_reviews_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM review_sessions WHERE scheduled_date <= ? AND actual_date IS NULL AND was_skipped = 0"
    ).bind(now).fetch_one(pool).await?;

    // 已掌握知识点数
    let mastered_kp_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM knowledge_points WHERE is_mastered = 1"
    ).fetch_one(pool).await?;

    // 本周新掌握知识点数（mastered_at 精确记录首次掌握时间）
    let new_kps_this_week: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM knowledge_points WHERE is_mastered = 1 AND mastered_at >= ?"
    ).bind(week_ago).fetch_one(pool).await?;

    // 全部科目的知识点周目标总和
    let total_weekly_goal_kps: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(weekly_goal_kps), 0) FROM subjects WHERE is_active = 1"
    ).fetch_one(pool).await?;

    Ok(DashboardStats {
        weekly_hours,
        completed_plans: completed,
        avg_completion_rate,
        review_count: reviews,
        due_reviews_count,
        mastered_kp_count,
        new_kps_this_week,
        total_weekly_goal_kps,
    })
}

#[tauri::command]
pub async fn get_subject_distribution(
    state: State<'_, AppState>,
    start_date: Option<i64>,
    end_date: Option<i64>,
) -> Result<Vec<SubjectDistribution>, crate::errors::AppError> {
    let pool = &*state.db;
    let start = start_date.unwrap_or(0);
    let end = end_date.unwrap_or(chrono::Utc::now().timestamp() + 86400);

    let rows = sqlx::query(
        r#"SELECT
            s.id as subject_id, s.name as subject_name, s.color,
            COALESCE(SUM(e.actual_hours), 0) as total_hours,
            COUNT(e.id) as execution_count
           FROM subjects s
           LEFT JOIN plans p ON p.subject_id = s.id
           LEFT JOIN executions e ON e.plan_id = p.id
               AND e.start_time >= ? AND e.start_time <= ?
           WHERE s.is_active = 1
           GROUP BY s.id
           ORDER BY total_hours DESC"#
    )
    .bind(start)
    .bind(end)
    .fetch_all(pool)
    .await?;

    let results: Vec<SubjectDistribution> = rows.iter().map(|row| {
        SubjectDistribution {
            subject_id: row.get("subject_id"),
            subject_name: row.get("subject_name"),
            color: row.get("color"),
            total_hours: row.get("total_hours"),
            execution_count: row.get("execution_count"),
        }
    }).collect();

    Ok(results)
}

#[tauri::command]
pub async fn get_weekly_trend(
    state: State<'_, AppState>,
    weeks: Option<i32>,
) -> Result<Vec<WeeklyTrend>, crate::errors::AppError> {
    let pool = &*state.db;
    let weeks = weeks.unwrap_or(4);
    let now = chrono::Utc::now().timestamp();
    let start = now - (weeks as i64 * 7 * 86400);

    let rows = sqlx::query(
        r#"SELECT
            (start_time / 86400) * 86400 as date,
            COALESCE(SUM(actual_hours), 0) as hours,
            COUNT(*) as execution_count
           FROM executions
           WHERE start_time >= ?
           GROUP BY date
           ORDER BY date ASC"#
    )
    .bind(start)
    .fetch_all(pool)
    .await?;

    let results: Vec<WeeklyTrend> = rows.iter().map(|row| {
        WeeklyTrend {
            date: row.get("date"),
            hours: row.get("hours"),
            execution_count: row.get("execution_count"),
        }
    }).collect();

    Ok(results)
}

#[tauri::command]
pub async fn get_recent_reviews(
    state: State<'_, AppState>,
    limit: Option<i64>,
) -> Result<Vec<serde_json::Value>, crate::errors::AppError> {
    let pool = &*state.db;
    let limit = limit.unwrap_or(5);

    let rows = sqlx::query(
        r#"SELECT
            r.id, r.review_date, r.mood_score, r.energy_level, r.what_went_well,
            COALESCE(p.title, '未知计划') as plan_title,
            COALESCE(s.name, '未知科目') as subject_name
           FROM reviews r
           LEFT JOIN plans p ON p.id = r.plan_id
           LEFT JOIN subjects s ON s.id = p.subject_id
           ORDER BY r.review_date DESC
           LIMIT ?"#
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    let results: Vec<serde_json::Value> = rows.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<String, _>("id"),
            "plan_title": row.get::<String, _>("plan_title"),
            "subject_name": row.get::<String, _>("subject_name"),
            "review_date": row.get::<i64, _>("review_date"),
            "mood_score": row.get::<i32, _>("mood_score"),
            "energy_level": row.get::<i32, _>("energy_level"),
            "what_went_well": row.get::<String, _>("what_went_well"),
        })
    }).collect();

    Ok(results)
}

/// 每周目标达成进度（方案B：时间+知识点复合）
#[tauri::command]
pub async fn get_weekly_goal_progress(
    state: State<'_, AppState>,
) -> Result<Vec<WeeklyGoalProgress>, crate::errors::AppError> {
    let pool = &*state.db;
    let now = chrono::Utc::now().timestamp();
    let week_ago = now - 7 * 86400;

    // 获取所有活跃科目
    let subjects = sqlx::query(
        "SELECT id, name, color, weekly_goal_hours, weekly_goal_kps FROM subjects WHERE is_active = 1"
    ).fetch_all(pool).await?;

    let mut results = Vec::new();

    for row in &subjects {
        let subj_id: String = row.get("id");
        let subj_name: String = row.get("name");
        let color: String = row.get("color");
        let goal_hours: f64 = row.get("weekly_goal_hours");
        let goal_kps: i32 = row.get("weekly_goal_kps");

        // 本周实际时长
        let weekly_hours: f64 = sqlx::query_scalar::<_, Option<f64>>(
            r#"SELECT COALESCE(SUM(e.actual_hours), 0.0)
               FROM executions e
               JOIN plans p ON e.plan_id = p.id
               WHERE p.subject_id = ? AND e.start_time >= ?"#
        ).bind(&subj_id).bind(week_ago).fetch_one(pool).await?.unwrap_or(0.0);

        // 本周新掌握知识点数（mastered_at 精确记录首次掌握时间，不会被后续复习刷新）
        let new_kps: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM knowledge_points WHERE subject_id = ? AND is_mastered = 1 AND mastered_at >= ?"
        ).bind(&subj_id).bind(week_ago).fetch_one(pool).await?;

        // 计算各项达成率
        let time_rate = if goal_hours > 0.0 {
            (weekly_hours / goal_hours).min(1.0)
        } else { 0.0 };

        let kp_rate = if goal_kps > 0 {
            (new_kps as f64 / goal_kps as f64).min(1.0)
        } else { 0.0 };

        // 综合达成率：有KP目标时50:50，否则纯时间
        let composite_rate = if goal_kps > 0 {
            time_rate * 0.5 + kp_rate * 0.5
        } else {
            time_rate
        };

        results.push(WeeklyGoalProgress {
            subject_id: subj_id,
            subject_name: subj_name,
            color,
            weekly_hours,
            goal_hours,
            new_kps,
            goal_kps,
            time_rate,
            kp_rate,
            composite_rate,
        });
    }

    // 按综合达成率升序（差的排前面）
    results.sort_by(|a, b| a.composite_rate.partial_cmp(&b.composite_rate).unwrap_or(std::cmp::Ordering::Equal));
    Ok(results)
}
