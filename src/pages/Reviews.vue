<template>
  <div class="space-y-6">
    <div>
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white">复盘记录</h2>
      <p class="text-gray-500 dark:text-gray-400 mt-1">番茄钟结束后自动生成轻量复盘，也可在这里查看历史。</p>
    </div>

    <div v-if="reviews.length === 0" class="card text-center py-16">
      <span class="text-6xl">📝</span>
      <h3 class="text-xl font-semibold text-gray-900 dark:text-white mt-4">暂无复盘</h3>
      <p class="text-gray-500 dark:text-gray-400 mt-2">完成番茄钟后复盘会自动生成。</p>
    </div>

    <div v-else class="space-y-3">
      <div v-for="r in reviews" :key="r.id"
        class="card hover:shadow-sm transition-shadow">
        <div class="flex items-start justify-between mb-3">
          <div>
            <span class="font-medium text-gray-900 dark:text-white">
              {{ getPlanTitle(r.plan_id) }}
            </span>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">
              {{ formatDate(r.review_date) }}
            </p>
          </div>
          <div class="flex items-center gap-1 text-sm">
            <span :title="'心情: ' + r.mood_score">{{ moodEmoji(r.mood_score) }}</span>
            <span :title="'精力: ' + r.energy_level">{{ energyEmoji(r.energy_level) }}</span>
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-3 text-sm">
          <div v-if="r.what_went_well">
            <p class="text-xs font-medium text-green-600 mb-1">✅ 做得好</p>
            <p class="text-gray-600 dark:text-gray-400">{{ r.what_went_well }}</p>
          </div>
          <div v-if="r.what_to_improve">
            <p class="text-xs font-medium text-amber-600 mb-1">🔧 需改进</p>
            <p class="text-gray-600 dark:text-gray-400">{{ r.what_to_improve }}</p>
          </div>
          <div v-if="r.action_items">
            <p class="text-xs font-medium text-primary-600 mb-1">📋 行动计划</p>
            <p class="text-gray-600 dark:text-gray-400">{{ r.action_items }}</p>
          </div>
        </div>

        <div v-if="!r.what_went_well && !r.what_to_improve && !r.action_items"
          class="text-xs text-gray-400 italic">
          轻量复盘（由系统自动生成）
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { getAllReviews, getAllPlans } from '../composables/api';
import type { Review, Plan } from '../types';

const reviews = ref<Review[]>([]);
const allPlans = ref<Plan[]>([]);

function getPlanTitle(id: string) {
  return allPlans.value.find(p => p.id === id)?.title || id.slice(0, 8);
}

function formatDate(ts: number) {
  return new Date(ts * 1000).toLocaleDateString('zh-CN', { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' });
}

function moodEmoji(score: number) {
  return ['', '😫', '😔', '😐', '😊', '😄'][score] || '😐';
}

function energyEmoji(level: number) {
  return ['', '🔋', '🔋', '⚡', '⚡', '⚡'][level] || '⚡';
}

onMounted(async () => {
  try {
    [reviews.value, allPlans.value] = await Promise.all([
      getAllReviews(),
      getAllPlans(),
    ]);
  } catch (e) { console.error('加载复盘数据失败:', e); }
});
</script>
