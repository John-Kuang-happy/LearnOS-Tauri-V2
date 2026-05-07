use sqlx::SqlitePool;
use std::sync::Arc;

/// 应用状态，包含数据库连接池
pub struct AppState {
    pub db: Arc<SqlitePool>,
    pub db_path: String,
}
