use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::Insight;

pub struct InsightRepository;

impl InsightRepository {
    pub async fn create(
        pool: &SqlitePool, insight_type: &str, title: &str, content: &str,
        related_subject_id: Option<&str>, severity: &str, metadata: &str,
    ) -> Result<Insight> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let expires_at = now + 7 * 86400; // 7 天后过期

        let insight = sqlx::query_as::<_, Insight>(
            r#"INSERT INTO insights (id, insight_type, title, content,
               related_subject_id, severity, is_read, generated_at, expires_at,
               metadata, created_at)
               VALUES (?, ?, ?, ?, ?, ?, 0, ?, ?, ?, ?) RETURNING *"#
        )
        .bind(&id).bind(insight_type).bind(title).bind(content)
        .bind(related_subject_id).bind(severity).bind(now).bind(expires_at)
        .bind(metadata).bind(now)
        .fetch_one(pool).await?;
        Ok(insight)
    }

    /// 创建永不过期的里程碑（成就类记录）
    pub async fn create_permanent(
        pool: &SqlitePool, insight_type: &str, title: &str, content: &str,
        related_subject_id: Option<&str>, severity: &str, metadata: &str,
    ) -> Result<Insight> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        let insight = sqlx::query_as::<_, Insight>(
            r#"INSERT INTO insights (id, insight_type, title, content,
               related_subject_id, severity, is_read, generated_at, expires_at,
               metadata, created_at)
               VALUES (?, ?, ?, ?, ?, ?, 0, ?, NULL, ?, ?) RETURNING *"#
        )
        .bind(&id).bind(insight_type).bind(title).bind(content)
        .bind(related_subject_id).bind(severity).bind(now)
        .bind(metadata).bind(now)
        .fetch_one(pool).await?;
        Ok(insight)
    }

    pub async fn find_by_type(pool: &SqlitePool, insight_type: &str) -> Result<Vec<Insight>> {
        let insights = sqlx::query_as::<_, Insight>(
            "SELECT * FROM insights WHERE insight_type = ? ORDER BY generated_at DESC"
        ).bind(insight_type).fetch_all(pool).await?;
        Ok(insights)
    }

    pub async fn find_unread(pool: &SqlitePool) -> Result<Vec<Insight>> {
        let now = chrono::Utc::now().timestamp();
        let insights = sqlx::query_as::<_, Insight>(
            "SELECT * FROM insights WHERE is_read = 0 AND (expires_at IS NULL OR expires_at > ?) ORDER BY generated_at DESC"
        ).bind(now).fetch_all(pool).await?;
        Ok(insights)
    }

    pub async fn find_recent(pool: &SqlitePool, limit: i64) -> Result<Vec<Insight>> {
        let now = chrono::Utc::now().timestamp();
        let insights = sqlx::query_as::<_, Insight>(
            "SELECT * FROM insights WHERE expires_at IS NULL OR expires_at > ? ORDER BY generated_at DESC LIMIT ?"
        ).bind(now).bind(limit).fetch_all(pool).await?;
        Ok(insights)
    }

    pub async fn mark_read(pool: &SqlitePool, id: &str) -> Result<()> {
        sqlx::query("UPDATE insights SET is_read = 1 WHERE id = ?")
            .bind(id).execute(pool).await?;
        Ok(())
    }

    pub async fn mark_all_read(pool: &SqlitePool) -> Result<()> {
        sqlx::query("UPDATE insights SET is_read = 1 WHERE is_read = 0")
            .execute(pool).await?;
        Ok(())
    }

    pub async fn delete_expired(pool: &SqlitePool) -> Result<i64> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query("DELETE FROM insights WHERE expires_at IS NOT NULL AND expires_at <= ?")
            .bind(now).execute(pool).await?;
        Ok(result.rows_affected() as i64)
    }

    pub async fn delete_by_type(pool: &SqlitePool, insight_type: &str) -> Result<i64> {
        let result = sqlx::query("DELETE FROM insights WHERE insight_type = ?")
            .bind(insight_type).execute(pool).await?;
        Ok(result.rows_affected() as i64)
    }
}
