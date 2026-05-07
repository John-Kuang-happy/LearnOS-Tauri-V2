<template>
  <div class="space-y-6">
    <div>
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white">执行记录</h2>
      <p class="text-gray-500 dark:text-gray-400 mt-1">查看过往学习执行历史</p>
    </div>

    <div v-if="executions.length === 0" class="card text-center py-16">
      <span class="text-6xl">⏱️</span>
      <h3 class="text-xl font-semibold text-gray-900 dark:text-white mt-4">暂无记录</h3>
      <p class="text-gray-500 dark:text-gray-400 mt-2">在首页启动番茄钟后，记录会自动显示在这里。</p>
    </div>

    <div v-else class="space-y-4">
      <div v-for="group in groupedExecs" :key="group.label"
        class="card">
        <h3 class="text-sm font-semibold text-gray-500 dark:text-gray-400 mb-3">{{ group.label }}</h3>
        <div class="space-y-2">
          <div v-for="e in group.items" :key="e.id"
            class="flex items-center gap-3 p-2 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800/50">
            <span class="text-xs w-10 text-gray-400">{{ formatTime(e.start_time) }}</span>
            <span class="flex-1 text-sm text-gray-900 dark:text-white truncate">
              <router-link v-if="e.plan_id" :to="`/plans`" class="hover:text-primary-500">
                {{ getPlanTitle(e.plan_id) }}
              </router-link>
              <span v-else>未关联计划</span>
            </span>
            <span class="text-xs text-gray-400">{{ (e.actual_hours || 0).toFixed(1) }}h</span>
            <span class="text-xs w-12 text-right"
              :class="(e.completion_rate || 0) >= 0.8 ? 'text-green-500' : (e.completion_rate || 0) >= 0.5 ? 'text-amber-500' : 'text-red-400'">
              {{ Math.round((e.completion_rate || 0) * 100) }}%
            </span>
            <span v-if="e.pomodoro_count" class="text-xs text-gray-400">{{ '🍅'.repeat(e.pomodoro_count) }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { getRecentExecutions, getAllPlans } from '../composables/api';
import type { Execution, Plan } from '../types';

const executions = ref<Execution[]>([]);
const allPlans = ref<Plan[]>([]);

const groupedExecs = computed(() => {
  const groups: Record<string, { label: string; items: Execution[] }> = {};
  const now = new Date();
  const todayStr = now.toDateString();
  const weekAgo = now.getTime() - 7 * 86400000;

  for (const e of executions.value) {
    if (!e.end_time) continue; // 跳过未完成的
    const d = new Date(e.start_time * 1000);
    let key: string;
    if (d.toDateString() === todayStr) {
      key = 'today';
    } else if (d.getTime() >= weekAgo) {
      key = 'week';
    } else {
      key = d.toLocaleDateString('zh-CN', { month: 'long', day: 'numeric' });
    }
    const label = key === 'today' ? '今天' : key === 'week' ? '近 7 天' : key;
    if (!groups[key]) groups[key] = { label, items: [] };
    groups[key].items.push(e);
  }
  return Object.values(groups);
});

function getPlanTitle(id: string) {
  return allPlans.value.find(p => p.id === id)?.title || '未命名';
}

function formatTime(ts: number) {
  return new Date(ts * 1000).toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
}

onMounted(async () => {
  try {
    const monthAgo = Math.floor(Date.now() / 1000) - 30 * 86400;
    [executions.value, allPlans.value] = await Promise.all([
      getRecentExecutions(monthAgo),
      getAllPlans(),
    ]);
  } catch (e) { console.error('加载执行记录失败:', e); }
});
</script>
