use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::Review;
use crate::models::dto::*;
use crate::db::repositories::review_repo::ReviewRepository;

pub async fn create_review(pool: &SqlitePool, req: CreateReviewRequest) -> Result<Review> {
    if req.plan_id.is_empty() {
        return Err(crate::errors::AppError::Internal("计划ID不能为空".to_string()));
    }
    ReviewRepository::create(pool, req).await
}

pub async fn get_reviews_by_plan_id(pool: &SqlitePool, plan_id: &str) -> Result<Vec<Review>> {
    ReviewRepository::find_by_plan_id(pool, plan_id).await
}

pub async fn get_all_reviews(pool: &SqlitePool) -> Result<Vec<Review>> {
    ReviewRepository::find_all(pool).await
}

pub async fn update_review(pool: &SqlitePool, id: &str, req: UpdateReviewRequest) -> Result<Review> {
    ReviewRepository::update(
        pool, id, req.plan_id, req.review_date, req.what_went_well,
        req.what_to_improve, req.action_items, req.mood_score, req.energy_level,
    ).await
}

pub async fn delete_review(pool: &SqlitePool, id: &str) -> Result<()> {
    ReviewRepository::delete(pool, id).await
}
