use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::Plan;
use crate::models::dto::*;
use crate::db::repositories::{plan_repo::PlanRepository, settings_repo::SettingsRepository};

pub async fn create_plan(pool: &SqlitePool, req: CreatePlanRequest) -> Result<Plan> {
    PlanRepository::create(pool, req).await
}

pub async fn get_all_plans(pool: &SqlitePool, filter: PlanFilter) -> Result<Vec<Plan>> {
    PlanRepository::find_all(pool, filter).await
}

pub async fn get_plan(pool: &SqlitePool, id: &str) -> Result<Plan> {
    PlanRepository::find_by_id(pool, id).await
}

pub async fn update_plan(pool: &SqlitePool, id: &str, req: UpdatePlanRequest) -> Result<Plan> {
    PlanRepository::update(pool, id, req).await
}

pub async fn delete_plan(pool: &SqlitePool, id: &str) -> Result<()> {
    PlanRepository::delete(pool, id).await
}

/// 获取应用设置（V2 扩展版，包含所有设置项）
pub async fn get_settings(pool: &SqlitePool) -> Result<AppSettings> {
    let all_settings = SettingsRepository::get_all(pool).await?;

    let mut settings = AppSettings {
        gaokao_date: None,
        theme: Some("system".to_string()),
        llm_enabled: Some(false),
        llm_api_key: None,
        llm_model: Some("claude-sonnet-4-6".to_string()),
        llm_endpoint: None,
    };

    for (key, value) in all_settings {
        match key.as_str() {
            "gaokao_date" => settings.gaokao_date = Some(value),
            "theme" => settings.theme = Some(value),
            "llm_enabled" => settings.llm_enabled = Some(value == "true"),
            "llm_api_key" => settings.llm_api_key = Some(value),
            "llm_model" => settings.llm_model = Some(value),
            "llm_endpoint" => settings.llm_endpoint = Some(value),
            _ => {}
        }
    }

    Ok(settings)
}

/// 更新应用设置
pub async fn update_settings(pool: &SqlitePool, req: UpdateSettingsRequest) -> Result<AppSettings> {
    if let Some(ref v) = req.gaokao_date {
        SettingsRepository::set_value(pool, "gaokao_date", v).await?;
    }
    if let Some(ref v) = req.theme {
        SettingsRepository::set_value(pool, "theme", v).await?;
    }
    if let Some(v) = req.llm_enabled {
        SettingsRepository::set_value(pool, "llm_enabled", if v { "true" } else { "false" }).await?;
    }
    if let Some(ref v) = req.llm_api_key {
        SettingsRepository::set_value(pool, "llm_api_key", v).await?;
    }
    if let Some(ref v) = req.llm_model {
        SettingsRepository::set_value(pool, "llm_model", v).await?;
    }
    if let Some(ref v) = req.llm_endpoint {
        SettingsRepository::set_value(pool, "llm_endpoint", v).await?;
    }

    get_settings(pool).await
}

/// 清空所有数据
pub async fn delete_all_data(pool: &SqlitePool) -> Result<()> {
    SettingsRepository::delete_all(pool).await
}
