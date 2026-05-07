use tauri::State;
use crate::state::AppState;
use crate::models::entity::Subject;
use crate::models::dto::{CreateSubjectRequest, UpdateSubjectRequest};
use crate::services;

#[tauri::command]
pub async fn create_subject(
    state: State<'_, AppState>,
    req: CreateSubjectRequest,
) -> Result<Subject, crate::errors::AppError> {
    services::subject_service::create_subject(&state.db, req).await
}

#[tauri::command]
pub async fn get_all_subjects(
    state: State<'_, AppState>,
) -> Result<Vec<Subject>, crate::errors::AppError> {
    services::subject_service::get_all_subjects(&state.db).await
}

#[tauri::command]
pub async fn get_subject(
    state: State<'_, AppState>,
    id: String,
) -> Result<Subject, crate::errors::AppError> {
    services::subject_service::get_subject(&state.db, &id).await
}

#[tauri::command]
pub async fn update_subject(
    state: State<'_, AppState>,
    id: String,
    req: UpdateSubjectRequest,
) -> Result<Subject, crate::errors::AppError> {
    services::subject_service::update_subject(&state.db, &id, req).await
}

#[tauri::command]
pub async fn delete_subject(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), crate::errors::AppError> {
    services::subject_service::delete_subject(&state.db, &id).await
}
