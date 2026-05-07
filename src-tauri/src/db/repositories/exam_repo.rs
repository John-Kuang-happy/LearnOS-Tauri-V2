use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::Exam;
use crate::models::dto::CreateExamRequest;

pub struct ExamRepository;

impl ExamRepository {
    pub async fn create(pool: &SqlitePool, req: CreateExamRequest) -> Result<Exam> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let exam_type = req.exam_type.unwrap_or_else(|| "final".to_string());
        let remarks = req.remarks.unwrap_or_default();

        let exam = sqlx::query_as::<_, Exam>(
            r#"INSERT INTO exams (id, name, exam_type, target_date, remarks, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?)
               RETURNING *"#
        )
        .bind(&id).bind(&req.name).bind(&exam_type).bind(req.target_date)
        .bind(&remarks).bind(now).bind(now)
        .fetch_one(pool).await?;

        Ok(exam)
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Exam>> {
        let exams = sqlx::query_as::<_, Exam>(
            "SELECT * FROM exams WHERE is_active = 1 ORDER BY target_date ASC"
        ).fetch_all(pool).await?;
        Ok(exams)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Exam> {
        let exam = sqlx::query_as::<_, Exam>("SELECT * FROM exams WHERE id = ?")
            .bind(id).fetch_optional(pool).await?
            .ok_or_else(|| crate::errors::AppError::NotFound(format!("考试 {} 未找到", id)))?;
        Ok(exam)
    }

    pub async fn update(
        pool: &SqlitePool, id: &str,
        name: Option<String>, exam_type: Option<String>,
        target_date: Option<i64>, remarks: Option<String>,
    ) -> Result<Exam> {
        let now = chrono::Utc::now().timestamp();
        let exam = sqlx::query_as::<_, Exam>(
            r#"UPDATE exams SET name = COALESCE(?, name), exam_type = COALESCE(?, exam_type),
               target_date = COALESCE(?, target_date), remarks = COALESCE(?, remarks),
               updated_at = ? WHERE id = ? RETURNING *"#
        )
        .bind(name).bind(exam_type).bind(target_date).bind(remarks).bind(now).bind(id)
        .fetch_optional(pool).await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("考试 {} 未找到", id)))?;
        Ok(exam)
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query("UPDATE exams SET is_active = 0, updated_at = ? WHERE id = ?")
            .bind(now).bind(id).execute(pool).await?;
        if result.rows_affected() == 0 {
            return Err(crate::errors::AppError::NotFound(format!("考试 {} 未找到", id)));
        }
        Ok(())
    }

    /// 获取最近的考试
    pub async fn find_upcoming(pool: &SqlitePool, limit: i64) -> Result<Vec<Exam>> {
        let now = chrono::Utc::now().timestamp();
        let exams = sqlx::query_as::<_, Exam>(
            "SELECT * FROM exams WHERE is_active = 1 AND target_date >= ? ORDER BY target_date ASC LIMIT ?"
        ).bind(now).bind(limit).fetch_all(pool).await?;
        Ok(exams)
    }
}
