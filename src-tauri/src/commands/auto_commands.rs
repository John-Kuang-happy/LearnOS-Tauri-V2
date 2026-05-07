use tauri::State;
use crate::state::AppState;
use crate::models::dto::*;
use crate::services;

#[tauri::command]
pub async fn daily_checkin(
    state: State<'_, AppState>,
) -> Result<DailyCheckin, crate::errors::AppError> {
    services::auto_service::daily_checkin(&state.db).await
}

#[tauri::command]
pub async fn on_execution_complete(
    state: State<'_, AppState>,
    execution_id: String,
    feedback: Option<ExecutionFeedback>,
) -> Result<ExecutionCompleteResult, crate::errors::AppError> {
    services::auto_service::on_execution_complete(&state.db, &execution_id, feedback).await
}

#[tauri::command]
pub async fn get_today_recommendations(
    state: State<'_, AppState>,
) -> Result<TodayRecommendation, crate::errors::AppError> {
    services::auto_service::get_today_recommendations(&state.db).await
}
