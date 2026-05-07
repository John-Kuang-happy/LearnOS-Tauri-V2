use tauri::State;
use crate::state::AppState;
use crate::models::entity::Insight;
use crate::services;

#[tauri::command]
pub async fn generate_suggestions(
    state: State<'_, AppState>,
) -> Result<Vec<Insight>, crate::errors::AppError> {
    services::insight_service::generate_suggestions(&state.db).await
}
