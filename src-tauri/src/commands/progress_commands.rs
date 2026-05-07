use tauri::State;
use crate::state::AppState;
use crate::models::dto::StreakData;
use crate::models::entity::Insight;
use crate::services;

#[tauri::command]
pub async fn get_milestones(
    state: State<'_, AppState>,
    limit: i64,
) -> Result<Vec<Insight>, crate::errors::AppError> {
    services::milestone_service::get_milestones(&state.db, limit).await
}

#[tauri::command]
pub async fn get_streak_data(
    state: State<'_, AppState>,
) -> Result<StreakData, crate::errors::AppError> {
    services::milestone_service::get_streak_data(&state.db).await
}
