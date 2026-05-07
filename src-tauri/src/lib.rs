mod commands;
mod db;
mod errors;
mod models;
mod services;
mod state;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use state::AppState;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    std::env::set_var("RUST_LOG", "info,learnos_lib=debug,sqlx=warn");
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None))
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;

            let db_path = app_data_dir.join("learnos_v2.db");
            log::info!("数据库路径: {:?}", db_path);

            let pool = tauri::async_runtime::block_on(async {
                let options = SqliteConnectOptions::new()
                    .filename(&db_path)
                    .foreign_keys(true)
                    .create_if_missing(true);
                SqlitePoolOptions::new().connect_with(options).await.expect("创建数据库连接池失败")
            });

            tauri::async_runtime::block_on(async {
                db::MIGRATOR.run(&pool).await.expect("数据库迁移失败");
            });

            let db_path_str = db_path.to_string_lossy().to_string();
            app.manage(AppState {
                db: Arc::new(pool),
                db_path: db_path_str,
            });

            log::info!("数据库初始化成功");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 科目命令
            commands::subject_commands::create_subject,
            commands::subject_commands::get_all_subjects,
            commands::subject_commands::get_subject,
            commands::subject_commands::update_subject,
            commands::subject_commands::delete_subject,
            // 计划命令
            commands::plan_commands::create_plan,
            commands::plan_commands::get_all_plans,
            commands::plan_commands::get_plan,
            commands::plan_commands::update_plan,
            commands::plan_commands::delete_plan,
            commands::plan_commands::get_settings,
            commands::plan_commands::update_settings,
            commands::plan_commands::delete_all_data,
            // 执行命令
            commands::execution_commands::start_execution,
            commands::execution_commands::end_execution,
            commands::execution_commands::get_executions_by_date,
            commands::execution_commands::get_executions_by_plan_id,
            commands::execution_commands::update_execution,
            commands::execution_commands::get_recent_executions,
            commands::execution_commands::delete_execution,
            // 复盘命令
            commands::review_commands::create_review,
            commands::review_commands::get_reviews_by_plan_id,
            commands::review_commands::get_all_reviews,
            commands::review_commands::update_review,
            commands::review_commands::delete_review,
            // 仪表盘命令
            commands::dashboard_commands::get_dashboard_stats,
            commands::dashboard_commands::get_subject_distribution,
            commands::dashboard_commands::get_weekly_trend,
            commands::dashboard_commands::get_recent_reviews,
            commands::dashboard_commands::get_weekly_goal_progress,
            // 考试命令
            commands::exam_commands::create_exam,
            commands::exam_commands::get_all_exams,
            commands::exam_commands::get_exam,
            commands::exam_commands::update_exam,
            commands::exam_commands::delete_exam,
            commands::exam_commands::get_upcoming_exams,
            // 知识点命令
            commands::knowledge_point_commands::create_knowledge_point,
            commands::knowledge_point_commands::get_all_knowledge_points,
            commands::knowledge_point_commands::get_knowledge_points_by_subject,
            commands::knowledge_point_commands::get_knowledge_point,
            commands::knowledge_point_commands::update_knowledge_point,
            commands::knowledge_point_commands::create_kps_batch,
            commands::knowledge_point_commands::delete_knowledge_point,
            // 艾宾浩斯复习命令
            commands::ebbinghaus_commands::submit_review_feedback,
            commands::ebbinghaus_commands::get_due_reviews,
            commands::ebbinghaus_commands::get_upcoming_reviews,
            commands::ebbinghaus_commands::get_review_sessions_by_kp,
            commands::ebbinghaus_commands::skip_review_session,
            commands::ebbinghaus_commands::get_review_stats,
            commands::ebbinghaus_commands::get_review_heatmap,
            // 智能分析命令
            commands::analysis_commands::get_weak_subject_alerts,
            commands::analysis_commands::get_time_distribution,
            commands::analysis_commands::get_efficiency_trend,
            commands::analysis_commands::run_full_analysis,
            commands::analysis_commands::get_recent_insights,
            commands::analysis_commands::mark_insight_read,
            commands::analysis_commands::mark_all_insights_read,
            // 智能建议命令
            commands::insight_commands::generate_suggestions,
            // NLP 命令
            commands::nlp_commands::parse_message,
            commands::nlp_commands::suggest_knowledge_points,
            // 自动化命令
            commands::auto_commands::daily_checkin,
            commands::auto_commands::on_execution_complete,
            commands::auto_commands::get_today_recommendations,
            // 进度页命令
            commands::progress_commands::get_milestones,
            commands::progress_commands::get_streak_data,
            // 设置页命令
            commands::settings_commands::backup_database,
            commands::settings_commands::restore_database,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
