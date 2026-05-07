<template>
  <div class="space-y-5">
    <!-- 顶部：问候 + 考试 + 进度条 -->
    <div class="flex items-start justify-between">
      <div>
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white">{{ checkin.greeting }}</h2>
        <p class="text-gray-500 dark:text-gray-400 text-sm mt-0.5">{{ todayStr }}</p>
      </div>
      <div class="flex items-center gap-4">
        <div v-if="nearestExam" class="text-right">
          <p class="text-xs text-gray-500 dark:text-gray-400">{{ examLabel }}</p>
          <p class="text-xl font-bold" :class="examColorClass">{{ examDays }} 天</p>
        </div>
      </div>
    </div>

    <!-- 本周目标总览 -->
    <div v-if="weeklyOverview !== null" class="card py-3">
      <div class="flex items-center justify-between mb-2">
        <span class="text-xs font-medium text-gray-500 dark:text-gray-400">🎯 本周目标总览</span>
        <span class="text-xs text-gray-400">还剩 {{ weeklyOverview.daysLeft }} 天</span>
      </div>
      <!-- 各科目精简进度 -->
      <div class="flex items-center gap-3 mb-2 flex-wrap">
        <div v-for="s in weeklyOverview.subjects" :key="s.subject_id" class="flex items-center gap-1">
          <div class="w-1.5 h-1.5 rounded-full flex-shrink-0" :style="{ backgroundColor: s.color }"></div>
          <span class="text-xs text-gray-700 dark:text-gray-300">{{ s.name }}</span>
          <span class="text-xs font-medium"
            :class="s.percent >= 80 ? 'text-green-500' : s.percent >= 40 ? 'text-amber-500' : 'text-red-500'">
            {{ s.percent }}%
          </span>
        </div>
        <span v-if="weeklyOverview.subjects.length === 0" class="text-xs text-gray-400">暂无科目目标</span>
      </div>
      <!-- 总体进度条 -->
      <div class="flex items-center gap-2">
        <div class="flex-1 h-2 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
          <div class="h-full rounded-full transition-all" :style="{ width: weeklyOverview.overall + '%', backgroundColor: weeklyOverview.overall >= 80 ? '#22c55e' : weeklyOverview.overall >= 40 ? '#f59e0b' : '#ef4444' }"></div>
        </div>
        <span class="text-sm font-bold text-gray-900 dark:text-white w-10 text-right">{{ weeklyOverview.overall }}%</span>
      </div>
    </div>

    <!-- 主区域：推荐列表 + 番茄钟 -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-5">
      <!-- 左：推荐列表（占2列） -->
      <div class="lg:col-span-2 space-y-4">
        <!-- 数据加载错误提示 -->
        <div v-if="loadError" class="card border-l-4 border-l-red-500 bg-red-50 dark:bg-red-900/20 p-4">
          <p class="text-sm text-red-700 dark:text-red-400 font-medium">⚠️ {{ loadError }}</p>
          <p class="text-xs text-red-500 dark:text-red-400 mt-1">请打开浏览器控制台（F12）查看详细错误信息，或重启应用后重试。</p>
        </div>
        <!-- AI 推荐卡片 -->
        <div class="card">
          <div class="flex items-center justify-between mb-4">
            <h3 class="font-semibold text-gray-900 dark:text-white">🎯 今日推荐</h3>
            <span class="text-xs text-gray-400">{{ recommendation.items.length }} 项</span>
          </div>

          <!-- 空状态 -->
          <div v-if="recommendation.items.length === 0" class="text-center py-10">
            <span class="text-4xl">🎉</span>
            <p class="text-gray-500 dark:text-gray-400 mt-3">今天没有待办任务</p>
            <p class="text-xs text-gray-400 mt-1">在「学习计划」页面创建计划，或添加知识点后会自动生成复习任务</p>
            <router-link to="/plans" class="btn-primary inline-block mt-4 text-sm">+ 创建计划</router-link>
          </div>

          <!-- 推荐列表 -->
          <div v-else class="space-y-2">
            <div v-for="(item, idx) in recommendation.items" :key="item.id"
              @click="item.item_type === 'review' ? onStartReview(item) : onStartPlan(item)"
              class="flex items-center gap-3 p-3 rounded-lg border cursor-pointer hover:shadow-sm transition-all group"
              :class="[
                item.item_type === 'review'
                  ? 'border-amber-200 dark:border-amber-800 bg-amber-50/50 dark:bg-amber-900/5 hover:border-amber-400 dark:hover:border-amber-600'
                  : 'border-gray-200 dark:border-gray-700 hover:border-primary-300 dark:hover:border-primary-700 hover:bg-gray-50 dark:hover:bg-gray-800/50',
                currentItemId === item.id ? 'opacity-60 pointer-events-none' : ''
              ]">
              <span class="text-sm font-bold text-gray-400 flex-shrink-0 w-5 text-center">{{ idx + 1 }}</span>
              <span class="text-xs px-2 py-1 rounded font-medium text-white flex-shrink-0"
                :style="{ backgroundColor: item.subject_color }">{{ item.subject_name }}</span>
              <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-gray-900 dark:text-white truncate">{{ item.title }}</p>
                <p class="text-xs text-gray-400 mt-0.5">
                  <template v-if="item.item_type === 'review'">
                    掌握 {{ Math.round(item.mastery_level * 100) }}%
                    <template v-if="item.overdue_days > 0">
                      <span class="text-red-500"> · 已逾期 {{ item.overdue_days }} 天</span>
                    </template>
                    <template v-else> · 今日到期</template>
                  </template>
                  <template v-else>
                    计划 {{ item.estimated_minutes }} 分钟 · {{ item.reason }}
                    <span v-if="planKpMap[item.id]" class="text-gray-400">
                      · 📖 {{ planKpMap[item.id].mastered }}/{{ planKpMap[item.id].total }} 个已掌握
                    </span>
                  </template>
                </p>
              </div>
              <span class="text-xs text-gray-400 flex-shrink-0">{{ item.estimated_minutes }}min</span>
              <span class="text-xs px-2 py-1 rounded-full flex-shrink-0"
                :class="currentItemId === item.id
                  ? 'bg-gray-400 text-white'
                  : item.item_type === 'review' ? 'bg-amber-500 text-white' : 'bg-primary-500 text-white'">
                {{ currentItemId === item.id ? '⏳ 进行中' : (item.item_type === 'review' ? '复习' : '开始') }}
              </span>
            </div>
          </div>
        </div>

        <!-- 今日已完成 -->
        <div v-if="todayCompleted.length > 0" class="card">
          <div class="flex items-center justify-between mb-3">
            <h3 class="font-semibold text-gray-900 dark:text-white">✅ 今日已完成</h3>
            <span class="text-xs text-gray-400">{{ todayCompleted.length }} 项</span>
          </div>
          <div class="space-y-1">
            <div v-for="e in todayCompleted" :key="e.id"
              class="flex items-center gap-2 text-sm p-2 rounded-lg bg-gray-50 dark:bg-gray-800/50">
              <span class="text-xs px-1.5 py-0.5 rounded font-medium text-white flex-shrink-0"
                :style="{ backgroundColor: getSubjectColor(e.subject_id) }">{{ getSubjectName(e.subject_id) }}</span>
              <span class="text-gray-900 dark:text-white truncate flex-1">{{ e.plan_title || '未命名任务' }}</span>
              <span class="text-xs text-gray-400 flex-shrink-0">{{ (e.actual_hours || 0).toFixed(1) }}h</span>
              <span class="text-xs flex-shrink-0 w-10 text-right"
                :class="(e.completion_rate || 0) >= 0.8 ? 'text-green-500' : 'text-amber-500'">
                {{ Math.round((e.completion_rate || 0) * 100) }}%
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- 右：番茄钟 + 今日摘要 -->
      <div class="space-y-4">
        <!-- 番茄钟 -->
        <PomodoroInline
          ref="pomodoroRef"
          :study-title="activeStudyTitle"
          @complete="onPomodoroComplete"
        />

        <!-- 掌握度反馈卡片 -->
        <MasteryCard
          ref="masteryRef"
          :plan-title="activeStudyTitle"
          @submit="onMasterySubmit"
          @retry="onRetry"
          @close="onMasteryClose"
        />

        <!-- 今日摘要 -->
        <div class="card">
          <h3 class="font-semibold text-gray-900 dark:text-white mb-3">📊 今日摘要</h3>
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-500 dark:text-gray-400">已学时长</span>
              <span class="text-sm font-bold text-green-600">{{ checkin.today_minutes }} 分钟</span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-500 dark:text-gray-400">已完成</span>
              <span class="text-sm font-bold text-primary-600">{{ checkin.today_completed }} 个</span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-500 dark:text-gray-400">待复习</span>
              <span class="text-sm font-bold"
                :class="dueCount > 0 ? 'text-amber-600' : 'text-gray-600 dark:text-gray-400'">
                {{ dueCount }} 个
              </span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-500 dark:text-gray-400">待办计划</span>
              <span class="text-sm font-bold text-blue-600">{{ planCount }} 个</span>
            </div>
          </div>

          <!-- 今日建议 -->
          <div v-if="recommendation.suggestion" class="mt-4 pt-3 border-t border-gray-200 dark:border-gray-700">
            <p class="text-xs text-gray-400 mb-1">💡 今日建议</p>
            <p class="text-sm text-gray-700 dark:text-gray-300">{{ recommendation.suggestion }}</p>
          </div>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import {
  getTodayRecommendations, getUpcomingExams,
  getAllPlans, getExecutionsByDate, getAllSubjects,
  startExecution, endExecution, onExecutionComplete,
  submitReviewFeedback, getRecentInsights, getWeeklyGoalProgress,
  getAllKnowledgePoints,
} from '../composables/api';
import type { TodayRecommendation, RecommendedItem, Plan, Subject, Exam, Insight, WeeklyGoalProgress } from '../types';
import PomodoroInline from '../components/home/PomodoroInline.vue';
import MasteryCard from '../components/home/MasteryCard.vue';

const recommendation = ref<TodayRecommendation>({
  total_estimated_minutes: 0, completed_minutes: 0, items: [], suggestion: '',
});
const loadError = ref('');
const todayCompleted = ref<any[]>([]);
const subjects = ref<Subject[]>([]);
const exams = ref<Exam[]>([]);
const insights = ref<Insight[]>([]);
const weeklyProgress = ref<WeeklyGoalProgress[]>([]);

const pomodoroRef = ref<InstanceType<typeof PomodoroInline> | null>(null);
const masteryRef = ref<InstanceType<typeof MasteryCard> | null>(null);
const activeStudyTitle = ref('');
const currentMode = ref<'plan' | 'review'>('plan');
const currentItemId = ref('');
const currentHasKp = ref(true);
let activeExecutionId = '';
let currentSessionId = '';
let currentElapsedSeconds = 0;
const allPlansMap = ref<Record<string, Plan>>({});
const planKpMap = ref<Record<string, { total: number; mastered: number }>>({});

// 从 recommendation 中计算统计值
const dueCount = computed(() => recommendation.value.items.filter(i => i.item_type === 'review').length);
const planCount = computed(() => recommendation.value.items.filter(i => i.item_type === 'plan').length);

const checkin = computed(() => {
  const hour = new Date().getHours();
  const greeting = hour < 12 ? '早上好 ☀️' : hour < 18 ? '下午好 🌤️' : '晚上好 🌙';
  return { greeting, today_minutes: recommendation.value.completed_minutes, today_completed: todayCompleted.value.length };
});

// 本周目标总览
const weeklyOverview = computed(() => {
  const items = weeklyProgress.value;
  if (items.length === 0) return null;

  // 各科目综合达成率
  const subjects = items.map(p => ({
    subject_id: p.subject_id,
    name: p.subject_name,
    color: p.color,
    percent: Math.round(p.composite_rate * 100),
  }));

  // 总体：按目标小时加权的综合达成率
  let totalGoalHours = 0;
  let weightedRate = 0;
  for (const p of items) {
    totalGoalHours += p.goal_hours;
    weightedRate += p.composite_rate * p.goal_hours;
  }
  const overall = totalGoalHours > 0
    ? Math.min(100, Math.round((weightedRate / totalGoalHours) * 100))
    : 0;

  // 距离周末剩余天数
  const now = new Date();
  const dayOfWeek = now.getDay() || 7; // 周日=7
  const daysLeft = Math.max(0, 7 - dayOfWeek);

  return { subjects, overall, daysLeft };
});

// 日期格式化
const todayStr = computed(() => {
  const d = new Date();
  return `${d.getFullYear()}年${d.getMonth() + 1}月${d.getDate()}日 ${['周日', '周一', '周二', '周三', '周四', '周五', '周六'][d.getDay()]}`;
});

// 最近考试
const nearestExam = computed(() => {
  const active = exams.value.filter(e => e.target_date * 1000 > Date.now());
  active.sort((a, b) => a.target_date - b.target_date);
  return active[0] || null;
});
const examDays = computed(() => {
  if (!nearestExam.value) return '--';
  return Math.max(0, Math.ceil((nearestExam.value.target_date * 1000 - Date.now()) / 86400000));
});
const examLabel = computed(() => {
  if (!nearestExam.value) return '';
  const labels: Record<string, string> = { gaokao: '距离高考', final: '距离期末', midterm: '距离期中', mock: '距离模拟考' };
  return labels[nearestExam.value.exam_type] || nearestExam.value.name;
});
const examColorClass = computed(() => {
  const d = Number(examDays.value);
  if (d <= 7) return 'text-red-500';
  if (d <= 30) return 'text-amber-500';
  return 'text-primary-500';
});

function getSubjectName(id: string) {
  if (!id) return '未归类';
  return subjects.value.find(s => s.id === id)?.name || '未归类';
}
function getSubjectColor(id: string) {
  if (!id) return '#94a3b8';
  return subjects.value.find(s => s.id === id)?.color || '#94a3b8';
}

// 开始复习（不创建 plan，直接启动番茄钟）
function onStartReview(item: RecommendedItem) {
  activeStudyTitle.value = `复习：${item.title}`;
  currentMode.value = 'review';
  currentItemId.value = item.id;
  currentSessionId = item.id;
  activeExecutionId = '';
  pomodoroRef.value?.startWithTitle(`复习：${item.title}`, 25);
}

// 开始计划（创建 execution，启动番茄钟）
async function onStartPlan(item: RecommendedItem) {
  activeStudyTitle.value = item.title;
  currentMode.value = 'plan';
  currentItemId.value = item.id;
  currentSessionId = '';
  // 检查计划是否关联了知识点
  const plan = allPlansMap.value[item.id];
  currentHasKp.value = !!(plan?.source_kp_id);
  try {
    const exec = await startExecution({ plan_id: item.id });
    activeExecutionId = exec.id;
  } catch (e) {
    console.error('创建执行记录失败，无法启动学习:', e);
    activeExecutionId = '';
    return;
  }
  pomodoroRef.value?.startWithTitle(item.title, Math.max(25, Math.round(item.estimated_minutes)));
}

// 番茄钟完成
async function onPomodoroComplete(seconds: number) {
  currentElapsedSeconds = seconds;
  const hours = seconds / 3600;
  const rate = seconds > 0 ? Math.min(1, seconds / (25 * 60)) : 0.8;
  if (activeExecutionId) {
    try {
      await endExecution(activeExecutionId, { actual_hours: hours, completion_rate: rate });
    } catch (e) {
      console.error('结束执行记录失败:', e);
    }
  }
  masteryRef.value?.show(activeStudyTitle.value, currentHasKp.value);
}

// 掌握度提交
async function onMasterySubmit(data: { mastery_score: number; mastery_label: string; mood_score: number }) {
  let nextDate = '';
  if (currentMode.value === 'review' && currentSessionId) {
    try {
      await submitReviewFeedback({
        session_id: currentSessionId,
        mastery_score: data.mastery_score,
        time_spent_seconds: currentElapsedSeconds,
      });
      currentSessionId = '';
    } catch (e) { console.error('提交复习反馈失败:', e); }
  } else if (activeExecutionId) {
    try {
      const result = await onExecutionComplete(activeExecutionId, {
        mastery_score: data.mastery_score,
        mastery_label: data.mastery_label,
        mood_score: data.mood_score,
      });
      nextDate = result.next_review_date || '';
    } catch (e) { console.error('执行完成处理失败:', e); }
  }
  // 刷新数据
  await loadData();

  // 计算掌握状态
  const mastered = data.mastery_score >= 0.9;
  let msg = '';
  if (data.mastery_score >= 0.9) {
    msg = '已掌握';
  } else if (data.mastery_score >= 0.65) {
    msg = '还需巩固 1 次';
  } else {
    msg = '建议重新学习';
  }

  // 显示结果页
  masteryRef.value?.showResult({
    mastered,
    nextDate,
    message: msg,
  });
}

// 再学一次
function onRetry() {
  const itemId = currentItemId.value;
  const mode = currentMode.value;
  const title = activeStudyTitle.value;
  // 清理当前状态
  activeExecutionId = '';
  currentElapsedSeconds = 0;
  // 重新开始
  if (mode === 'plan') {
    onStartPlan({ id: itemId, title, estimated_minutes: 25, subject_name: '', subject_color: '', mastery_level: 0, overdue_days: 0, priority: 1, reason: '', item_type: 'plan' });
  } else {
    onStartReview({ id: itemId, title, estimated_minutes: 25, subject_name: '', subject_color: '', mastery_level: 0, overdue_days: 0, priority: 1, reason: '', item_type: 'review' });
  }
}

// 结果页关闭
function onMasteryClose() {
  activeExecutionId = '';
  activeStudyTitle.value = '';
  currentItemId.value = '';
  currentElapsedSeconds = 0;
}

// 数据加载
async function loadData() {
  try {
    const todayStart = Math.floor(new Date().setHours(0, 0, 0, 0) / 1000);
    // 使用 allSettled 避免单个接口失败导致所有数据不加载
    const results = await Promise.allSettled([
      getTodayRecommendations(),
      getAllSubjects(),
      getUpcomingExams(),
      getRecentInsights(5),
    ]);
    if (results[0].status === 'fulfilled') recommendation.value = results[0].value;
    else { loadError.value = `数据加载失败: ${results[0].reason}`; console.error('getTodayRecommendations 失败:', results[0].reason); }
    if (results[1].status === 'fulfilled') subjects.value = results[1].value;
    if (results[2].status === 'fulfilled') exams.value = results[2].value;
    if (results[3].status === 'fulfilled') insights.value = results[3].value;
    // 加载本周目标进度
    try {
      weeklyProgress.value = await getWeeklyGoalProgress();
    } catch (e) { console.error('加载周目标进度失败:', e); }
    const todayExecs = await getExecutionsByDate(todayStart);
    todayCompleted.value = todayExecs.filter((e: any) => e.end_time).slice(0, 10);
    // 补充 subject_id 和 plan_title
    const allPlans = await getAllPlans();
    const planMap: Record<string, Plan> = {};
    for (const p of allPlans) { planMap[p.id] = p; }
    allPlansMap.value = planMap;
    // 加载各计划知识点掌握进度
    try {
      const allKps = await getAllKnowledgePoints();
      const kpMap: Record<string, { total: number; mastered: number }> = {};
      for (const kp of allKps) {
        if (kp.source) {
          if (!kpMap[kp.source]) kpMap[kp.source] = { total: 0, mastered: 0 };
          kpMap[kp.source].total++;
          if (kp.is_mastered) kpMap[kp.source].mastered++;
        }
      }
      planKpMap.value = kpMap;
    } catch (e) { console.error('加载知识点数据失败:', e); }
    for (const e of todayCompleted.value) {
      const plan = planMap[e.plan_id];
      if (plan) { e.subject_id = plan.subject_id; e.plan_title = plan.title; }
      else { e.plan_title = '未命名任务'; }
    }
  } catch (e) { console.error('[Home] loadData 出错:', e); }
}

onMounted(() => { loadData(); });
</script>
