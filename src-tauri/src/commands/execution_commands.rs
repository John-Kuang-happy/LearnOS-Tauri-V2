use tauri::State;
use crate::state::AppState;
use crate::models::entity::Execution;
use crate::models::dto::*;
use crate::services;

#[tauri::command]
pub async fn start_execution(
    state: State<'_, AppState>,
    req: StartExecutionRequest,
) -> Result<Execution, crate::errors::AppError> {
    services::execution_service::start_execution(&state.db, &req.plan_id).await
}

#[tauri::command]
pub async fn end_execution(
    state: State<'_, AppState>,
    id: String,
    data: EndExecutionRequest,
) -> Result<Execution, crate::errors::AppError> {
    services::execution_service::end_execution(
        &state.db, &id, data.actual_hours, data.completion_rate, data.notes, data.pomodoro_count,
    ).await
}

#[tauri::command]
pub async fn get_executions_by_date(
    state: State<'_, AppState>,
    date: i64,
) -> Result<Vec<Execution>, crate::errors::AppError> {
    services::execution_service::get_executions_by_date(&state.db, date).await
}

#[tauri::command]
pub async fn get_executions_by_plan_id(
    state: State<'_, AppState>,
    plan_id: String,
) -> Result<Vec<Execution>, crate::errors::AppError> {
    services::execution_service::get_executions_by_plan_id(&state.db, &plan_id).await
}

#[tauri::command]
pub async fn update_execution(
    state: State<'_, AppState>,
    id: String,
    data: UpdateExecutionRequest,
) -> Result<Execution, crate::errors::AppError> {
    services::execution_service::update_execution(
        &state.db, &id, data.plan_id, data.start_time, data.end_time,
        data.actual_hours, data.completion_rate, data.notes, data.pomodoro_count,
    ).await
}

#[tauri::command]
pub async fn get_recent_executions(
    state: State<'_, AppState>,
    since: i64,
) -> Result<Vec<Execution>, crate::errors::AppError> {
    services::execution_service::get_recent_executions(&state.db, since).await
}

#[tauri::command]
pub async fn delete_execution(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), crate::errors::AppError> {
    services::execution_service::delete_execution(&state.db, &id).await
}
