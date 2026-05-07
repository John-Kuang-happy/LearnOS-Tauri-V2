use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::KnowledgePoint;
use crate::models::dto::{CreateKnowledgePointRequest, UpdateKnowledgePointRequest};
use crate::db::repositories::knowledge_point_repo::KnowledgePointRepository;

pub async fn create_kp(pool: &SqlitePool, req: CreateKnowledgePointRequest) -> Result<KnowledgePoint> {
    if req.name.trim().is_empty() {
        return Err(crate::errors::AppError::Internal("知识点名称不能为空".to_string()));
    }
    let kp = KnowledgePointRepository::create(pool, req).await?;
    // 自动生成首次复习记录（1天后）
    crate::services::ebbinghaus_service::generate_initial_review(pool, &kp).await?;
    Ok(kp)
}

pub async fn get_kps_by_subject(pool: &SqlitePool, subject_id: &str) -> Result<Vec<KnowledgePoint>> {
    KnowledgePointRepository::find_by_subject(pool, subject_id).await
}

pub async fn get_all_kps(pool: &SqlitePool) -> Result<Vec<KnowledgePoint>> {
    KnowledgePointRepository::find_all(pool).await
}

pub async fn get_kp(pool: &SqlitePool, id: &str) -> Result<KnowledgePoint> {
    KnowledgePointRepository::find_by_id(pool, id).await
}

pub async fn update_kp(pool: &SqlitePool, id: &str, req: UpdateKnowledgePointRequest) -> Result<KnowledgePoint> {
    KnowledgePointRepository::update(
        pool, id, req.name, req.description, req.difficulty_level,
        req.importance_level, req.is_mastered, req.source,
    ).await
}

pub async fn delete_kp(pool: &SqlitePool, id: &str) -> Result<()> {
    KnowledgePointRepository::delete(pool, id).await
}

pub async fn get_due_reviews(pool: &SqlitePool) -> Result<Vec<KnowledgePoint>> {
    KnowledgePointRepository::find_due_reviews(pool).await
}

pub async fn get_mastered_count(pool: &SqlitePool) -> Result<i64> {
    KnowledgePointRepository::count_mastered(pool).await
}

/// 批量创建知识点（每个自动生成首次复习记录）
pub async fn create_kps_batch(
    pool: &SqlitePool,
    names: Vec<String>,
    subject_id: &str,
    plan_id: Option<&str>,
) -> Result<Vec<KnowledgePoint>> {
    // 加载已有知识点名称，跳过重复
    let mut existing: std::collections::HashSet<String> = sqlx::query_scalar::<_, String>(
        "SELECT name FROM knowledge_points WHERE subject_id = ?"
    ).bind(subject_id).fetch_all(pool).await?.into_iter().collect();

    let mut created = Vec::new();
    for name in &names {
        let trimmed = name.trim();
        if trimmed.is_empty() { continue; }
        if existing.contains(trimmed) { continue; } // 跳过同名

        let req = CreateKnowledgePointRequest {
            subject_id: subject_id.to_string(),
            name: trimmed.to_string(),
            description: None,
            difficulty_level: Some(3),
            importance_level: Some(3),
            source: plan_id.map(|p| p.to_string()),
        };
        let kp = KnowledgePointRepository::create(pool, req).await?;
        // 自动生成首次复习记录
        crate::services::ebbinghaus_service::generate_initial_review(pool, &kp).await?;
        created.push(kp);
        existing.insert(trimmed.to_string());
    }

    // 关联第一个 KP 到计划（向后兼容）
    if let Some(pid) = plan_id {
        if let Some(first_kp) = created.first() {
            sqlx::query("UPDATE plans SET source_kp_id = ? WHERE id = ?")
                .bind(&first_kp.id).bind(pid)
                .execute(pool).await?;
        }
    }

    Ok(created)
}
