use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::Plan;
use crate::models::dto::{CreatePlanRequest, PlanFilter, UpdatePlanRequest};

pub struct PlanRepository;

impl PlanRepository {
    pub async fn create(pool: &SqlitePool, req: CreatePlanRequest) -> Result<Plan> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let plan_type = req.plan_type.unwrap_or_else(|| "normal".to_string());
        let priority = req.priority.unwrap_or(2);
        let status = req.status.unwrap_or_else(|| "pending".to_string());
        let time_slot = req.time_slot.unwrap_or_else(|| "any".to_string());
        let auto_review_enabled = if req.auto_review_enabled.unwrap_or(false) { 1 } else { 0 };
        let review_rule = req.review_rule.map(|r| serde_json::to_string(&r).unwrap());
        let tags = req.tags.map(|t| serde_json::to_string(&t).unwrap());

        let plan = sqlx::query_as::<_, Plan>(
            r#"INSERT INTO plans (id, title, subject_id, plan_type, source_plan_id, source_kp_id,
               priority, status, start_date, end_date, estimated_hours, time_slot,
               auto_review_enabled, review_rule, tags, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING *"#
        )
        .bind(&id)
        .bind(&req.title)
        .bind(&req.subject_id)
        .bind(&plan_type)
        .bind(&req.source_plan_id)
        .bind(&req.source_kp_id)
        .bind(priority)
        .bind(&status)
        .bind(req.start_date)
        .bind(req.end_date)
        .bind(req.estimated_hours)
        .bind(&time_slot)
        .bind(auto_review_enabled)
        .bind(&review_rule)
        .bind(&tags)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await?;

        Ok(plan)
    }

    pub async fn find_all(pool: &SqlitePool, filter: PlanFilter) -> Result<Vec<Plan>> {
        let mut builder = sqlx::QueryBuilder::new("SELECT * FROM plans WHERE 1=1");

        if let Some(ref subject_id) = filter.subject_id {
            builder.push(" AND subject_id = ");
            builder.push_bind(subject_id);
        }
        if let Some(ref status) = filter.status {
            builder.push(" AND status = ");
            builder.push_bind(status);
        }
        if let Some(priority) = filter.priority {
            builder.push(" AND priority = ");
            builder.push_bind(priority);
        }
        if let Some(ref plan_type) = filter.plan_type {
            builder.push(" AND plan_type = ");
            builder.push_bind(plan_type);
        }

        builder.push(" ORDER BY start_date DESC, priority ASC");

        let plans = builder.build_query_as::<Plan>().fetch_all(pool).await?;

        Ok(plans)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: &str) -> Result<Plan> {
        let plan = sqlx::query_as::<_, Plan>("SELECT * FROM plans WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| crate::errors::AppError::NotFound(format!("计划 {} 未找到", id)))?;

        Ok(plan)
    }

    pub async fn update(pool: &SqlitePool, id: &str, req: UpdatePlanRequest) -> Result<Plan> {
        let existing = Self::find_by_id(pool, id).await?;
        let now = chrono::Utc::now().timestamp();

        let plan = sqlx::query_as::<_, Plan>(
            r#"UPDATE plans SET
               title = COALESCE(?, title),
               subject_id = COALESCE(?, subject_id),
               plan_type = COALESCE(?, plan_type),
               source_plan_id = COALESCE(?, source_plan_id),
               source_kp_id = COALESCE(?, source_kp_id),
               priority = COALESCE(?, priority),
               status = COALESCE(?, status),
               start_date = COALESCE(?, start_date),
               end_date = COALESCE(?, end_date),
               estimated_hours = COALESCE(?, estimated_hours),
               time_slot = COALESCE(?, time_slot),
               auto_review_enabled = COALESCE(?, auto_review_enabled),
               review_rule = COALESCE(?, review_rule),
               tags = COALESCE(?, tags),
               updated_at = ?
               WHERE id = ?
               RETURNING *"#
        )
        .bind(req.title.as_ref().or(Some(&existing.title)))
        .bind(req.subject_id.as_ref().or(Some(&existing.subject_id)))
        .bind(req.plan_type.as_ref().or(Some(&existing.plan_type)))
        .bind(req.source_plan_id.as_ref().or(existing.source_plan_id.as_ref()))
        .bind(req.source_kp_id.as_ref().or(existing.source_kp_id.as_ref()))
        .bind(req.priority.or(Some(existing.priority)))
        .bind(req.status.as_ref().or(Some(&existing.status)))
        .bind(req.start_date.or(Some(existing.start_date)))
        .bind(req.end_date.or(Some(existing.end_date)))
        .bind(req.estimated_hours.or(Some(existing.estimated_hours)))
        .bind(req.time_slot.as_ref().or(Some(&existing.time_slot)))
        .bind(req.auto_review_enabled.map(|v| v as i32).or(Some(existing.auto_review_enabled)))
        .bind(req.review_rule.as_ref().map(|r| serde_json::to_string(r).unwrap()).or(existing.review_rule.clone()))
        .bind(req.tags.as_ref().map(|t| serde_json::to_string(t).unwrap()).or(existing.tags.clone()))
        .bind(now)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| crate::errors::AppError::NotFound(format!("计划 {} 未找到", id)))?;

        Ok(plan)
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM plans WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::errors::AppError::NotFound(format!("计划 {} 未找到", id)));
        }
        Ok(())
    }

    pub async fn count_by_status(pool: &SqlitePool, status: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM plans WHERE status = ?"
        )
        .bind(status)
        .fetch_one(pool)
        .await?;
        Ok(count)
    }
}
