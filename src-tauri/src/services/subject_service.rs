use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::Subject;
use crate::models::dto::{CreateSubjectRequest, UpdateSubjectRequest};
use crate::db::repositories::subject_repo::SubjectRepository;

pub async fn create_subject(pool: &SqlitePool, req: CreateSubjectRequest) -> Result<Subject> {
    if req.name.trim().is_empty() {
        return Err(crate::errors::AppError::Internal("科目名称不能为空".to_string()));
    }
    SubjectRepository::create(pool, req).await
}

pub async fn get_all_subjects(pool: &SqlitePool) -> Result<Vec<Subject>> {
    SubjectRepository::find_all(pool).await
}

pub async fn get_subject(pool: &SqlitePool, id: &str) -> Result<Subject> {
    SubjectRepository::find_by_id(pool, id).await
}

pub async fn update_subject(pool: &SqlitePool, id: &str, req: UpdateSubjectRequest) -> Result<Subject> {
    SubjectRepository::update(
        pool, id, req.name, req.color, req.icon, req.category,
        req.weekly_goal_hours, req.weekly_goal_kps,
        req.sort_order, req.is_active,
    ).await
}

pub async fn delete_subject(pool: &SqlitePool, id: &str) -> Result<()> {
    SubjectRepository::delete(pool, id).await
}
