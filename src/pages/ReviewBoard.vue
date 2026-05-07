<template>
  <div class="space-y-5">
    <div>
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white">复习看板</h2>
      <p class="text-gray-500 dark:text-gray-400 text-sm mt-0.5">艾宾浩斯复习日历，追踪知识点复习进度</p>
    </div>

    <!-- 统计卡片 -->
    <div class="grid grid-cols-2 lg:grid-cols-4 gap-3">
      <div class="card text-center py-3">
        <p class="text-2xl font-bold" :class="stats.due_today > 0 ? 'text-red-500' : 'text-green-500'">{{ stats.due_today }}</p>
        <p class="text-xs text-gray-500 dark:text-gray-400">今日待复习</p>
      </div>
      <div class="card text-center py-3">
        <p class="text-2xl font-bold text-amber-500">{{ stats.due_this_week }}</p>
        <p class="text-xs text-gray-500 dark:text-gray-400">本周待复习</p>
      </div>
      <div class="card text-center py-3">
        <p class="text-2xl font-bold text-primary-500">{{ streakData.current_streak }}</p>
        <p class="text-xs text-gray-500 dark:text-gray-400">连续打卡（天）</p>
      </div>
      <div class="card text-center py-3">
        <p class="text-2xl font-bold text-green-500">{{ stats.completed_this_week }}</p>
        <p class="text-xs text-gray-500 dark:text-gray-400">本周已完成</p>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-5">
      <!-- 左侧：月历 + 热力图 -->
      <div class="lg:col-span-2 space-y-5">
        <!-- 月历 -->
        <div class="card">
          <div class="flex items-center justify-between mb-3">
            <button @click="prevMonth" class="p-1.5 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-400">←</button>
            <h3 class="font-semibold text-gray-900 dark:text-white text-center">{{ monthLabel }}</h3>
            <button @click="nextMonth" class="p-1.5 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-400">→</button>
          </div>
          <!-- 星期头 -->
          <div class="grid grid-cols-7 gap-1 text-center text-xs font-medium text-gray-400 dark:text-gray-500 mb-1">
            <div>一</div><div>二</div><div>三</div><div>四</div><div>五</div><div>六</div><div>日</div>
          </div>
          <!-- 日期格 -->
          <div class="grid grid-cols-7 gap-1">
            <button v-for="(day, idx) in calDays" :key="idx"
              @click="day.date ? selectDate(day.date) : null"
              :disabled="!day.date"
              class="aspect-square rounded-lg flex flex-col items-center justify-center transition-colors text-xs relative"
              :class="day.date
                ? (day.date === selectedDate
                  ? 'bg-primary-500 text-white shadow-md'
                  : day.isToday
                    ? 'bg-primary-50 dark:bg-primary-900/20 text-primary-600 dark:text-primary-400 font-bold border border-primary-200 dark:border-primary-700'
                    : day.reviewCount > 0
                      ? 'bg-amber-50 dark:bg-amber-900/20 hover:bg-amber-100 dark:hover:bg-amber-900/40 text-gray-900 dark:text-white'
                      : 'hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-900 dark:text-white')
                : 'text-gray-300 dark:text-gray-600'">
              <span>{{ day.dayNum }}</span>
              <span v-if="day.reviewCount > 0" class="text-[10px] leading-none mt-0.5"
                :class="day.date === selectedDate ? 'text-white' : day.isToday ? 'text-primary-500' : 'text-amber-600 dark:text-amber-400'">
                {{ day.reviewCount }}个
              </span>
            </button>
          </div>
        </div>

        <!-- 热力图 -->
        <div class="card">
          <h3 class="font-semibold text-gray-900 dark:text-white mb-3 text-sm">📈 近 30 天复习完成情况</h3>
          <div v-if="heatmap.length === 0" class="text-center py-3 text-xs text-gray-400">暂无数据</div>
          <div v-else class="flex items-end gap-1 flex-wrap">
            <div v-for="(h, idx) in heatmap" :key="idx"
              :title="heatmapLabel(h)"
              class="flex-1 min-w-[8px] rounded-sm transition-colors"
              :style="{ height: Math.max(4, h.count * 6) + 'px', backgroundColor: heatColor(h.count) }">
            </div>
          </div>
          <div class="flex items-center gap-2 mt-2 text-[10px] text-gray-400">
            <span>少</span>
            <span class="w-3 h-3 rounded-sm" style="background:#e5e7eb"></span>
            <span class="w-3 h-3 rounded-sm" style="background:#fcd34d"></span>
            <span class="w-3 h-3 rounded-sm" style="background:#f97316"></span>
            <span class="w-3 h-3 rounded-sm" style="background:#22c55e"></span>
            <span>多</span>
          </div>
        </div>
      </div>

      <!-- 右侧：选中日期的复习清单 -->
      <div class="card">
        <h3 class="font-semibold text-gray-900 dark:text-white mb-3 text-sm">
          {{ selectedDateLabel }}
          <span v-if="dayReviews.length > 0" class="text-gray-400 font-normal">（{{ dayReviews.length }}个）</span>
        </h3>

        <div v-if="dayReviews.length === 0" class="text-center py-8">
          <span class="text-3xl">🎉</span>
          <p class="text-sm text-gray-400 mt-2">当天没有待复习内容</p>
        </div>

        <div v-else class="space-y-2">
          <div v-for="item in dayReviews" :key="item.session_id"
            class="p-3 rounded-xl border border-amber-200 dark:border-amber-800 bg-amber-50/30 dark:bg-amber-900/5">
            <div class="flex items-center gap-2 mb-1.5">
              <span class="text-xs px-1.5 py-0.5 rounded font-medium text-white flex-shrink-0"
                :style="{ backgroundColor: item.subject_color }">{{ item.subject_name }}</span>
              <span class="text-xs text-gray-400 flex-shrink-0">掌握 {{ Math.round(item.mastery_level * 100) }}%</span>
            </div>
            <p class="text-sm font-medium text-gray-900 dark:text-white mb-2">{{ item.kp_name }}</p>
            <div class="flex gap-1.5">
              <button @click="startReview(item)"
                class="flex-1 py-1.5 text-xs rounded-lg bg-primary-500 text-white hover:bg-primary-600 transition-colors">
                开始复习（25分钟）
              </button>
              <button @click="skipReview(item.session_id)"
                class="px-3 py-1.5 text-xs rounded-lg bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors">
                跳过
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import {
  getUpcomingReviews, getReviewStats, getStreakData,
  getReviewHeatmap, skipReviewSession,
} from '../composables/api';
import type { ReviewSessionWithKp, ReviewStats, StreakData, DailyReviewCount } from '../types';

const router = useRouter();

const stats = ref<ReviewStats>({
  due_today: 0, due_this_week: 0, completed_this_week: 0,
  skipped_this_week: 0, avg_mastery_score: 0, total_kps: 0, mastered_kps: 0,
});
const streakData = ref<StreakData>({
  current_streak: 0, longest_streak: 0, weekly_days: [], total_study_days: 0,
});
const upcomingReviews = ref<ReviewSessionWithKp[]>([]);
const heatmap = ref<DailyReviewCount[]>([]);
const selectedDate = ref('');
const todayStr = computed(() => {
  const d = new Date();
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
});

// 月历
const monthOffset = ref(0);

const monthLabel = computed(() => {
  const d = new Date(new Date().getFullYear(), new Date().getMonth() + monthOffset.value, 1);
  return `${d.getFullYear()}年${d.getMonth() + 1}月`;
});

const calDays = computed(() => {
  const today = new Date();
  const baseDate = new Date(today.getFullYear(), today.getMonth() + monthOffset.value, 1);
  const year = baseDate.getFullYear();
  const month = baseDate.getMonth();
  const firstDayJS = baseDate.getDay();
  const startCol = firstDayJS === 0 ? 6 : firstDayJS - 1;
  const daysInMonth = new Date(year, month + 1, 0).getDate();
  const todayDateStr = `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(today.getDate()).padStart(2, '0')}`;

  // 按日期构建复习计数
  const countMap: Record<string, number> = {};
  const nowS = Math.floor(Date.now() / 1000);
  for (const r of upcomingReviews.value) {
    const d = new Date(r.scheduled_date * 1000);
    let key = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
    // 逾期任务：归入今天，保持日历与日计划列表/统计卡口径一致
    if (r.scheduled_date < nowS) {
      key = todayDateStr;
    }
    countMap[key] = (countMap[key] || 0) + 1;
  }

  const totalRows = Math.ceil((startCol + daysInMonth) / 7);
  const totalCells = totalRows * 7;
  const result: Array<{ date: string | null; dayNum: number; isToday: boolean; reviewCount: number }> = [];

  let dayNum = 1;
  for (let i = 0; i < totalCells; i++) {
    if (i < startCol || dayNum > daysInMonth) {
      result.push({ date: null, dayNum: 0, isToday: false, reviewCount: 0 });
    } else {
      const d = new Date(year, month, dayNum);
      const dateStr = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
      result.push({
        date: dateStr,
        dayNum,
        isToday: dateStr === todayDateStr,
        reviewCount: countMap[dateStr] || 0,
      });
      dayNum++;
    }
  }
  return result;
});

function prevMonth() { monthOffset.value--; }
function nextMonth() { monthOffset.value++; }

// 选中日期
function selectDate(date: string) {
  selectedDate.value = date;
}

const selectedDateLabel = computed(() => {
  if (!selectedDate.value) return '选择日期';
  const [, m, d] = selectedDate.value.split('-').map(Number);
  return `${m}月${d}日`;
});

// 选中日期的复习列表
const dayReviews = computed(() => {
  if (!selectedDate.value) return [];
  const dayStart = Math.floor(new Date(selectedDate.value + 'T00:00:00').getTime() / 1000);
  const dayEnd = dayStart + 86400;
  const now = Math.floor(Date.now() / 1000);

  // 今天或过去日期：显示所有到期未完成的（含逾期），与顶部"今日待复习"口径一致
  if (dayEnd <= now || selectedDate.value === todayStr.value) {
    return upcomingReviews.value.filter(r =>
      r.scheduled_date < dayEnd && !r.was_skipped
    );
  }
  // 未来日期：仅显示当天排期的
  return upcomingReviews.value.filter(r =>
    r.scheduled_date >= dayStart && r.scheduled_date < dayEnd && !r.was_skipped
  );
});

// 热力图
function heatColor(count: number): string {
  if (count === 0) return '#e5e7eb';
  if (count <= 2) return '#fcd34d';
  if (count <= 5) return '#f97316';
  return '#22c55e';
}

function heatmapLabel(h: DailyReviewCount): string {
  const d = new Date(h.date * 1000);
  return `${d.getMonth() + 1}/${d.getDate()} ${h.count}次`;
}

// 开始复习 → 跳转首页
function startReview(item: ReviewSessionWithKp) {
  router.push({ path: '/', query: { review: item.session_id } });
}

// 跳过复习
async function skipReview(id: string) {
  try {
    await skipReviewSession(id);
    // 刷新
    upcomingReviews.value = await getUpcomingReviews(60);
    stats.value = await getReviewStats();
  } catch (e) { console.error('跳过复习失败:', e); }
}

async function loadData() {
  try {
    const results = await Promise.allSettled([
      getUpcomingReviews(60),
      getReviewStats(),
      getStreakData(),
      getReviewHeatmap(30),
    ]);
    if (results[0].status === 'fulfilled') upcomingReviews.value = results[0].value;
    if (results[1].status === 'fulfilled') stats.value = results[1].value;
    if (results[2].status === 'fulfilled') streakData.value = results[2].value;
    if (results[3].status === 'fulfilled') heatmap.value = results[3].value;
  } catch (e) { console.error('加载数据失败:', e); }
}

onMounted(() => {
  selectedDate.value = todayStr.value;
  loadData();
});
</script>
