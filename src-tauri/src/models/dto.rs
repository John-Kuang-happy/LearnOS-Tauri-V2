use serde::{Deserialize, Serialize};

// ============================================================
// 科目 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubjectRequest {
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
    pub category: Option<String>,
    pub weekly_goal_hours: Option<f64>,
    pub weekly_goal_kps: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSubjectRequest {
    pub name: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub category: Option<String>,
    pub weekly_goal_hours: Option<f64>,
    pub weekly_goal_kps: Option<i32>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

// ============================================================
// 计划 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanRequest {
    pub title: String,
    pub subject_id: String,
    pub plan_type: Option<String>,
    pub source_plan_id: Option<String>,
    pub source_kp_id: Option<String>,
    pub priority: Option<i32>,
    pub status: Option<String>,
    pub start_date: i64,
    pub end_date: i64,
    pub estimated_hours: f64,
    pub time_slot: Option<String>,
    pub auto_review_enabled: Option<bool>,
    pub review_rule: Option<Vec<i64>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePlanRequest {
    pub title: Option<String>,
    pub subject_id: Option<String>,
    pub plan_type: Option<String>,
    pub source_plan_id: Option<String>,
    pub source_kp_id: Option<String>,
    pub priority: Option<i32>,
    pub status: Option<String>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub estimated_hours: Option<f64>,
    pub time_slot: Option<String>,
    pub auto_review_enabled: Option<bool>,
    pub review_rule: Option<Vec<i64>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanFilter {
    pub subject_id: Option<String>,
    pub status: Option<String>,
    pub priority: Option<i32>,
    pub plan_type: Option<String>,
}

// ============================================================
// 知识点 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKnowledgePointRequest {
    pub subject_id: String,
    pub name: String,
    pub description: Option<String>,
    pub difficulty_level: Option<i32>,
    pub importance_level: Option<i32>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateKnowledgePointRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub difficulty_level: Option<i32>,
    pub importance_level: Option<i32>,
    pub is_mastered: Option<bool>,
    pub source: Option<String>,
}

// ============================================================
// 复习 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitReviewFeedbackRequest {
    pub session_id: String,
    pub mastery_score: f64,
    pub time_spent_seconds: Option<i32>,
    pub feedback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReviewSessionsRequest {
    pub plan_id: String,
    pub custom_intervals: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSessionWithKp {
    pub session_id: String,
    pub knowledge_point_id: String,
    pub kp_name: String,
    pub kp_description: String,
    pub subject_id: String,
    pub subject_name: String,
    pub subject_color: String,
    pub scheduled_date: i64,
    pub mastery_score: Option<f64>,
    pub time_spent_seconds: i32,
    pub feedback: String,
    pub was_skipped: i32,
    pub difficulty_level: i32,
    pub importance_level: i32,
    pub review_interval_days: i32,
    pub mastery_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewStats {
    pub due_today: i64,
    pub due_this_week: i64,
    pub completed_this_week: i64,
    pub skipped_this_week: i64,
    pub avg_mastery_score: f64,
    pub total_kps: i64,
    pub mastered_kps: i64,
}

// ============================================================
// 考试 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExamRequest {
    pub name: String,
    pub exam_type: Option<String>,
    pub target_date: i64,
    pub remarks: Option<String>,
}

// ============================================================
// 执行 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartExecutionRequest {
    pub plan_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndExecutionRequest {
    pub actual_hours: f64,
    pub completion_rate: f64,
    pub notes: Option<String>,
    pub pomodoro_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateExecutionRequest {
    pub plan_id: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub actual_hours: Option<f64>,
    pub completion_rate: Option<f64>,
    pub notes: Option<String>,
    pub pomodoro_count: Option<i32>,
}

// ============================================================
// 复盘 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReviewRequest {
    pub plan_id: String,
    pub review_date: i64,
    pub what_went_well: Option<String>,
    pub what_to_improve: Option<String>,
    pub action_items: Option<String>,
    pub mood_score: Option<i32>,
    pub energy_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateReviewRequest {
    pub plan_id: Option<String>,
    pub review_date: Option<i64>,
    pub what_went_well: Option<String>,
    pub what_to_improve: Option<String>,
    pub action_items: Option<String>,
    pub mood_score: Option<i32>,
    pub energy_level: Option<i32>,
}

// ============================================================
// 记录 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecordRequest {
    pub execution_id: String,
    pub content_type: String,
    pub content_summary: String,
    pub difficulty_level: Option<i32>,
}

// ============================================================
// 设置 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub gaokao_date: Option<String>,
    pub theme: Option<String>,
    pub llm_enabled: Option<bool>,
    pub llm_api_key: Option<String>,
    pub llm_model: Option<String>,
    pub llm_endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSettingsRequest {
    pub gaokao_date: Option<String>,
    pub theme: Option<String>,
    pub llm_enabled: Option<bool>,
    pub llm_api_key: Option<String>,
    pub llm_model: Option<String>,
    pub llm_endpoint: Option<String>,
}

// ============================================================
// 仪表盘 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub weekly_hours: f64,
    pub completed_plans: i64,
    pub avg_completion_rate: f64,
    pub review_count: i64,
    pub due_reviews_count: i64,
    pub mastered_kp_count: i64,
    pub new_kps_this_week: i64,
    pub total_weekly_goal_kps: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectDistribution {
    pub subject_id: String,
    pub subject_name: String,
    pub color: String,
    pub total_hours: f64,
    pub execution_count: i64,
}

/// 每周目标达成进度（方案B：时间+知识点复合）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyGoalProgress {
    pub subject_id: String,
    pub subject_name: String,
    pub color: String,
    pub weekly_hours: f64,
    pub goal_hours: f64,
    pub new_kps: i64,
    pub goal_kps: i32,
    pub time_rate: f64,
    pub kp_rate: f64,
    pub composite_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyTrend {
    pub date: i64,
    pub hours: f64,
    pub execution_count: i64,
}

// ============================================================
// 分析 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeakSubjectAlert {
    pub subject_id: String,
    pub subject_name: String,
    pub color: String,
    pub score: f64,
    pub factors: Vec<WeakSubjectFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeakSubjectFactor {
    pub name: String,
    pub weight: f64,
    pub value: f64,
}

// ============================================================
// 知识点 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKpsBatchRequest {
    pub names: Vec<String>,
    pub subject_id: String,
    pub plan_id: Option<String>,
}

// ============================================================
// NLP / 聊天 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub reply: String,
    pub intent_type: String,
    pub actions: Vec<ActionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItem {
    pub action_type: String,
    pub description: String,
    pub data: serde_json::Value,
}

// ============================================================
// 进度页 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreakData {
    pub current_streak: i32,
    pub longest_streak: i32,
    pub weekly_days: Vec<bool>,
    pub total_study_days: i32,
}

/// 每日复习完成数（热力图用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReviewCount {
    pub date: i64,
    pub count: i32,
}

// ============================================================
// 自动化 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCheckin {
    pub date: String,
    pub greeting: String,
    pub due_review_count: i64,
    pub pending_plan_count: i64,
    pub today_completed: i64,
    pub today_minutes: i64,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionFeedback {
    pub mastery_score: Option<f64>,
    pub mastery_label: Option<String>,
    pub mood_score: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionCompleteResult {
    pub plan_updated: bool,
    pub review_created: bool,
    pub kp_updated: bool,
    pub message: String,
    pub next_review_date: Option<String>,
}

// ============================================================
// 首页推荐 DTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodayRecommendation {
    pub total_estimated_minutes: i64,
    pub completed_minutes: i64,
    pub items: Vec<RecommendedItem>,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedItem {
    pub id: String,
    pub item_type: String,
    pub title: String,
    pub subject_name: String,
    pub subject_color: String,
    pub mastery_level: f64,
    pub overdue_days: i32,
    pub estimated_minutes: i32,
    pub priority: i32,
    pub reason: String,
}
