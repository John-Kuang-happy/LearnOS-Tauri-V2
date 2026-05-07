use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 科目
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Subject {
    pub id: String,
    pub name: String,
    pub color: String,
    pub icon: String,
    pub category: String,
    pub weekly_goal_hours: f64,
    pub weekly_goal_kps: i32,
    pub sort_order: i32,
    pub is_active: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 学习计划（V2 增强版）
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub title: String,
    pub subject_id: String,
    pub plan_type: String,
    pub source_plan_id: Option<String>,
    pub source_kp_id: Option<String>,
    pub priority: i32,
    pub status: String,
    pub start_date: i64,
    pub end_date: i64,
    pub estimated_hours: f64,
    pub time_slot: String,
    pub auto_review_enabled: i32,
    pub review_rule: Option<String>,
    pub tags: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 知识点（艾宾浩斯复习核心）
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct KnowledgePoint {
    pub id: String,
    pub subject_id: String,
    pub name: String,
    pub description: String,
    pub difficulty_level: i32,
    pub importance_level: i32,
    pub mastery_level: f64,
    pub last_studied_at: Option<i64>,
    pub last_reviewed_at: Option<i64>,
    pub next_review_at: Option<i64>,
    pub review_count: i32,
    pub consecutive_correct: i32,
    pub review_interval_days: i32,
    pub ease_factor: f64,
    pub review_history: String,
    pub is_mastered: i32,
    pub mastered_at: Option<i64>,
    pub source: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 复习记录
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ReviewSession {
    pub id: String,
    pub knowledge_point_id: String,
    pub plan_id: Option<String>,
    pub scheduled_date: i64,
    pub actual_date: Option<i64>,
    pub mastery_score: Option<f64>,
    pub time_spent_seconds: i32,
    pub feedback: String,
    pub was_skipped: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 考试
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Exam {
    pub id: String,
    pub name: String,
    pub exam_type: String,
    pub target_date: i64,
    pub remarks: String,
    pub is_active: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 执行记录
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Execution {
    pub id: String,
    pub plan_id: String,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub actual_hours: Option<f64>,
    pub completion_rate: Option<f64>,
    pub notes: Option<String>,
    pub pomodoro_count: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 学习记录
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub execution_id: String,
    pub content_type: String,
    pub content_summary: String,
    pub difficulty_level: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 复盘
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Review {
    pub id: String,
    pub plan_id: String,
    pub review_date: i64,
    pub what_went_well: String,
    pub what_to_improve: String,
    pub action_items: String,
    pub mood_score: i32,
    pub energy_level: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 智能分析结果
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Insight {
    pub id: String,
    pub insight_type: String,
    pub title: String,
    pub content: String,
    pub related_subject_id: Option<String>,
    pub severity: String,
    pub is_read: i32,
    pub generated_at: i64,
    pub expires_at: Option<i64>,
    pub metadata: String,
    pub created_at: i64,
}
