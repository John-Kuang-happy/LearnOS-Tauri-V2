<template>
  <div class="space-y-6">
    <div>
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white">数据看板</h2>
      <p class="text-gray-500 dark:text-gray-400 mt-1">学习数据一览</p>
    </div>

    <!-- 统计卡片 -->
    <div class="grid grid-cols-2 lg:grid-cols-5 gap-4">
      <div class="card">
        <div class="flex items-center gap-3">
          <span class="text-3xl">⏱️</span>
          <div>
            <p class="text-sm text-gray-500 dark:text-gray-400">本周学习</p>
            <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ stats.weekly_hours.toFixed(1) }}h</p>
            <p v-if="totalHoursGoal > 0" class="text-xs text-gray-400 mt-0.5">
              目标 {{ totalHoursGoal }}h · 达成 {{ overallTimePct }}%
            </p>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="flex items-center gap-3">
          <span class="text-3xl">📖</span>
          <div>
            <p class="text-sm text-gray-500 dark:text-gray-400">新掌握KP</p>
            <p class="text-2xl font-bold text-purple-500">+{{ stats.new_kps_this_week }}</p>
            <p v-if="stats.total_weekly_goal_kps > 0" class="text-xs text-gray-400 mt-0.5">
              目标 {{ stats.total_weekly_goal_kps }}个 · 达成 {{ overallKpPct }}%
            </p>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="flex items-center gap-3">
          <span class="text-3xl">✅</span>
          <div>
            <p class="text-sm text-gray-500 dark:text-gray-400">已完成计划</p>
            <p class="text-2xl font-bold text-emerald-500">{{ stats.completed_plans }}</p>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="flex items-center gap-3">
          <span class="text-3xl">🔄</span>
          <div>
            <p class="text-sm text-gray-500 dark:text-gray-400">复盘次数</p>
            <p class="text-2xl font-bold text-primary-500">{{ stats.review_count }}</p>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="flex items-center gap-3">
          <span class="text-3xl">🧠</span>
          <div>
            <p class="text-sm text-gray-500 dark:text-gray-400">已掌握KP</p>
            <p class="text-2xl font-bold text-accent-500">{{ stats.mastered_kp_count }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 图表区域 -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 科目综合达成 -->
      <div class="card">
        <h3 class="font-semibold text-gray-900 dark:text-white mb-4">科目综合达成</h3>
        <div v-if="weeklyProgress.length === 0" class="text-center py-8 text-gray-500 dark:text-gray-400">
          暂无数据，开始学习吧！
        </div>
        <div v-else class="space-y-3">
          <div v-for="item in weeklyProgress" :key="item.subject_id" class="flex items-center gap-3">
            <div class="w-3 h-3 rounded-full flex-shrink-0" :style="{ backgroundColor: item.color }"></div>
            <div class="flex-1 min-w-0">
              <div class="flex items-center justify-between mb-1">
                <span class="text-sm text-gray-700 dark:text-gray-300">{{ item.subject_name }}</span>
                <span class="text-xs text-gray-500 dark:text-gray-400">
                  ⏱{{ item.weekly_hours.toFixed(1) }}/{{ item.goal_hours }}h
                  <template v-if="item.goal_kps > 0"> · 📖+{{ item.new_kps }}/{{ item.goal_kps }}个</template>
                </span>
              </div>
              <div class="w-full h-2 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
                <div class="h-full rounded-full transition-all" :style="{ width: (item.composite_rate * 100) + '%', backgroundColor: item.color }"></div>
              </div>
            </div>
            <span class="text-sm font-bold flex-shrink-0 w-10 text-right"
              :class="item.composite_rate >= 0.8 ? 'text-green-500' : item.composite_rate >= 0.4 ? 'text-amber-500' : 'text-red-500'">
              {{ Math.round(item.composite_rate * 100) }}%
            </span>
          </div>
        </div>
      </div>

      <!-- 最近复盘 -->
      <div class="card">
        <h3 class="font-semibold text-gray-900 dark:text-white mb-4">最近复盘</h3>
        <div v-if="recentReviews.length === 0" class="text-center py-8 text-gray-500 dark:text-gray-400">
          暂无复盘记录
        </div>
        <div v-else class="space-y-2">
          <div v-for="r in recentReviews" :key="r.id"
            class="flex items-center justify-between p-3 rounded-xl bg-gray-50 dark:bg-gray-800">
            <div>
              <p class="text-sm font-medium text-gray-900 dark:text-white">{{ r.plan_title }}</p>
              <p class="text-xs text-gray-500 dark:text-gray-400">{{ r.subject_name }} · {{ formatDate(r.review_date) }}</p>
            </div>
            <div class="flex items-center gap-2">
              <span class="text-sm">{{ moodEmoji(r.mood_score) }}</span>
              <span class="text-xs text-gray-500">{{ r.what_went_well?.slice(0, 20) }}{{ r.what_went_well?.length > 20 ? '...' : '' }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { getDashboardStats, getSubjectDistribution, getRecentReviews, getWeeklyGoalProgress } from '../composables/api';
import type { DashboardStats, SubjectDistribution, WeeklyGoalProgress } from '../types';

const stats = ref<DashboardStats>({
  weekly_hours: 0, completed_plans: 0, avg_completion_rate: 0,
  review_count: 0, due_reviews_count: 0, mastered_kp_count: 0,
  new_kps_this_week: 0, total_weekly_goal_kps: 0,
});

const subjectDist = ref<SubjectDistribution[]>([]);
const weeklyProgress = ref<WeeklyGoalProgress[]>([]);
const recentReviews = ref<any[]>([]);

// 时间总目标（从 weeklyProgress 汇总）
const totalHoursGoal = computed(() => {
  return weeklyProgress.value.reduce((sum, p) => sum + p.goal_hours, 0);
});

// 整体时间达成率
const overallTimePct = computed(() => {
  if (totalHoursGoal.value <= 0) return 0;
  return Math.min(100, Math.round((stats.value.weekly_hours / totalHoursGoal.value) * 100));
});

// 整体KP达成率
const overallKpPct = computed(() => {
  if (stats.value.total_weekly_goal_kps <= 0) return 0;
  return Math.min(100, Math.round((stats.value.new_kps_this_week / stats.value.total_weekly_goal_kps) * 100));
});

function moodEmoji(score: number) {
  return ['😫', '😔', '😐', '😊', '😄'][score - 1] || '😐';
}

function formatDate(ts: number) {
  return new Date(ts * 1000).toLocaleDateString('zh-CN');
}

onMounted(async () => {
  try {
    const results = await Promise.allSettled([
      getDashboardStats(),
      getSubjectDistribution(),
      getRecentReviews(5),
      getWeeklyGoalProgress(),
    ]);
    if (results[0].status === 'fulfilled') stats.value = results[0].value;
    if (results[1].status === 'fulfilled') subjectDist.value = results[1].value;
    if (results[2].status === 'fulfilled') recentReviews.value = results[2].value;
    if (results[3].status === 'fulfilled') weeklyProgress.value = results[3].value;
  } catch (e) {
    console.error('加载仪表盘数据失败:', e);
  }
});
</script>
