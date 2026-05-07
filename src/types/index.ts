// ============================================================
// LearnOS V2 TypeScript 类型定义
// ============================================================

// 科目
export interface Subject {
  id: string;
  name: string;
  color: string;
  icon: string;
  category: string;
  weekly_goal_hours: number;
  weekly_goal_kps: number;
  sort_order: number;
  is_active: number;
  created_at: number;
  updated_at: number;
}

// 学习计划（V2 增强版）
export interface Plan {
  id: string;
  title: string;
  subject_id: string;
  plan_type: string;
  source_plan_id: string | null;
  source_kp_id: string | null;
  priority: number;
  status: string;
  start_date: number;
  end_date: number;
  estimated_hours: number;
  time_slot: string;
  auto_review_enabled: number;
  review_rule: string | null;
  tags: string | null;
  created_at: number;
  updated_at: number;
}

// 知识点
export interface KnowledgePoint {
  id: string;
  subject_id: string;
  name: string;
  description: string;
  difficulty_level: number;
  importance_level: number;
  mastery_level: number;
  last_studied_at: number | null;
  last_reviewed_at: number | null;
  next_review_at: number | null;
  review_count: number;
  consecutive_correct: number;
  review_interval_days: number;
  ease_factor: number;
  review_history: string;
  is_mastered: number;
  mastered_at: number | null;
  source: string;
  created_at: number;
  updated_at: number;
}

// 复习记录
export interface ReviewSession {
  id: string;
  knowledge_point_id: string;
  plan_id: string | null;
  scheduled_date: number;
  actual_date: number | null;
  mastery_score: number | null;
  time_spent_seconds: number;
  feedback: string;
  was_skipped: number;
  created_at: number;
  updated_at: number;
}

// 考试
export interface Exam {
  id: string;
  name: string;
  exam_type: string;
  target_date: number;
  remarks: string;
  is_active: number;
  created_at: number;
  updated_at: number;
}

// 执行记录
export interface Execution {
  id: string;
  plan_id: string;
  start_time: number;
  end_time: number | null;
  actual_hours: number | null;
  completion_rate: number | null;
  notes: string | null;
  pomodoro_count: number;
  created_at: number;
  updated_at: number;
}

// 学习记录
export interface Record {
  id: string;
  execution_id: string;
  content_type: string;
  content_summary: string;
  difficulty_level: number;
  created_at: number;
  updated_at: number;
}

// 复盘
export interface Review {
  id: string;
  plan_id: string;
  review_date: number;
  what_went_well: string;
  what_to_improve: string;
  action_items: string;
  mood_score: number;
  energy_level: number;
  created_at: number;
  updated_at: number;
}

// 智能分析
export interface Insight {
  id: string;
  insight_type: string;
  title: string;
  content: string;
  related_subject_id: string | null;
  severity: string;
  is_read: number;
  generated_at: number;
  expires_at: number | null;
  metadata: string;
  created_at: number;
}

// ============================================================
// 请求类型
// ============================================================

export interface CreateSubjectRequest {
  name: string;
  color: string;
  icon?: string;
  category?: string;
  weekly_goal_hours?: number;
  weekly_goal_kps?: number;
}

export interface UpdateSubjectRequest {
  name?: string;
  color?: string;
  icon?: string;
  category?: string;
  weekly_goal_hours?: number;
  weekly_goal_kps?: number;
  sort_order?: number;
  is_active?: boolean;
}

export interface CreatePlanRequest {
  title: string;
  subject_id: string;
  plan_type?: string;
  source_plan_id?: string;
  source_kp_id?: string;
  priority?: number;
  status?: string;
  start_date: number;
  end_date: number;
  estimated_hours: number;
  time_slot?: string;
  auto_review_enabled?: boolean;
  review_rule?: number[] | null;
  tags?: string[];
}

export interface PlanFilter {
  subject_id?: string;
  status?: string;
  priority?: number;
  plan_type?: string;
}

export interface UpdatePlanRequest {
  title?: string;
  subject_id?: string;
  plan_type?: string;
  source_plan_id?: string;
  source_kp_id?: string;
  priority?: number;
  status?: string;
  start_date?: number;
  end_date?: number;
  estimated_hours?: number;
  time_slot?: string;
  auto_review_enabled?: boolean;
  review_rule?: number[] | null;
  tags?: string[];
}

// 执行
export interface StartExecutionRequest {
  plan_id: string;
}

export interface EndExecutionRequest {
  actual_hours: number;
  completion_rate: number;
  notes?: string;
  pomodoro_count?: number;
}

export interface UpdateExecutionRequest {
  plan_id?: string;
  start_time?: number;
  end_time?: number;
  actual_hours?: number;
  completion_rate?: number;
  notes?: string;
  pomodoro_count?: number;
}

// 复盘
export interface CreateReviewRequest {
  plan_id: string;
  review_date: number;
  what_went_well?: string;
  what_to_improve?: string;
  action_items?: string;
  mood_score?: number;
  energy_level?: number;
}

export interface UpdateReviewRequest {
  plan_id?: string;
  review_date?: number;
  what_went_well?: string;
  what_to_improve?: string;
  action_items?: string;
  mood_score?: number;
  energy_level?: number;
}

// 设置
export interface AppSettings {
  gaokao_date?: string;
  theme?: string;
  llm_enabled?: boolean;
  llm_api_key?: string;
  llm_model?: string;
  llm_endpoint?: string;
}

export interface UpdateSettingsRequest {
  gaokao_date?: string;
  theme?: string;
  llm_enabled?: boolean;
  llm_api_key?: string;
  llm_model?: string;
  llm_endpoint?: string;
}

// 仪表盘
export interface DashboardStats {
  weekly_hours: number;
  completed_plans: number;
  avg_completion_rate: number;
  review_count: number;
  due_reviews_count: number;
  mastered_kp_count: number;
  new_kps_this_week: number;
  total_weekly_goal_kps: number;
}

// 每周目标达成进度（方案B：时间+知识点复合）
export interface WeeklyGoalProgress {
  subject_id: string;
  subject_name: string;
  color: string;
  weekly_hours: number;
  goal_hours: number;
  new_kps: number;
  goal_kps: number;
  time_rate: number;
  kp_rate: number;
  composite_rate: number;
}

export interface SubjectDistribution {
  subject_id: string;
  subject_name: string;
  color: string;
  total_hours: number;
  execution_count: number;
}

export interface WeeklyTrend {
  date: number;
  hours: number;
  execution_count: number;
}

// 分析
export interface WeakSubjectAlert {
  subject_id: string;
  subject_name: string;
  color: string;
  score: number;
  factors: WeakSubjectFactor[];
}

export interface WeakSubjectFactor {
  name: string;
  weight: number;
  value: number;
}

// 艾宾浩斯复习
export interface ReviewSessionWithKp {
  session_id: string;
  knowledge_point_id: string;
  kp_name: string;
  kp_description: string;
  subject_id: string;
  subject_name: string;
  subject_color: string;
  scheduled_date: number;
  mastery_score: number | null;
  time_spent_seconds: number;
  feedback: string;
  was_skipped: number;
  difficulty_level: number;
  importance_level: number;
  review_interval_days: number;
  mastery_level: number;
}

export interface ReviewStats {
  due_today: number;
  due_this_week: number;
  completed_this_week: number;
  skipped_this_week: number;
  avg_mastery_score: number;
  total_kps: number;
  mastered_kps: number;
}

export interface SubmitReviewFeedbackRequest {
  session_id: string;
  mastery_score: number;
  time_spent_seconds?: number;
  feedback?: string;
}

// NLP 聊天
export interface ChatRequest {
  message: string;
}

export interface ChatResponse {
  reply: string;
  intent_type: string;
  actions: ActionItem[];
}

export interface ActionItem {
  action_type: string;
  description: string;
  data: { [key: string]: unknown };
}

// 自动化
export interface DailyCheckin {
  date: string;
  greeting: string;
  due_review_count: number;
  pending_plan_count: number;
  today_completed: number;
  today_minutes: number;
  suggestion: string;
}

export interface ExecutionFeedback {
  mastery_score?: number;
  mastery_label?: string;
  mood_score?: number;
  notes?: string;
}

export interface ExecutionCompleteResult {
  plan_updated: boolean;
  review_created: boolean;
  kp_updated: boolean;
  message: string;
  next_review_date?: string;
}

// ==================== 进度页类型 ====================

export interface StreakData {
  current_streak: number;
  longest_streak: number;
  weekly_days: boolean[];
  total_study_days: number;
}

export interface DailyReviewCount {
  date: number;
  count: number;
}

// ==================== 首页推荐类型 ====================

export interface TodayRecommendation {
  total_estimated_minutes: number;
  completed_minutes: number;
  items: RecommendedItem[];
  suggestion: string;
}

export interface RecommendedItem {
  id: string;
  item_type: 'review' | 'plan';
  title: string;
  subject_name: string;
  subject_color: string;
  mastery_level: number;
  overdue_days: number;
  estimated_minutes: number;
  priority: number;
  reason: string;
}
