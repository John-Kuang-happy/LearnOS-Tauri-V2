use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::Subject;
use crate::models::dto::CreateSubjectRequest;

pub struct SubjectRepository;

impl SubjectRepository {
    pub async fn create(pool: &SqlitePool, req: CreateSubjectRequest) -> Result<Subject> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let icon = req.icon.unwrap_or_else(|| "📖".to_string());
        let category = req.category.unwrap_or_else(|| "other".to_string());
        let weekly_goal_hours = req.weekly_goal_hours.unwrap_or(10.0);
        let weekly_goal_kps = req.weekly_goal_kps.unwrap_or(3);

        let subject = sqlx::query_as::<_, Subject>(
            r#"INSERT INTO subjects (id, name, color, icon, category, weekly_goal_hours, weekly_goal_kps, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING *"#
        )
        .bind(&id)
        .bind(&req.name)
        .bind(&req.color)
        .bind(&icon)
        .bind(&category)
        .bind(weekly_goal_hours)
        .bind(weekly_goal_kps)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await?;

        Ok(subject)
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Subject>> {
        let subjects = sqlx::query_as::<_, Subject>(
            "SELECT * FROM subjects WHERE is_active = 1 ORDER BY sort_order, created_at"
        )
        .fetch_all(pool)
        .await?;

        Ok(subjects)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Subject> {
        let subject = sqlx::query_as::<_, Subject>(
            "SELECT * FROM subjects WHERE id = ? AND is_active = 1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("科目 {} 未找到", id)))?;

        Ok(subject)
    }

    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        name: Option<String>,
        color: Option<String>,
        icon: Option<String>,
        category: Option<String>,
        weekly_goal_hours: Option<f64>,
        weekly_goal_kps: Option<i32>,
        sort_order: Option<i32>,
        is_active: Option<bool>,
    ) -> Result<Subject> {
        let now = chrono::Utc::now().timestamp();
        let subject = sqlx::query_as::<_, Subject>(
            r#"UPDATE subjects SET
               name = COALESCE(?, name),
               color = COALESCE(?, color),
               icon = COALESCE(?, icon),
               category = COALESCE(?, category),
               weekly_goal_hours = COALESCE(?, weekly_goal_hours),
               weekly_goal_kps = COALESCE(?, weekly_goal_kps),
               sort_order = COALESCE(?, sort_order),
               is_active = COALESCE(?, is_active),
               updated_at = ?
               WHERE id = ?
               RETURNING *"#
        )
        .bind(name)
        .bind(color)
        .bind(icon)
        .bind(category)
        .bind(weekly_goal_hours)
        .bind(weekly_goal_kps)
        .bind(sort_order)
        .bind(is_active.map(|v| v as i32))
        .bind(now)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("科目 {} 未找到", id)))?;

        Ok(subject)
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            "UPDATE subjects SET is_active = 0, updated_at = ? WHERE id = ?"
        )
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(crate::errors::AppError::NotFound(format!("科目 {} 未找到", id)));
        }
        Ok(())
    }
}
