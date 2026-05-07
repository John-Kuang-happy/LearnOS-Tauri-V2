use sqlx::{SqlitePool, Row};
use crate::errors::Result;
use crate::models::entity::ReviewSession;
use crate::models::dto::ReviewSessionWithKp;

pub struct ReviewSessionRepository;

impl ReviewSessionRepository {
    pub async fn create(
        pool: &SqlitePool, knowledge_point_id: &str, scheduled_date: i64,
        plan_id: Option<&str>,
    ) -> Result<ReviewSession> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let session = sqlx::query_as::<_, ReviewSession>(
            r#"INSERT INTO review_sessions (id, knowledge_point_id, plan_id,
               scheduled_date, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?) RETURNING *"#
        )
        .bind(&id).bind(knowledge_point_id).bind(plan_id)
        .bind(scheduled_date).bind(now).bind(now)
        .fetch_one(pool).await?;
        Ok(session)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<ReviewSession> {
        sqlx::query_as::<_, ReviewSession>("SELECT * FROM review_sessions WHERE id = ?")
            .bind(id).fetch_optional(pool).await?
            .ok_or_else(|| crate::errors::AppError::NotFound(format!("复习记录 {} 未找到", id)))
    }

    pub async fn find_by_kp_id(pool: &SqlitePool, kp_id: &str) -> Result<Vec<ReviewSession>> {
        let sessions = sqlx::query_as::<_, ReviewSession>(
            "SELECT * FROM review_sessions WHERE knowledge_point_id = ? ORDER BY scheduled_date DESC"
        ).bind(kp_id).fetch_all(pool).await?;
        Ok(sessions)
    }

    /// 获取到期的复习记录（已排期但未完成且未跳过）
    pub async fn find_due(pool: &SqlitePool) -> Result<Vec<ReviewSessionWithKp>> {
        let now = chrono::Utc::now().timestamp();
        let rows = sqlx::query(
            r#"SELECT rs.id as session_id, rs.knowledge_point_id,
               COALESCE(kp.name, '(已删除)') as kp_name,
               COALESCE(kp.description, '') as kp_description,
               COALESCE(kp.subject_id, '') as subject_id,
               COALESCE(s.name, '') as subject_name,
               COALESCE(s.color, '#0ea5e9') as subject_color,
               rs.scheduled_date, rs.mastery_score, rs.time_spent_seconds,
               COALESCE(rs.feedback, '') as feedback, rs.was_skipped,
               COALESCE(kp.difficulty_level, 3) as difficulty_level,
               COALESCE(kp.importance_level, 3) as importance_level,
               COALESCE(kp.review_interval_days, 1) as review_interval_days,
               COALESCE(kp.mastery_level, 0.0) as mastery_level
               FROM review_sessions rs
               LEFT JOIN knowledge_points kp ON rs.knowledge_point_id = kp.id
               LEFT JOIN subjects s ON kp.subject_id = s.id
               WHERE rs.scheduled_date <= ? AND rs.actual_date IS NULL AND rs.was_skipped = 0
               ORDER BY rs.scheduled_date ASC"#
        ).bind(now).fetch_all(pool).await?;

        Ok(rows.iter().map(|r| ReviewSessionWithKp {
            session_id: r.get("session_id"),
            knowledge_point_id: r.get("knowledge_point_id"),
            kp_name: r.get("kp_name"),
            kp_description: r.get("kp_description"),
            subject_id: r.get("subject_id"),
            subject_name: r.get("subject_name"),
            subject_color: r.get("subject_color"),
            scheduled_date: r.get("scheduled_date"),
            mastery_score: r.get("mastery_score"),
            time_spent_seconds: r.get("time_spent_seconds"),
            feedback: r.get("feedback"),
            was_skipped: r.get("was_skipped"),
            difficulty_level: r.get("difficulty_level"),
            importance_level: r.get("importance_level"),
            review_interval_days: r.get("review_interval_days"),
            mastery_level: r.get("mastery_level"),
        }).collect())
    }

    /// 获取复习记录（包含近 N 天逾期 + 未来 N 天）
    pub async fn find_upcoming(pool: &SqlitePool, days: i64) -> Result<Vec<ReviewSessionWithKp>> {
        let now = chrono::Utc::now().timestamp();
        let today_start = now - (now % 86400); // 今天 UTC 0 点
        // 下界向前推 days 天，覆盖任何时区的"今天"及逾期记录
        let start = today_start - days * 86400;
        let end = today_start + days * 86400;
        let rows = sqlx::query(
            r#"SELECT rs.id as session_id, rs.knowledge_point_id,
               kp.name as kp_name, kp.description as kp_description,
               kp.subject_id, COALESCE(s.name, '') as subject_name,
               COALESCE(s.color, '#0ea5e9') as subject_color,
               rs.scheduled_date, rs.mastery_score, rs.time_spent_seconds,
               COALESCE(rs.feedback, '') as feedback, rs.was_skipped,
               kp.difficulty_level, kp.importance_level,
               kp.review_interval_days, kp.mastery_level
               FROM review_sessions rs
               JOIN knowledge_points kp ON rs.knowledge_point_id = kp.id
               LEFT JOIN subjects s ON kp.subject_id = s.id
               WHERE rs.scheduled_date >= ? AND rs.scheduled_date < ?
               AND rs.actual_date IS NULL AND rs.was_skipped = 0
               ORDER BY rs.scheduled_date ASC"#
        ).bind(start).bind(end).fetch_all(pool).await?;

        Ok(rows.iter().map(|r| ReviewSessionWithKp {
            session_id: r.get("session_id"),
            knowledge_point_id: r.get("knowledge_point_id"),
            kp_name: r.get("kp_name"),
            kp_description: r.get("kp_description"),
            subject_id: r.get("subject_id"),
            subject_name: r.get("subject_name"),
            subject_color: r.get("subject_color"),
            scheduled_date: r.get("scheduled_date"),
            mastery_score: r.get("mastery_score"),
            time_spent_seconds: r.get("time_spent_seconds"),
            feedback: r.get("feedback"),
            was_skipped: r.get("was_skipped"),
            difficulty_level: r.get("difficulty_level"),
            importance_level: r.get("importance_level"),
            review_interval_days: r.get("review_interval_days"),
            mastery_level: r.get("mastery_level"),
        }).collect())
    }

    /// 查找已掌握知识点对应的完成复习记录
    pub async fn find_completed_by_kp(pool: &SqlitePool, kp_id: &str) -> Result<Vec<ReviewSession>> {
        let sessions = sqlx::query_as::<_, ReviewSession>(
            "SELECT * FROM review_sessions WHERE knowledge_point_id = ? AND actual_date IS NOT NULL ORDER BY actual_date DESC"
        ).bind(kp_id).fetch_all(pool).await?;
        Ok(sessions)
    }

    /// 提交复习反馈
    pub async fn submit_feedback(
        pool: &SqlitePool, id: &str, mastery_score: f64,
        time_spent_seconds: i32, feedback: &str,
    ) -> Result<ReviewSession> {
        let now = chrono::Utc::now().timestamp();
        let session = sqlx::query_as::<_, ReviewSession>(
            r#"UPDATE review_sessions SET actual_date = ?, mastery_score = ?,
               time_spent_seconds = ?, feedback = ?, updated_at = ?
               WHERE id = ? RETURNING *"#
        ).bind(now).bind(mastery_score).bind(time_spent_seconds)
         .bind(feedback).bind(now).bind(id)
        .fetch_optional(pool).await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("复习记录 {} 未找到", id)))?;
        Ok(session)
    }

    /// 跳过复习
    pub async fn skip(pool: &SqlitePool, id: &str) -> Result<ReviewSession> {
        let now = chrono::Utc::now().timestamp();
        let session = sqlx::query_as::<_, ReviewSession>(
            "UPDATE review_sessions SET was_skipped = 1, updated_at = ? WHERE id = ? RETURNING *"
        ).bind(now).bind(id)
        .fetch_optional(pool).await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("复习记录 {} 未找到", id)))?;
        Ok(session)
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM review_sessions WHERE id = ?")
            .bind(id).execute(pool).await?;
        if result.rows_affected() == 0 {
            return Err(crate::errors::AppError::NotFound(format!("复习记录 {} 未找到", id)));
        }
        Ok(())
    }

    /// 统计复习数据
    pub async fn get_stats(pool: &SqlitePool) -> Result<(i64, i64, i64, i64, f64)> {
        let now = chrono::Utc::now().timestamp();
        let week_ago = now - 7 * 86400;

        let due_today: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM review_sessions WHERE scheduled_date <= ? AND actual_date IS NULL AND was_skipped = 0"
        ).bind(now).fetch_one(pool).await?;

        let due_this_week: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM review_sessions WHERE scheduled_date > ? AND scheduled_date <= ? AND actual_date IS NULL AND was_skipped = 0"
        ).bind(now).bind(now + 7 * 86400).fetch_one(pool).await?;

        let completed_this_week: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM review_sessions WHERE actual_date >= ? AND was_skipped = 0"
        ).bind(week_ago - 1).fetch_one(pool).await?;

        let skipped_this_week: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM review_sessions WHERE updated_at >= ? AND was_skipped = 1"
        ).bind(week_ago - 1).fetch_one(pool).await?;

        let avg_mastery: Option<f64> = sqlx::query_scalar(
            "SELECT AVG(mastery_score) FROM review_sessions WHERE actual_date >= ? AND mastery_score IS NOT NULL"
        ).bind(week_ago - 1).fetch_optional(pool).await?
        .and_then(|v: Option<f64>| v);

        Ok((due_today, due_this_week, completed_this_week, skipped_this_week, avg_mastery.unwrap_or(0.0)))
    }

    /// 近 N 天每日完成复习数（热力图用）
    pub async fn get_daily_counts(pool: &SqlitePool, days: i64) -> Result<Vec<crate::models::dto::DailyReviewCount>> {
        let since = chrono::Utc::now().timestamp() - days * 86400;
        let rows = sqlx::query(
            "SELECT CAST(actual_date / 86400 AS INTEGER) as day_idx, COUNT(*) as cnt
             FROM review_sessions WHERE actual_date IS NOT NULL AND actual_date >= ?
             GROUP BY day_idx ORDER BY day_idx ASC"
        ).bind(since).fetch_all(pool).await?;

        Ok(rows.iter().map(|r| crate::models::dto::DailyReviewCount {
            date: r.get::<i64, _>("day_idx") * 86400,
            count: r.get::<i32, _>("cnt"),
        }).collect())
    }
}
