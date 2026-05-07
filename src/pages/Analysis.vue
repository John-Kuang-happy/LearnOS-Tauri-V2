<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white">智能分析</h2>
        <p class="text-gray-500 dark:text-gray-400 mt-1">多维度分析学习状况，发现薄弱环节</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="doGenerateSuggestions" class="btn-secondary text-sm" :disabled="analyzing">
          💡 生成建议
        </button>
        <button @click="doAiAdvice" class="btn-secondary text-sm" :disabled="aiBusy">
          {{ aiBusy ? 'AI 思考中...' : '🤖 AI 深度分析' }}
        </button>
        <button @click="doAnalysis" class="btn-primary" :disabled="analyzing">
          {{ analyzing ? '分析中...' : '🔍 运行分析' }}
        </button>
      </div>
    </div>

    <!-- 分析摘要 -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
      <div class="card text-center py-3">
        <p class="text-2xl font-bold text-primary-500">{{ suggestionInsights.length }}</p>
        <p class="text-xs text-gray-500 dark:text-gray-400">分析建议</p>
      </div>
      <div class="card text-center py-3">
        <p class="text-2xl font-bold" :class="weakCount > 0 ? 'text-red-500' : 'text-green-500'">{{ weakCount }}</p>
        <p class="text-xs text-gray-500 dark:text-gray-400">薄弱科目</p>
      </div>
      <div class="card text-center py-3">
        <p class="text-2xl font-bold text-amber-500">{{ avgWeakness }}%</p>
        <p class="text-xs text-gray-500 dark:text-gray-400">平均薄弱度</p>
      </div>
      <div class="card text-center py-3">
        <p class="text-2xl font-bold text-purple-500">{{ timeDist.length }}</p>
        <p class="text-xs text-gray-500 dark:text-gray-400">活跃科目</p>
      </div>
    </div>

    <!-- 周目标达成概览 -->
    <div class="card">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
        <span class="mr-2">🎯</span>本周目标达成
      </h3>
      <div v-if="weeklyGoalOverview.length === 0" class="text-center py-4 text-sm text-gray-400">
        暂无数据，请先在科目管理页设置每周目标
      </div>
      <div v-else class="space-y-2">
        <div v-for="item in weeklyGoalOverview" :key="item.subject_id"
          class="flex items-center gap-3">
          <div class="w-2.5 h-2.5 rounded-full flex-shrink-0" :style="{ backgroundColor: item.color }"></div>
          <span class="text-sm text-gray-700 dark:text-gray-300 w-16 truncate flex-shrink-0">{{ item.name }}</span>
          <div class="flex-1 h-2 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden min-w-0">
            <div class="h-full rounded-full transition-all" :style="{ width: item.percent + '%', backgroundColor: item.color }"></div>
          </div>
          <span class="text-xs font-medium flex-shrink-0 text-right"
            :class="item.percent >= 80 ? 'text-green-500' : item.percent >= 40 ? 'text-amber-500' : 'text-red-500'"
            :title="item.detail">
            {{ item.percent }}%
          </span>
        </div>
      </div>
    </div>

    <!-- 薄弱科目 -->
    <div class="card">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
        <span class="mr-2">⚠️</span>薄弱科目分析
      </h3>
      <div v-if="weakSubjects.length === 0" class="text-center py-8">
        <span class="text-3xl">🎉</span>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">暂无分析数据，点击「运行分析」开始</p>
      </div>
      <div v-else class="space-y-3">
        <div v-for="alert in weakSubjects" :key="alert.subject_id"
          class="p-4 rounded-xl border" :class="severityBorder(alert.score)">
          <div class="flex items-center justify-between mb-2">
            <div class="flex items-center gap-2">
              <span class="w-3 h-3 rounded-full" :style="{ backgroundColor: alert.color }"></span>
              <h4 class="font-semibold text-gray-900 dark:text-white">{{ alert.subject_name }}</h4>
              <span class="text-xs px-2 py-0.5 rounded-full font-medium" :class="severityBadge(alert.score)">
                {{ weaknessLabel(alert.score) }}
              </span>
            </div>
            <span class="text-lg font-bold" :class="severityText(alert.score)">
              {{ Math.round(alert.score * 100) }}%
            </span>
          </div>

          <!-- 因子分解 -->
          <div class="space-y-1 mt-2">
            <div v-for="factor in alert.factors" :key="factor.name"
              class="flex items-center gap-2 text-xs">
              <span class="w-20 text-gray-500 dark:text-gray-400 shrink-0">{{ factor.name }}</span>
              <div class="flex-1 h-1.5 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
                <div class="h-full rounded-full" :style="{ width: factor.value * 100 + '%', backgroundColor: factorColor(factor.value) }"></div>
              </div>
              <span class="w-10 text-right text-gray-400">×{{ factor.weight }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 图表区 -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- 时间分布 -->
      <div class="card">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          <span class="mr-2">⏱️</span>近 30 天时间分布
        </h3>
        <div v-if="timeDist.length === 0" class="text-center py-12">
          <p class="text-sm text-gray-500 dark:text-gray-400">暂无学习记录</p>
        </div>
        <div v-else class="h-64">
          <Doughnut :data="timeDistChartData" :options="doughnutOptions" />
        </div>
      </div>

      <!-- 效率趋势 -->
      <div class="card">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          <span class="mr-2">📈</span>学习效率趋势
        </h3>
        <div v-if="trend.length === 0" class="text-center py-12">
          <p class="text-sm text-gray-500 dark:text-gray-400">暂无趋势数据</p>
        </div>
        <div v-else class="h-64">
          <Line :data="trendChartData" :options="lineOptions" />
        </div>
      </div>
    </div>

    <!-- AI 深度分析结果 -->
    <div v-if="aiAdvice" class="card border-l-4 border-l-purple-500">
      <div class="flex items-center justify-between mb-3">
        <h3 class="text-lg font-semibold text-purple-600">
          <span class="mr-2">🤖</span>AI 深度分析
        </h3>
        <button @click="aiAdvice = ''" class="text-xs text-gray-400 hover:text-gray-600">关闭</button>
      </div>
      <div class="prose prose-sm dark:prose-invert max-w-none text-gray-700 dark:text-gray-300 whitespace-pre-line">{{ aiAdvice }}</div>
    </div>

    <!-- 智能建议 -->
    <div class="card">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
          <span class="mr-2">💡</span>智能建议
        </h3>
        <button v-if="suggestionInsights.length > 0" @click="doMarkAllRead" class="text-xs text-primary-500 hover:text-primary-600">
          全部已读
        </button>
      </div>
      <div v-if="suggestionInsights.length === 0" class="text-center py-8">
        <span class="text-3xl">💡</span>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">运行分析后会生成智能建议</p>
      </div>
      <div v-else class="space-y-2">
        <div v-for="insight in suggestionInsights" :key="insight.id"
          class="flex items-start gap-3 p-3 rounded-lg transition-colors"
          :class="insight.is_read ? 'bg-gray-50 dark:bg-gray-800/50' : 'bg-primary-50 dark:bg-primary-900/20 border-l-4 border-l-primary-500'">
          <span class="text-lg mt-0.5">{{ severityIcon(insight.severity) }}</span>
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
              <h4 class="font-medium text-gray-900 dark:text-white text-sm">{{ insight.title }}</h4>
              <span v-if="insight.is_read === 0" class="w-2 h-2 rounded-full bg-primary-500 flex-shrink-0"></span>
            </div>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">{{ insight.content }}</p>
            <p class="text-xs text-gray-400 mt-1">{{ formatDate(insight.generated_at) }}</p>
          </div>
          <button v-if="insight.is_read === 0"
            @click="doMarkRead(insight.id)"
            class="text-xs text-gray-400 hover:text-primary-500 shrink-0 mt-1">
            已读
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { Doughnut, Line } from 'vue-chartjs';
import {
  Chart as ChartJS, ArcElement, Tooltip, Legend,
  CategoryScale, LinearScale, PointElement, LineElement, Filler,
} from 'chart.js';
import {
  runFullAnalysis, getTimeDistribution, getEfficiencyTrend,
  getRecentInsights, markInsightRead, markAllInsightsRead,
  generateSuggestions, getReviewStats, getWeeklyGoalProgress,
} from '../composables/api';
import { requestLlmAdvice } from '../composables/llm';
import type { WeakSubjectAlert, SubjectDistribution, WeeklyTrend, Insight, WeeklyGoalProgress } from '../types';

ChartJS.register(ArcElement, Tooltip, Legend, CategoryScale, LinearScale, PointElement, LineElement, Filler);

const analyzing = ref(false);
const aiBusy = ref(false);
const aiAdvice = ref('');
const weakSubjects = ref<WeakSubjectAlert[]>([]);
const timeDist = ref<SubjectDistribution[]>([]);
const trend = ref<WeeklyTrend[]>([]);
const insights = ref<Insight[]>([]);
const weeklyProgress = ref<WeeklyGoalProgress[]>([]);

// 周目标达成概览
const weeklyGoalOverview = computed(() => {
  return weeklyProgress.value.map(p => ({
    subject_id: p.subject_id,
    name: p.subject_name,
    color: p.color,
    actual: p.weekly_hours,
    goal: p.goal_hours,
    percent: Math.round(p.composite_rate * 100),
    detail: p.goal_kps > 0
      ? `⏱${p.weekly_hours.toFixed(1)}/${p.goal_hours}h · 📖+${p.new_kps}/${p.goal_kps}`
      : `⏱${p.weekly_hours.toFixed(1)}/${p.goal_hours}h`,
  }));
});

const weakCount = computed(() => weakSubjects.value.filter(w => w.score > 0.4).length);
const avgWeakness = computed(() => {
  if (!weakSubjects.value.length) return 0;
  return Math.round(weakSubjects.value.reduce((s, w) => s + w.score, 0) / weakSubjects.value.length * 100);
});

// 过滤掉里程碑（已在进度页展示），只保留建议和提醒
const suggestionInsights = computed(() =>
  insights.value.filter(i => i.insight_type !== 'milestone'),
);

// 时间分布图数据
const timeDistChartData = computed(() => ({
  labels: timeDist.value.map(d => d.subject_name),
  datasets: [{
    data: timeDist.value.map(d => d.total_hours || 0.01), // 避免 0 值
    backgroundColor: timeDist.value.map(d => d.color || '#0ea5e9'),
    borderWidth: 0,
  }],
}));

const doughnutOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { position: 'right' as const, labels: { padding: 12, usePointStyle: true, boxWidth: 8 } },
  },
};

// 效率趋势图数据
const trendChartData = computed(() => ({
  labels: trend.value.map(t => {
    const d = new Date(t.date * 1000);
    return `${d.getMonth() + 1}/${d.getDate()}`;
  }),
  datasets: [{
    label: '学习时长（小时）',
    data: trend.value.map(t => t.hours),
    borderColor: '#0ea5e9',
    backgroundColor: 'rgba(14, 165, 233, 0.1)',
    fill: true,
    tension: 0.3,
    pointRadius: 2,
    pointHoverRadius: 4,
  }],
}));

const lineOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: false },
  },
  scales: {
    x: { grid: { display: false }, ticks: { maxTicksLimit: 8, font: { size: 10 } } },
    y: { beginAtZero: true, ticks: { font: { size: 10 } } },
  },
};

function severityBorder(score: number) {
  if (score >= 0.7) return 'border-red-300 dark:border-red-800';
  if (score >= 0.4) return 'border-amber-300 dark:border-amber-800';
  return 'border-gray-200 dark:border-gray-700';
}

function severityBadge(score: number) {
  if (score >= 0.7) return 'bg-red-100 text-red-700';
  if (score >= 0.4) return 'bg-amber-100 text-amber-700';
  return 'bg-green-100 text-green-700';
}

function severityText(score: number) {
  if (score >= 0.7) return 'text-red-500';
  if (score >= 0.4) return 'text-amber-500';
  return 'text-green-500';
}

function weaknessLabel(score: number) {
  if (score >= 0.7) return '需重点关注';
  if (score >= 0.4) return '需要加强';
  return '状态良好';
}

function factorColor(value: number) {
  if (value >= 0.7) return '#ef4444';
  if (value >= 0.4) return '#f59e0b';
  return '#22c55e';
}

function severityIcon(s: string) {
  return { warning: '⚠️', critical: '🚨', info: 'ℹ️', success: '✅' }[s] || '💡';
}

function formatDate(ts: number) {
  return new Date(ts * 1000).toLocaleDateString('zh-CN', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
}

async function loadData() {
  try {
    const results = await Promise.allSettled([
      getTimeDistribution(30),
      getEfficiencyTrend(8),
      getRecentInsights(20),
      getWeeklyGoalProgress(),
    ]);
    if (results[0].status === 'fulfilled') timeDist.value = results[0].value;
    if (results[1].status === 'fulfilled') trend.value = results[1].value;
    if (results[2].status === 'fulfilled') insights.value = results[2].value;
    if (results[3].status === 'fulfilled') weeklyProgress.value = results[3].value;
  } catch {
    // 数据未初始化
  }
}

async function doAnalysis() {
  analyzing.value = true;
  try {
    weakSubjects.value = await runFullAnalysis();
    // 刷新关联数据
    const results = await Promise.allSettled([
      getTimeDistribution(30),
      getEfficiencyTrend(8),
      getRecentInsights(20),
      getWeeklyGoalProgress(),
    ]);
    if (results[0].status === 'fulfilled') timeDist.value = results[0].value;
    if (results[1].status === 'fulfilled') trend.value = results[1].value;
    if (results[2].status === 'fulfilled') insights.value = results[2].value;
    if (results[3].status === 'fulfilled') weeklyProgress.value = results[3].value;
  } catch {
    // 分析失败
  } finally {
    analyzing.value = false;
  }
}

async function doMarkRead(id: string) {
  await markInsightRead(id);
  insights.value = insights.value.map(i => i.id === id ? { ...i, is_read: 1 } : i);
}

async function doGenerateSuggestions() {
  analyzing.value = true;
  try {
    await generateSuggestions();
    insights.value = await getRecentInsights(20);
  } catch {
    // 生成失败
  } finally {
    analyzing.value = false;
  }
}

async function doAiAdvice() {
  aiBusy.value = true;
  aiAdvice.value = '';
  try {
    const reviewStats = await getReviewStats();
    aiAdvice.value = await requestLlmAdvice(
      weakSubjects.value, timeDist.value, trend.value, reviewStats,
    );
  } catch (e: any) {
    const msg = e instanceof Error ? e.message : String(e ?? '');
    aiAdvice.value = `❌ ${msg || 'AI 分析失败，请检查 API Key 配置和网络连接。'}`;
  } finally {
    aiBusy.value = false;
  }
}

async function doMarkAllRead() {
  await markAllInsightsRead();
  insights.value = insights.value.map(i => ({ ...i, is_read: 1 }));
}

onMounted(() => loadData());
</script>
