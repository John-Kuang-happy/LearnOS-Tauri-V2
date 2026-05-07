use tauri::State;
use crate::state::AppState;
use crate::models::entity::Plan;
use crate::models::dto::*;
use crate::services;

#[tauri::command]
pub async fn create_plan(
    state: State<'_, AppState>,
    req: CreatePlanRequest,
) -> Result<Plan, crate::errors::AppError> {
    services::plan_service::create_plan(&state.db, req).await
}

#[tauri::command]
pub async fn get_all_plans(
    state: State<'_, AppState>,
    filter: PlanFilter,
) -> Result<Vec<Plan>, crate::errors::AppError> {
    services::plan_service::get_all_plans(&state.db, filter).await
}

#[tauri::command]
pub async fn get_plan(
    state: State<'_, AppState>,
    id: String,
) -> Result<Plan, crate::errors::AppError> {
    services::plan_service::get_plan(&state.db, &id).await
}

#[tauri::command]
pub async fn update_plan(
    state: State<'_, AppState>,
    id: String,
    req: UpdatePlanRequest,
) -> Result<Plan, crate::errors::AppError> {
    services::plan_service::update_plan(&state.db, &id, req).await
}

#[tauri::command]
pub async fn delete_plan(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), crate::errors::AppError> {
    services::plan_service::delete_plan(&state.db, &id).await
}

#[tauri::command]
pub async fn get_settings(
    state: State<'_, AppState>,
) -> Result<AppSettings, crate::errors::AppError> {
    services::plan_service::get_settings(&state.db).await
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    req: UpdateSettingsRequest,
) -> Result<AppSettings, crate::errors::AppError> {
    services::plan_service::update_settings(&state.db, req).await
}

#[tauri::command]
pub async fn delete_all_data(
    state: State<'_, AppState>,
) -> Result<(), crate::errors::AppError> {
    services::plan_service::delete_all_data(&state.db).await
}
