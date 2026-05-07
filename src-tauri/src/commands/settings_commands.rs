use tauri::{State, AppHandle, Manager};
use crate::state::AppState;

/// 备份数据库到指定路径
#[tauri::command]
pub async fn backup_database(
    state: State<'_, AppState>,
    dest_path: String,
) -> Result<String, String> {
    // 确保目标路径的父目录存在
    if let Some(parent) = std::path::Path::new(&dest_path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }
    std::fs::copy(&state.db_path, &dest_path)
        .map_err(|e| format!("备份失败: {}", e))?;
    Ok(format!("数据已备份到: {}", dest_path))
}

/// 从备份文件还原数据库
#[tauri::command]
pub async fn restore_database(
    state: State<'_, AppState>,
    app: AppHandle,
    backup_path: String,
) -> Result<(), String> {
    // 校验备份文件是否存在
    if !std::path::Path::new(&backup_path).exists() {
        return Err("备份文件不存在".into());
    }

    // 关闭所有数据库连接
    state.db.close().await;

    // 用备份文件替换当前数据库
    std::fs::copy(&backup_path, &state.db_path)
        .map_err(|e| format!("还原失败: {}", e))?;

    // 重启应用以重新建立数据库连接
    app.restart();
    // unreachable, but keep compiler happy
    Ok(())
}
