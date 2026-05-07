use tauri::State;
use crate::state::AppState;
use crate::models::entity::KnowledgePoint;
use crate::models::dto::*;
use crate::services;

#[tauri::command]
pub async fn create_knowledge_point(
    state: State<'_, AppState>, req: CreateKnowledgePointRequest,
) -> Result<KnowledgePoint, crate::errors::AppError> {
    services::knowledge_point_service::create_kp(&state.db, req).await
}

#[tauri::command]
pub async fn get_all_knowledge_points(
    state: State<'_, AppState>,
) -> Result<Vec<KnowledgePoint>, crate::errors::AppError> {
    services::knowledge_point_service::get_all_kps(&state.db).await
}

#[tauri::command]
pub async fn get_knowledge_points_by_subject(
    state: State<'_, AppState>, subject_id: String,
) -> Result<Vec<KnowledgePoint>, crate::errors::AppError> {
    services::knowledge_point_service::get_kps_by_subject(&state.db, &subject_id).await
}

#[tauri::command]
pub async fn get_knowledge_point(
    state: State<'_, AppState>, id: String,
) -> Result<KnowledgePoint, crate::errors::AppError> {
    services::knowledge_point_service::get_kp(&state.db, &id).await
}

#[tauri::command]
pub async fn update_knowledge_point(
    state: State<'_, AppState>, id: String, req: UpdateKnowledgePointRequest,
) -> Result<KnowledgePoint, crate::errors::AppError> {
    services::knowledge_point_service::update_kp(&state.db, &id, req).await
}

#[tauri::command]
pub async fn create_kps_batch(
    state: State<'_, AppState>, req: CreateKpsBatchRequest,
) -> Result<Vec<KnowledgePoint>, crate::errors::AppError> {
    services::knowledge_point_service::create_kps_batch(
        &state.db, req.names, &req.subject_id, req.plan_id.as_deref(),
    ).await
}

#[tauri::command]
pub async fn delete_knowledge_point(
    state: State<'_, AppState>, id: String,
) -> Result<(), crate::errors::AppError> {
    services::knowledge_point_service::delete_kp(&state.db, &id).await
}
