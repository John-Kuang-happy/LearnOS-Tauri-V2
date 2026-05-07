use tauri::State;
use crate::state::AppState;
use crate::models::entity::Review;
use crate::models::dto::*;
use crate::services;

#[tauri::command]
pub async fn create_review(
    state: State<'_, AppState>,
    req: CreateReviewRequest,
) -> Result<Review, crate::errors::AppError> {
    services::review_service::create_review(&state.db, req).await
}

#[tauri::command]
pub async fn get_reviews_by_plan_id(
    state: State<'_, AppState>,
    plan_id: String,
) -> Result<Vec<Review>, crate::errors::AppError> {
    services::review_service::get_reviews_by_plan_id(&state.db, &plan_id).await
}

#[tauri::command]
pub async fn get_all_reviews(
    state: State<'_, AppState>,
) -> Result<Vec<Review>, crate::errors::AppError> {
    services::review_service::get_all_reviews(&state.db).await
}

#[tauri::command]
pub async fn update_review(
    state: State<'_, AppState>,
    id: String,
    req: UpdateReviewRequest,
) -> Result<Review, crate::errors::AppError> {
    services::review_service::update_review(&state.db, &id, req).await
}

#[tauri::command]
pub async fn delete_review(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), crate::errors::AppError> {
    services::review_service::delete_review(&state.db, &id).await
}
