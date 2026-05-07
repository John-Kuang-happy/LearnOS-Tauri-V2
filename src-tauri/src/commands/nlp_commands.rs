use tauri::State;
use crate::state::AppState;
use crate::models::dto::*;
use crate::services;

#[tauri::command]
pub async fn parse_message(
    state: State<'_, AppState>, req: ChatRequest,
) -> Result<ChatResponse, crate::errors::AppError> {
    let (_, response) = services::nlp_service::parse_intent(&state.db, &req.message).await?;
    Ok(response)
}

#[tauri::command]
pub async fn suggest_knowledge_points(
    state: State<'_, AppState>,
    title: String,
    subject_id: String,
) -> Result<Vec<String>, crate::errors::AppError> {
    services::nlp_service::suggest_kps(&state.db, &title, &subject_id).await
}
