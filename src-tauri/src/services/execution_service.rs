use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::Execution;
use crate::db::repositories::execution_repo::ExecutionRepository;

pub async fn start_execution(pool: &SqlitePool, plan_id: &str) -> Result<Execution> {
    ExecutionRepository::create(pool, plan_id).await
}

pub async fn end_execution(
    pool: &SqlitePool,
    id: &str,
    actual_hours: f64,
    completion_rate: f64,
    notes: Option<String>,
    pomodoro_count: Option<i32>,
) -> Result<Execution> {
    ExecutionRepository::end(pool, id, actual_hours, completion_rate, notes, pomodoro_count).await
}

pub async fn get_executions_by_date(pool: &SqlitePool, date: i64) -> Result<Vec<Execution>> {
    let start = date;
    let end = date + 86400;
    ExecutionRepository::find_by_date_range(pool, start, end).await
}

pub async fn get_recent_executions(pool: &SqlitePool, since: i64) -> Result<Vec<Execution>> {
    ExecutionRepository::find_by_date_range(pool, since, chrono::Utc::now().timestamp()).await
}

pub async fn get_executions_by_plan_id(pool: &SqlitePool, plan_id: &str) -> Result<Vec<Execution>> {
    ExecutionRepository::find_by_plan_id(pool, plan_id).await
}

pub async fn update_execution(
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
    ExecutionRepository::update(pool, id, plan_id, start_time, end_time, actual_hours, completion_rate, notes, pomodoro_count).await
}

pub async fn delete_execution(pool: &SqlitePool, id: &str) -> Result<()> {
    ExecutionRepository::delete(pool, id).await
}
