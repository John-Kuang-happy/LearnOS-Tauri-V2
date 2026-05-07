use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::KnowledgePoint;
use crate::models::dto::CreateKnowledgePointRequest;

pub struct KnowledgePointRepository;

impl KnowledgePointRepository {
    pub async fn create(pool: &SqlitePool, req: CreateKnowledgePointRequest) -> Result<KnowledgePoint> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let description = req.description.unwrap_or_default();
        let difficulty = req.difficulty_level.unwrap_or(3);
        let importance = req.importance_level.unwrap_or(3);
        let source = req.source.unwrap_or_default();

        let kp = sqlx::query_as::<_, KnowledgePoint>(
            r#"INSERT INTO knowledge_points (id, subject_id, name, description,
               difficulty_level, importance_level, mastery_level, review_interval_days,
               ease_factor, review_history, source, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, 0.0, 1, 2.5, '[]', ?, ?, ?)
               RETURNING *"#
        )
        .bind(&id).bind(&req.subject_id).bind(&req.name).bind(&description)
        .bind(difficulty).bind(importance).bind(&source).bind(now).bind(now)
        .fetch_one(pool).await?;

        Ok(kp)
    }

    pub async fn find_by_subject(pool: &SqlitePool, subject_id: &str) -> Result<Vec<KnowledgePoint>> {
        let kps = sqlx::query_as::<_, KnowledgePoint>(
            "SELECT * FROM knowledge_points WHERE subject_id = ? ORDER BY created_at DESC"
        ).bind(subject_id).fetch_all(pool).await?;
        Ok(kps)
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<KnowledgePoint>> {
        let kps = sqlx::query_as::<_, KnowledgePoint>(
            "SELECT * FROM knowledge_points ORDER BY mastery_level ASC, created_at DESC"
        ).fetch_all(pool).await?;
        Ok(kps)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<KnowledgePoint> {
        let kp = sqlx::query_as::<_, KnowledgePoint>(
            "SELECT * FROM knowledge_points WHERE id = ?"
        ).bind(id).fetch_optional(pool).await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("知识点 {} 未找到", id)))?;
        Ok(kp)
    }

    pub async fn update(
        pool: &SqlitePool, id: &str,
        name: Option<String>, description: Option<String>,
        difficulty_level: Option<i32>, importance_level: Option<i32>,
        is_mastered: Option<bool>, source: Option<String>,
    ) -> Result<KnowledgePoint> {
        let now = chrono::Utc::now().timestamp();
        let kp = sqlx::query_as::<_, KnowledgePoint>(
            r#"UPDATE knowledge_points SET
               name = COALESCE(?, name), description = COALESCE(?, description),
               difficulty_level = COALESCE(?, difficulty_level),
               importance_level = COALESCE(?, importance_level),
               is_mastered = COALESCE(?, is_mastered),
               source = COALESCE(?, source), updated_at = ?
               WHERE id = ? RETURNING *"#
        )
        .bind(name).bind(description).bind(difficulty_level).bind(importance_level)
        .bind(is_mastered.map(|v| v as i32)).bind(source).bind(now).bind(id)
        .fetch_optional(pool).await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("知识点 {} 未找到", id)))?;
        Ok(kp)
    }

    pub async fn update_mastery(
        pool: &SqlitePool, id: &str, mastery_level: f64,
        interval_days: i64, ease_factor: f64, consecutive_correct: i32,
        next_review_at: i64, is_mastered: bool,
    ) -> Result<KnowledgePoint> {
        let now = chrono::Utc::now().timestamp();
        let new_mastered = if is_mastered { 1 } else { 0 };
        let kp = sqlx::query_as::<_, KnowledgePoint>(
            r#"UPDATE knowledge_points SET mastery_level = ?, review_interval_days = ?,
               ease_factor = ?, consecutive_correct = ?, next_review_at = ?,
               last_reviewed_at = ?, review_count = review_count + 1,
               is_mastered = ?,
               mastered_at = CASE WHEN is_mastered = 0 AND ? = 1 THEN ? ELSE mastered_at END,
               updated_at = ?
               WHERE id = ? RETURNING *"#
        )
        .bind(mastery_level).bind(interval_days as i32).bind(ease_factor)
        .bind(consecutive_correct).bind(next_review_at).bind(now)
        .bind(new_mastered).bind(new_mastered).bind(now).bind(now).bind(id)
        .fetch_optional(pool).await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("知识点 {} 未找到", id)))?;
        Ok(kp)
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM knowledge_points WHERE id = ?")
            .bind(id).execute(pool).await?;
        if result.rows_affected() == 0 {
            return Err(crate::errors::AppError::NotFound(format!("知识点 {} 未找到", id)));
        }
        Ok(())
    }

    /// 获取待复习的知识点（next_review_at <= now）
    pub async fn find_due_reviews(pool: &SqlitePool) -> Result<Vec<KnowledgePoint>> {
        let now = chrono::Utc::now().timestamp();
        let kps = sqlx::query_as::<_, KnowledgePoint>(
            "SELECT * FROM knowledge_points WHERE is_mastered = 0 AND next_review_at IS NOT NULL AND next_review_at <= ? ORDER BY next_review_at ASC"
        ).bind(now).fetch_all(pool).await?;
        Ok(kps)
    }

    pub async fn count_mastered(pool: &SqlitePool) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM knowledge_points WHERE is_mastered = 1"
        ).fetch_one(pool).await?;
        Ok(count)
    }
}
