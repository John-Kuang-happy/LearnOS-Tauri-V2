use tauri::State;
use crate::state::AppState;
use crate::models::entity::{ReviewSession, KnowledgePoint};
use crate::models::dto::*;
use crate::services;

#[tauri::command]
pub async fn submit_review_feedback(
    state: State<'_, AppState>, req: SubmitReviewFeedbackRequest,
) -> Result<(ReviewSession, KnowledgePoint), crate::errors::AppError> {
    services::ebbinghaus_service::submit_review_feedback(&state.db, req).await
}

#[tauri::command]
pub async fn get_due_reviews(
    state: State<'_, AppState>,
) -> Result<Vec<ReviewSessionWithKp>, crate::errors::AppError> {
    services::ebbinghaus_service::get_due_reviews(&state.db).await
}

#[tauri::command]
pub async fn get_upcoming_reviews(
    state: State<'_, AppState>, days: i64,
) -> Result<Vec<ReviewSessionWithKp>, crate::errors::AppError> {
    services::ebbinghaus_service::get_upcoming_reviews(&state.db, days).await
}

#[tauri::command]
pub async fn get_review_sessions_by_kp(
    state: State<'_, AppState>, kp_id: String,
) -> Result<Vec<ReviewSession>, crate::errors::AppError> {
    services::ebbinghaus_service::get_kp_review_history(&state.db, &kp_id).await
}

#[tauri::command]
pub async fn skip_review_session(
    state: State<'_, AppState>, session_id: String,
) -> Result<ReviewSession, crate::errors::AppError> {
    services::ebbinghaus_service::skip_review_session(&state.db, &session_id).await
}

#[tauri::command]
pub async fn get_review_stats(
    state: State<'_, AppState>,
) -> Result<ReviewStats, crate::errors::AppError> {
    services::ebbinghaus_service::get_review_stats(&state.db).await
}

#[tauri::command]
pub async fn get_review_heatmap(
    state: State<'_, AppState>,
    days: i64,
) -> Result<Vec<DailyReviewCount>, crate::errors::AppError> {
    crate::db::repositories::review_session_repo::ReviewSessionRepository::get_daily_counts(&state.db, days).await
}
