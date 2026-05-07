use tauri::State;
use crate::state::AppState;
use crate::models::entity::Exam;
use crate::models::dto::*;
use crate::services;

#[tauri::command]
pub async fn create_exam(state: State<'_, AppState>, req: CreateExamRequest) -> Result<Exam, crate::errors::AppError> {
    services::exam_service::create_exam(&state.db, req).await
}

#[tauri::command]
pub async fn get_all_exams(state: State<'_, AppState>) -> Result<Vec<Exam>, crate::errors::AppError> {
    services::exam_service::get_all_exams(&state.db).await
}

#[tauri::command]
pub async fn get_exam(state: State<'_, AppState>, id: String) -> Result<Exam, crate::errors::AppError> {
    services::exam_service::get_exam(&state.db, &id).await
}

#[tauri::command]
pub async fn update_exam(
    state: State<'_, AppState>, id: String,
    name: Option<String>, exam_type: Option<String>,
    target_date: Option<i64>, remarks: Option<String>,
) -> Result<Exam, crate::errors::AppError> {
    services::exam_service::update_exam(&state.db, &id, name, exam_type, target_date, remarks).await
}

#[tauri::command]
pub async fn delete_exam(state: State<'_, AppState>, id: String) -> Result<(), crate::errors::AppError> {
    services::exam_service::delete_exam(&state.db, &id).await
}

#[tauri::command]
pub async fn get_upcoming_exams(state: State<'_, AppState>) -> Result<Vec<Exam>, crate::errors::AppError> {
    services::exam_service::get_upcoming_exams(&state.db).await
}
