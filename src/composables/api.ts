import { invoke } from '@tauri-apps/api/core';
import type {
  Subject, CreateSubjectRequest, UpdateSubjectRequest,
  Plan, CreatePlanRequest, UpdatePlanRequest, PlanFilter,
  Execution, StartExecutionRequest, EndExecutionRequest, UpdateExecutionRequest,
  Review, CreateReviewRequest, UpdateReviewRequest,
  AppSettings, UpdateSettingsRequest,
  DashboardStats, SubjectDistribution, WeeklyTrend, WeeklyGoalProgress,
  Exam, KnowledgePoint, ReviewSession,
  ReviewSessionWithKp, ReviewStats, SubmitReviewFeedbackRequest,
  WeakSubjectAlert, Insight,
  ChatRequest, ChatResponse, DailyCheckin, ExecutionFeedback, ExecutionCompleteResult,
  TodayRecommendation,
  StreakData, DailyReviewCount,
} from '../types';

// ==================== 科目 API ====================

export async function createSubject(req: CreateSubjectRequest): Promise<Subject> {
  return invoke<Subject>('create_subject', { req });
}

export async function getAllSubjects(): Promise<Subject[]> {
  return invoke<Subject[]>('get_all_subjects');
}

export async function getSubject(id: string): Promise<Subject> {
  return invoke<Subject>('get_subject', { id });
}

export async function updateSubject(id: string, req: UpdateSubjectRequest): Promise<Subject> {
  return invoke<Subject>('update_subject', { id, req });
}

export async function deleteSubject(id: string): Promise<void> {
  return invoke<void>('delete_subject', { id });
}

// ==================== 计划 API ====================

export async function createPlan(req: CreatePlanRequest): Promise<Plan> {
  return invoke<Plan>('create_plan', { req });
}

export async function getAllPlans(filter: PlanFilter = {}): Promise<Plan[]> {
  return invoke<Plan[]>('get_all_plans', { filter });
}

export async function getPlan(id: string): Promise<Plan> {
  return invoke<Plan>('get_plan', { id });
}

export async function updatePlan(id: string, req: UpdatePlanRequest): Promise<Plan> {
  return invoke<Plan>('update_plan', { id, req });
}

export async function deletePlan(id: string): Promise<void> {
  return invoke<void>('delete_plan', { id });
}

// ==================== 执行 API ====================

export async function startExecution(req: StartExecutionRequest): Promise<Execution> {
  return invoke<Execution>('start_execution', { req });
}

export async function endExecution(id: string, data: EndExecutionRequest): Promise<Execution> {
  return invoke<Execution>('end_execution', { id, data });
}

export async function getExecutionsByDate(date: number): Promise<Execution[]> {
  return invoke<Execution[]>('get_executions_by_date', { date });
}

export async function getRecentExecutions(since: number): Promise<Execution[]> {
  return invoke<Execution[]>('get_recent_executions', { since });
}

export async function getExecutionsByPlanId(planId: string): Promise<Execution[]> {
  return invoke<Execution[]>('get_executions_by_plan_id', { planId });
}

export async function updateExecution(id: string, data: UpdateExecutionRequest): Promise<Execution> {
  return invoke<Execution>('update_execution', { id, data });
}

export async function deleteExecution(id: string): Promise<void> {
  return invoke<void>('delete_execution', { id });
}

// ==================== 复盘 API ====================

export async function createReview(req: CreateReviewRequest): Promise<Review> {
  return invoke<Review>('create_review', { req });
}

export async function getReviewsByPlanId(planId: string): Promise<Review[]> {
  return invoke<Review[]>('get_reviews_by_plan_id', { planId });
}

export async function getAllReviews(): Promise<Review[]> {
  return invoke<Review[]>('get_all_reviews');
}

export async function updateReview(id: string, req: UpdateReviewRequest): Promise<Review> {
  return invoke<Review>('update_review', { id, req });
}

export async function deleteReview(id: string): Promise<void> {
  return invoke<void>('delete_review', { id });
}

// ==================== 设置 API ====================

export async function getSettings(): Promise<AppSettings> {
  return invoke<AppSettings>('get_settings');
}

export async function updateSettings(req: UpdateSettingsRequest): Promise<AppSettings> {
  return invoke<AppSettings>('update_settings', { req });
}

export async function deleteAllData(): Promise<void> {
  return invoke<void>('delete_all_data');
}

export async function backupDatabase(destPath: string): Promise<string> {
  return invoke<string>('backup_database', { destPath });
}

export async function restoreDatabase(backupPath: string): Promise<void> {
  return invoke<void>('restore_database', { backupPath });
}

// ==================== 仪表盘 API ====================

export async function getDashboardStats(): Promise<DashboardStats> {
  return invoke<DashboardStats>('get_dashboard_stats');
}

export async function getSubjectDistribution(
  startDate?: number, endDate?: number
): Promise<SubjectDistribution[]> {
  return invoke<SubjectDistribution[]>('get_subject_distribution', {
    startDate, endDate,
  });
}

export async function getWeeklyTrend(weeks?: number): Promise<WeeklyTrend[]> {
  return invoke<WeeklyTrend[]>('get_weekly_trend', { weeks });
}

export async function getRecentReviews(limit?: number): Promise<Array<{
  id: string;
  plan_title: string;
  subject_name: string;
  review_date: number;
  mood_score: number;
  energy_level: number;
  what_went_well: string;
}>> {
  return invoke('get_recent_reviews', { limit });
}

export async function getWeeklyGoalProgress(): Promise<WeeklyGoalProgress[]> {
  return invoke<WeeklyGoalProgress[]>('get_weekly_goal_progress');
}

// ==================== 考试 API ====================

export async function createExam(req: { name: string; exam_type: string; target_date: number; remarks?: string }): Promise<Exam> {
  return invoke<Exam>('create_exam', { req });
}

export async function getAllExams(): Promise<Exam[]> {
  return invoke<Exam[]>('get_all_exams');
}

export async function getExam(id: string): Promise<Exam> {
  return invoke<Exam>('get_exam', { id });
}

export async function updateExam(id: string, name?: string, exam_type?: string, target_date?: number, remarks?: string): Promise<Exam> {
  return invoke<Exam>('update_exam', { id, name, examType: exam_type, targetDate: target_date, remarks });
}

export async function deleteExam(id: string): Promise<void> {
  return invoke<void>('delete_exam', { id });
}

export async function getUpcomingExams(): Promise<Exam[]> {
  return invoke<Exam[]>('get_upcoming_exams');
}

// ==================== 知识点 API ====================

export async function createKnowledgePoint(req: { subject_id: string; name: string; description?: string; difficulty_level?: number; importance_level?: number; source?: string }): Promise<KnowledgePoint> {
  return invoke<KnowledgePoint>('create_knowledge_point', { req });
}

export async function getAllKnowledgePoints(): Promise<KnowledgePoint[]> {
  return invoke<KnowledgePoint[]>('get_all_knowledge_points');
}

export async function getKnowledgePointsBySubject(subjectId: string): Promise<KnowledgePoint[]> {
  return invoke<KnowledgePoint[]>('get_knowledge_points_by_subject', { subjectId });
}

export async function getKnowledgePoint(id: string): Promise<KnowledgePoint> {
  return invoke<KnowledgePoint>('get_knowledge_point', { id });
}

export async function updateKnowledgePoint(id: string, req: { name?: string; description?: string; difficulty_level?: number; importance_level?: number; is_mastered?: boolean; source?: string }): Promise<KnowledgePoint> {
  return invoke<KnowledgePoint>('update_knowledge_point', { id, req });
}

export async function createKpsBatch(req: { names: string[]; subject_id: string; plan_id?: string }): Promise<KnowledgePoint[]> {
  return invoke<KnowledgePoint[]>('create_kps_batch', { req });
}

export async function suggestKnowledgePoints(title: string, subjectId: string): Promise<string[]> {
  return invoke<string[]>('suggest_knowledge_points', { title, subjectId });
}

export async function deleteKnowledgePoint(id: string): Promise<void> {
  return invoke<void>('delete_knowledge_point', { id });
}

// ==================== 艾宾浩斯复习 API ====================

export async function submitReviewFeedback(req: SubmitReviewFeedbackRequest): Promise<[ReviewSession, KnowledgePoint]> {
  return invoke<[ReviewSession, KnowledgePoint]>('submit_review_feedback', { req });
}

export async function getDueReviews(): Promise<ReviewSessionWithKp[]> {
  return invoke<ReviewSessionWithKp[]>('get_due_reviews');
}

export async function getUpcomingReviews(days: number): Promise<ReviewSessionWithKp[]> {
  return invoke<ReviewSessionWithKp[]>('get_upcoming_reviews', { days });
}

export async function getReviewSessionsByKp(kpId: string): Promise<ReviewSession[]> {
  return invoke<ReviewSession[]>('get_review_sessions_by_kp', { kpId });
}

export async function skipReviewSession(sessionId: string): Promise<void> {
  return invoke<void>('skip_review_session', { sessionId });
}

export async function getReviewStats(): Promise<ReviewStats> {
  return invoke<ReviewStats>('get_review_stats');
}

export async function getReviewHeatmap(days: number): Promise<DailyReviewCount[]> {
  return invoke<DailyReviewCount[]>('get_review_heatmap', { days });
}

// ==================== 智能分析 API ====================

export async function getWeakSubjectAlerts(): Promise<WeakSubjectAlert[]> {
  return invoke<WeakSubjectAlert[]>('get_weak_subject_alerts');
}

export async function getTimeDistribution(days: number): Promise<SubjectDistribution[]> {
  return invoke<SubjectDistribution[]>('get_time_distribution', { days });
}

export async function getEfficiencyTrend(weeks: number): Promise<WeeklyTrend[]> {
  return invoke<WeeklyTrend[]>('get_efficiency_trend', { weeks });
}

export async function runFullAnalysis(): Promise<WeakSubjectAlert[]> {
  return invoke<WeakSubjectAlert[]>('run_full_analysis');
}

export async function getRecentInsights(limit: number): Promise<Insight[]> {
  return invoke<Insight[]>('get_recent_insights', { limit });
}

export async function markInsightRead(id: string): Promise<void> {
  return invoke<void>('mark_insight_read', { id });
}

export async function markAllInsightsRead(): Promise<void> {
  return invoke<void>('mark_all_insights_read');
}

export async function generateSuggestions(): Promise<Insight[]> {
  return invoke<Insight[]>('generate_suggestions');
}

// ==================== NLP / 聊天 API ====================

export async function parseMessage(req: ChatRequest): Promise<ChatResponse> {
  return invoke<ChatResponse>('parse_message', { req });
}

// ==================== 进度页 API ====================

export async function getMilestones(limit: number = 20): Promise<Insight[]> {
  return invoke<Insight[]>('get_milestones', { limit });
}

export async function getStreakData(): Promise<StreakData> {
  return invoke<StreakData>('get_streak_data');
}

// ==================== 自动化 API ====================

export async function dailyCheckin(): Promise<DailyCheckin> {
  return invoke<DailyCheckin>('daily_checkin');
}

export async function onExecutionComplete(executionId: string, feedback?: ExecutionFeedback): Promise<ExecutionCompleteResult> {
  return invoke<ExecutionCompleteResult>('on_execution_complete', { executionId, feedback });
}

export async function getTodayRecommendations(): Promise<TodayRecommendation> {
  return invoke<TodayRecommendation>('get_today_recommendations');
}
