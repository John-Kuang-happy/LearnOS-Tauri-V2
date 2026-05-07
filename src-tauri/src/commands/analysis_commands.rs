use tauri::State;
use crate::state::AppState;
use crate::models::dto::*;
use crate::models::entity::Insight;
use crate::services;

#[tauri::command]
pub async fn get_weak_subject_alerts(
    state: State<'_, AppState>,
) -> Result<Vec<WeakSubjectAlert>, crate::errors::AppError> {
    services::analysis_service::analyze_weak_subjects(&state.db).await
}

#[tauri::command]
pub async fn get_time_distribution(
    state: State<'_, AppState>, days: i64,
) -> Result<Vec<SubjectDistribution>, crate::errors::AppError> {
    services::analysis_service::get_time_distribution(&state.db, days).await
}

#[tauri::command]
pub async fn get_efficiency_trend(
    state: State<'_, AppState>, weeks: i64,
) -> Result<Vec<WeeklyTrend>, crate::errors::AppError> {
    services::analysis_service::get_efficiency_trend(&state.db, weeks).await
}

#[tauri::command]
pub async fn run_full_analysis(
    state: State<'_, AppState>,
) -> Result<Vec<WeakSubjectAlert>, crate::errors::AppError> {
    services::analysis_service::run_full_analysis(&state.db).await
}

#[tauri::command]
pub async fn get_recent_insights(
    state: State<'_, AppState>, limit: i64,
) -> Result<Vec<Insight>, crate::errors::AppError> {
    crate::db::repositories::insight_repo::InsightRepository::find_recent(&state.db, limit).await
}

#[tauri::command]
pub async fn mark_insight_read(
    state: State<'_, AppState>, id: String,
) -> Result<(), crate::errors::AppError> {
    crate::db::repositories::insight_repo::InsightRepository::mark_read(&state.db, &id).await
}

#[tauri::command]
pub async fn mark_all_insights_read(
    state: State<'_, AppState>,
) -> Result<(), crate::errors::AppError> {
    crate::db::repositories::insight_repo::InsightRepository::mark_all_read(&state.db).await
}
