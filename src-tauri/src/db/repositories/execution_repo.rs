use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::Execution;

pub struct ExecutionRepository;

impl ExecutionRepository {
    pub async fn create(pool: &SqlitePool, plan_id: &str) -> Result<Execution> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        let execution = sqlx::query_as::<_, Execution>(
            r#"INSERT INTO executions (id, plan_id, start_time, pomodoro_count, created_at, updated_at)
               VALUES (?, ?, ?, 1, ?, ?)
               RETURNING *"#
        )
        .bind(&id)
        .bind(plan_id)
        .bind(now)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await?;

        Ok(execution)
    }

    pub async fn end(
        pool: &SqlitePool,
        id: &str,
        actual_hours: f64,
        completion_rate: f64,
        notes: Option<String>,
        pomodoro_count: Option<i32>,
    ) -> Result<Execution> {
        let now = chrono::Utc::now().timestamp();

        let execution = sqlx::query_as::<_, Execution>(
            r#"UPDATE executions SET
               end_time = ?, actual_hours = ?, completion_rate = ?, notes = ?,
               pomodoro_count = COALESCE(?, pomodoro_count),
               updated_at = ?
               WHERE id = ?
               RETURNING *"#
        )
        .bind(now)
        .bind(actual_hours)
        .bind(completion_rate)
        .bind(&notes)
        .bind(pomodoro_count)
        .bind(now)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("执行记录 {} 未找到", id)))?;

        Ok(execution)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Execution> {
        let execution = sqlx::query_as::<_, Execution>(
            "SELECT * FROM executions WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("执行记录 {} 未找到", id)))?;

        Ok(execution)
    }

    pub async fn find_by_date_range(pool: &SqlitePool, start: i64, end: i64) -> Result<Vec<Execution>> {
        let executions = sqlx::query_as::<_, Execution>(
            "SELECT * FROM executions WHERE start_time >= ? AND start_time <= ? ORDER BY start_time DESC"
        )
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await?;
        Ok(executions)
    }

    pub async fn find_by_plan_id(pool: &SqlitePool, plan_id: &str) -> Result<Vec<Execution>> {
        let executions = sqlx::query_as::<_, Execution>(
            "SELECT * FROM executions WHERE plan_id = ? ORDER BY start_time DESC"
        )
        .bind(plan_id)
        .fetch_all(pool)
        .await?;
        Ok(executions)
    }

    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        plan_id: Option<String>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        actual_hours: Option<f64>,
        completion_rate: Option<f64>,
        notes: Option<String>,
        pomodoro_count: Option<i32>,
    ) -> Result<Execution> {
        let now = chrono::Utc::now().timestamp();

        let execution = sqlx::query_as::<_, Execution>(
            r#"UPDATE executions SET
               plan_id = COALESCE(?, plan_id),
               start_time = COALESCE(?, start_time),
               end_time = COALESCE(?, end_time),
               actual_hours = COALESCE(?, actual_hours),
               completion_rate = COALESCE(?, completion_rate),
               notes = COALESCE(?, notes),
               pomodoro_count = COALESCE(?, pomodoro_count),
               updated_at = ?
               WHERE id = ?
               RETURNING *"#
        )
        .bind(plan_id)
        .bind(start_time)
        .bind(end_time)
        .bind(actual_hours)
        .bind(completion_rate)
        .bind(notes)
        .bind(pomodoro_count)
        .bind(now)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("执行记录 {} 未找到", id)))?;

        Ok(execution)
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM executions WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::errors::AppError::NotFound(format!("执行记录 {} 未找到", id)));
        }
        Ok(())
    }

    /// 获取所有有完成率的执行记录（用于统计）
    pub async fn find_all_with_completion_rate(pool: &SqlitePool) -> Result<Vec<Execution>> {
        let executions = sqlx::query_as::<_, Execution>(
            "SELECT * FROM executions WHERE completion_rate IS NOT NULL ORDER BY start_time DESC"
        )
        .fetch_all(pool)
        .await?;
        Ok(executions)
    }
}
