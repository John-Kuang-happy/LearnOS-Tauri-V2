use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::Review;
use crate::models::dto::CreateReviewRequest;

pub struct ReviewRepository;

impl ReviewRepository {
    pub async fn create(pool: &SqlitePool, req: CreateReviewRequest) -> Result<Review> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let what_went_well = req.what_went_well.unwrap_or_default();
        let what_to_improve = req.what_to_improve.unwrap_or_default();
        let action_items = req.action_items.unwrap_or_default();
        let mood_score = req.mood_score.unwrap_or(3);
        let energy_level = req.energy_level.unwrap_or(3);

        let review = sqlx::query_as::<_, Review>(
            r#"INSERT INTO reviews (id, plan_id, review_date, what_went_well, what_to_improve,
               action_items, mood_score, energy_level, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING *"#
        )
        .bind(&id)
        .bind(&req.plan_id)
        .bind(req.review_date)
        .bind(&what_went_well)
        .bind(&what_to_improve)
        .bind(&action_items)
        .bind(mood_score)
        .bind(energy_level)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await?;

        Ok(review)
    }

    pub async fn find_by_plan_id(pool: &SqlitePool, plan_id: &str) -> Result<Vec<Review>> {
        let reviews = sqlx::query_as::<_, Review>(
            "SELECT * FROM reviews WHERE plan_id = ? ORDER BY review_date DESC"
        )
        .bind(plan_id)
        .fetch_all(pool)
        .await?;
        Ok(reviews)
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Review>> {
        let reviews = sqlx::query_as::<_, Review>(
            "SELECT * FROM reviews ORDER BY review_date DESC"
        )
        .fetch_all(pool)
        .await?;
        Ok(reviews)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Review> {
        let review = sqlx::query_as::<_, Review>(
            "SELECT * FROM reviews WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("复盘 {} 未找到", id)))?;

        Ok(review)
    }

    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        plan_id: Option<String>,
        review_date: Option<i64>,
        what_went_well: Option<String>,
        what_to_improve: Option<String>,
        action_items: Option<String>,
        mood_score: Option<i32>,
        energy_level: Option<i32>,
    ) -> Result<Review> {
        let now = chrono::Utc::now().timestamp();

        let review = sqlx::query_as::<_, Review>(
            r#"UPDATE reviews SET
               plan_id = COALESCE(?, plan_id),
               review_date = COALESCE(?, review_date),
               what_went_well = COALESCE(?, what_went_well),
               what_to_improve = COALESCE(?, what_to_improve),
               action_items = COALESCE(?, action_items),
               mood_score = COALESCE(?, mood_score),
               energy_level = COALESCE(?, energy_level),
               updated_at = ?
               WHERE id = ?
               RETURNING *"#
        )
        .bind(plan_id)
        .bind(review_date)
        .bind(what_went_well)
        .bind(what_to_improve)
        .bind(action_items)
        .bind(mood_score)
        .bind(energy_level)
        .bind(now)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("复盘 {} 未找到", id)))?;

        Ok(review)
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM reviews WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::errors::AppError::NotFound(format!("复盘 {} 未找到", id)));
        }
        Ok(())
    }

    pub async fn count_all(pool: &SqlitePool) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM reviews")
            .fetch_one(pool)
            .await?;
        Ok(count)
    }

    pub async fn find_recent(pool: &SqlitePool, limit: i64) -> Result<Vec<Review>> {
        let reviews = sqlx::query_as::<_, Review>(
            "SELECT * FROM reviews ORDER BY review_date DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;
        Ok(reviews)
    }
}
