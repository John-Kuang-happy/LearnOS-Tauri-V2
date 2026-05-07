pub mod repositories;

use sqlx::{SqlitePool, migrate::Migrator, sqlite::{SqliteConnectOptions, SqlitePoolOptions}};
use crate::errors::{Result, AppError};

pub static MIGRATOR: Migrator = sqlx::migrate!();

/// 初始化数据库连接池（启用外键约束）
pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    // 去掉 sqlite: 前缀，提取文件路径
    let path = database_url.strip_prefix("sqlite:").unwrap_or(database_url);
    let options = SqliteConnectOptions::new()
        .filename(path)
        .foreign_keys(true)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;
    Ok(pool)
}

/// 运行数据库迁移
pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    MIGRATOR.run(pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(())
}
