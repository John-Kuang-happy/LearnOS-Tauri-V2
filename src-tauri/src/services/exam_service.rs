use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::Exam;
use crate::models::dto::CreateExamRequest;
use crate::db::repositories::exam_repo::ExamRepository;

pub async fn create_exam(pool: &SqlitePool, req: CreateExamRequest) -> Result<Exam> {
    ExamRepository::create(pool, req).await
}

pub async fn get_all_exams(pool: &SqlitePool) -> Result<Vec<Exam>> {
    ExamRepository::find_all(pool).await
}

pub async fn get_exam(pool: &SqlitePool, id: &str) -> Result<Exam> {
    ExamRepository::find_by_id(pool, id).await
}

pub async fn update_exam(
    pool: &SqlitePool, id: &str,
    name: Option<String>, exam_type: Option<String>,
    target_date: Option<i64>, remarks: Option<String>,
) -> Result<Exam> {
    ExamRepository::update(pool, id, name, exam_type, target_date, remarks).await
}

pub async fn delete_exam(pool: &SqlitePool, id: &str) -> Result<()> {
    ExamRepository::delete(pool, id).await
}

pub async fn get_upcoming_exams(pool: &SqlitePool) -> Result<Vec<Exam>> {
    ExamRepository::find_upcoming(pool, 5).await
}
