use sqlx::{SqlitePool, Row};
use crate::errors::Result;

pub struct SettingsRepository;

impl SettingsRepository {
    pub async fn get_value(pool: &SqlitePool, key: &str) -> Result<Option<String>> {
        let result = sqlx::query_scalar::<_, String>(
            "SELECT value FROM settings WHERE key = ?"
        )
        .bind(key)
        .fetch_optional(pool)
        .await?;
        Ok(result)
    }

    pub async fn set_value(pool: &SqlitePool, key: &str, value: &str) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        sqlx::query(
            "INSERT INTO settings (key, value, created_at, updated_at) VALUES (?, ?, ?, ?)
             ON CONFLICT(key) DO UPDATE SET value = ?, updated_at = ?"
        )
        .bind(key)
        .bind(value)
        .bind(now)
        .bind(now)
        .bind(value)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<(String, String)>> {
        let rows = sqlx::query("SELECT key, value FROM settings")
            .fetch_all(pool)
            .await?;

        let results: Vec<(String, String)> = rows.iter().map(|row| {
            let key: String = row.get("key");
            let value: String = row.get("value");
            (key, value)
        }).collect();

        Ok(results)
    }

    /// 清空所有数据（保留表结构，在事务中执行以保证原子性）
    pub async fn delete_all(pool: &SqlitePool) -> Result<()> {
        let mut tx = pool.begin().await?;
        sqlx::query("DELETE FROM records").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM executions").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM review_sessions").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM reviews").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM insights").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM plans").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM knowledge_points").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM exams").execute(&mut *tx).await?;
        sqlx::query("DELETE FROM subjects").execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }
}
