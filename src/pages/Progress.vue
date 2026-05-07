<template>
  <div class="space-y-5">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white">学习进度</h2>
        <p class="text-gray-500 dark:text-gray-400 text-sm mt-0.5">
          已掌握 {{ masteredCount }}/{{ kps.length }}
        </p>
      </div>
    </div>

    <!-- 科目掌握度进度条 -->
    <div class="card">
      <h3 class="font-semibold text-gray-900 dark:text-white mb-3">📊 科目掌握度</h3>
      <div v-if="subjectBars.length === 0" class="text-center py-4 text-sm text-gray-400">
        暂无知识点数据
      </div>
      <div v-else class="space-y-3">
        <div v-for="bar in subjectBars" :key="bar.subject_id" class="space-y-1">
          <!-- 累计掌握 -->
          <div class="flex items-center gap-3">
            <span class="text-sm text-gray-700 dark:text-gray-300 w-16 truncate">{{ bar.name }}</span>
            <div class="flex-1 h-2.5 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
              <div class="h-full rounded-full transition-all" :style="{ width: bar.percent + '%', backgroundColor: bar.color }"></div>
            </div>
            <span class="text-xs text-gray-400 w-16 text-right">{{ bar.mastered }}/{{ bar.total }}</span>
          </div>
          <!-- 本周 KP 达成 -->
          <div v-if="bar.weeklyGoal" class="flex items-center gap-3">
            <span class="text-[10px] text-gray-400 w-16 truncate text-right flex-shrink-0">本周</span>
            <div class="flex-1 h-1.5 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
              <div class="h-full rounded-full transition-all opacity-50" :style="{ width: bar.weeklyPct + '%', backgroundColor: bar.color }"></div>
            </div>
            <span class="text-[10px] w-16 text-right"
              :class="bar.weeklyPct >= 80 ? 'text-green-500' : bar.weeklyPct >= 40 ? 'text-amber-500' : 'text-red-500'">
              +{{ bar.newKps }}/{{ bar.weeklyGoal }}个
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- 知识点掌握情况（按科目分组） -->
    <div class="card">
      <div class="flex items-center justify-between mb-4">
        <h3 class="font-semibold text-gray-900 dark:text-white">📋 知识点掌握情况</h3>
        <div class="flex items-center gap-2">
          <select v-model="filterSubjectId" class="input w-auto text-xs py-1">
            <option value="">全部科目</option>
            <option v-for="s in subjects" :key="s.id" :value="s.id">{{ s.name }}</option>
          </select>
        </div>
      </div>

      <!-- 掌握度概览徽章 -->
      <div class="flex gap-2 mb-4 flex-wrap">
        <span class="text-xs px-2.5 py-1 rounded-full bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 font-medium">
          ✅ 已掌握 {{ filteredMasteredCount }}
        </span>
        <span class="text-xs px-2.5 py-1 rounded-full bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400 font-medium">
          📖 学习中 {{ filteredLearningCount }}
        </span>
        <span class="text-xs px-2.5 py-1 rounded-full bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400 font-medium">
          ⚠️ 待加强 {{ filteredWeakCount }}
        </span>
        <span class="text-xs px-2.5 py-1 rounded-full bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400 font-medium">
          🔄 待复习 {{ filteredDueKpCount }}
        </span>
      </div>

      <!-- 空状态 -->
      <div v-if="kps.length === 0" class="text-center py-6 text-sm text-gray-400">
        暂无知识点
      </div>

      <!-- 按科目分组列表 -->
      <div v-else class="space-y-3">
        <div v-for="group in groupedKps" :key="group.subject_id"
          class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden">

          <!-- 科目头部（可点击折叠） -->
          <div @click="toggleGroup(group.subject_id)"
            class="flex items-center gap-3 px-3 py-2.5 cursor-pointer select-none transition-colors hover:bg-gray-50 dark:hover:bg-gray-800/50">
            <span class="text-xs font-bold text-white px-2 py-0.5 rounded flex-shrink-0"
              :style="{ backgroundColor: group.color }">{{ group.name }}</span>
            <div class="flex-1 h-1.5 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
              <div class="h-full rounded-full" :style="{ width: group.masteryPercent + '%', backgroundColor: group.color }"></div>
            </div>
            <span class="text-xs text-gray-400 flex-shrink-0">{{ group.mastered }}/{{ group.list.length }}</span>
            <span class="text-xs text-gray-400 flex-shrink-0">{{ group.collapsed ? '▶' : '▼' }}</span>
          </div>

          <!-- 知识点列表 -->
          <div v-show="!group.collapsed" class="divide-y divide-gray-100 dark:divide-gray-700/50">
            <div v-for="kp in group.list" :key="kp.id"
              class="flex items-center gap-3 px-3 py-2.5 hover:bg-gray-50 dark:hover:bg-gray-800/30 transition-colors group/item">
              <!-- 掌握度小圆点 -->
              <span class="w-2 h-2 rounded-full flex-shrink-0" :style="{ backgroundColor: masteryColor(kp.mastery_level) }"></span>
              <!-- 名称 -->
              <span class="text-sm text-gray-900 dark:text-white truncate flex-1 min-w-0">{{ kp.name }}</span>
              <!-- 掌握度进度条 -->
              <div class="w-20 h-1.5 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden flex-shrink-0 hidden sm:block">
                <div class="h-full rounded-full transition-all" :style="{ width: kp.is_mastered ? '100%' : (kp.mastery_level * 100) + '%', backgroundColor: masteryColor(kp.is_mastered ? 1 : kp.mastery_level) }"></div>
              </div>
              <!-- 掌握度百分比 -->
              <span class="text-xs font-medium w-9 text-right flex-shrink-0"
                :style="{ color: masteryColor(kp.is_mastered ? 1 : kp.mastery_level) }">{{ kp.is_mastered ? '100' : Math.round(kp.mastery_level * 100) }}%</span>
              <!-- 复习状态 -->
              <span v-if="kp.is_mastered" class="text-[10px] px-1.5 py-0.5 rounded bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400 flex-shrink-0">已掌握</span>
              <span v-else-if="isOverdue(kp)" class="text-[10px] px-1.5 py-0.5 rounded bg-red-100 dark:bg-red-900/30 text-red-600 dark:text-red-400 flex-shrink-0">逾期</span>
              <span v-else-if="isDueSoon(kp)" class="text-[10px] px-1.5 py-0.5 rounded bg-amber-100 dark:bg-amber-900/30 text-amber-600 dark:text-amber-400 flex-shrink-0">即将复习</span>
              <!-- 删除 -->
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 近期里程碑 -->
    <div class="card">
      <div class="flex items-center justify-between mb-4">
        <h3 class="font-semibold text-gray-900 dark:text-white">🏅 近期里程碑</h3>
        <span class="text-xs text-gray-400">{{ milestones.length }} 项</span>
      </div>

      <div v-if="milestones.length === 0" class="text-center py-6">
        <span class="text-3xl">🚀</span>
        <p class="text-sm text-gray-400 mt-2">暂无里程碑，开始学习吧！</p>
      </div>

      <!-- 时间线 -->
      <div v-else class="relative">
        <div class="absolute left-[15px] top-2 bottom-2 w-0.5 bg-gray-200 dark:bg-gray-600"></div>

        <div v-for="m in milestones" :key="m.id" class="relative pl-10 pb-5 last:pb-0">
          <div class="absolute left-[9px] w-3.5 h-3.5 rounded-full border-2 border-white dark:border-gray-800 shadow-sm z-10"
            :class="severityDot(m.severity)"></div>

          <div class="p-3 rounded-lg border border-gray-200 dark:border-gray-700 hover:shadow-sm transition-shadow">
            <div class="flex items-start justify-between gap-2">
              <p class="text-sm font-medium text-gray-900 dark:text-white">{{ m.title }}</p>
              <span class="text-[10px] text-gray-400 whitespace-nowrap flex-shrink-0">{{ relativeDate(m.generated_at) }}</span>
            </div>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">{{ m.content }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import {
  getAllKnowledgePoints, getAllSubjects,
  getMilestones, getWeeklyGoalProgress,
} from '../composables/api';
import type { Insight, KnowledgePoint, Subject, WeeklyGoalProgress } from '../types';

const kps = ref<KnowledgePoint[]>([]);
const subjects = ref<Subject[]>([]);
const weeklyProgress = ref<WeeklyGoalProgress[]>([]);
const filterSubjectId = ref('');
const milestones = ref<Insight[]>([]);
const collapsedGroups = ref<Set<string>>(new Set());

const now = () => Math.floor(Date.now() / 1000);

// 统计
const masteredCount = computed(() => kps.value.filter(k => k.is_mastered).length);

// 筛选后的统计（随科目下拉框联动）
const filteredMasteredCount = computed(() => filteredKps.value.filter(k => k.is_mastered).length);
const filteredLearningCount = computed(() => filteredKps.value.filter(k => !k.is_mastered && k.mastery_level >= 0.3).length);
const filteredWeakCount = computed(() => filteredKps.value.filter(k => !k.is_mastered && k.mastery_level < 0.3).length);
const filteredDueKpCount = computed(() => filteredKps.value.filter(k => !k.is_mastered && k.next_review_at && k.next_review_at <= now()).length);

// 筛选
const filteredKps = computed(() => {
  if (!filterSubjectId.value) return kps.value;
  return kps.value.filter(k => k.subject_id === filterSubjectId.value);
});

// 按科目分组（同名知识点去重，保留掌握度最高的）
const groupedKps = computed(() => {
  const map: Record<string, { subject_id: string; name: string; color: string; list: KnowledgePoint[]; mastered: number; total: number; masteryPercent: number; collapsed: boolean }> = {};
  for (const kp of filteredKps.value) {
    if (!map[kp.subject_id]) {
      const s = subjects.value.find(s => s.id === kp.subject_id);
      map[kp.subject_id] = {
        subject_id: kp.subject_id,
        name: s?.name || '未归类',
        color: s?.color || '#94a3b8',
        list: [],
        mastered: 0,
        total: 0,
        masteryPercent: 0,
        collapsed: collapsedGroups.value.has(kp.subject_id),
      };
    }
    const group = map[kp.subject_id];
    // 同名去重：如果已存在同名知识点，保留掌握度更高的那个
    const dupIdx = group.list.findIndex(existing => existing.name === kp.name);
    if (dupIdx >= 0) {
      if (kp.mastery_level > group.list[dupIdx].mastery_level) {
        // 替换时同步纠正 mastered 计数
        if (group.list[dupIdx].is_mastered && !kp.is_mastered) group.mastered--;
        else if (!group.list[dupIdx].is_mastered && kp.is_mastered) group.mastered++;
        group.list[dupIdx] = kp;
      }
      continue;
    }
    group.list.push(kp);
    group.total++;
    if (kp.is_mastered) group.mastered++;
  }
  return Object.values(map).map(g => ({
    ...g,
    masteryPercent: g.total > 0 ? Math.round(g.mastered / g.total * 100) : 0,
  }));
});

// 科目掌握度进度条（含本周KP达成对比）
const subjectBars = computed(() => {
  const map: Record<string, { subject_id: string; name: string; color: string; total: number; mastered: number; weeklyGoal: number; newKps: number; weeklyPct: number }> = {};
  for (const kp of kps.value) {
    if (!map[kp.subject_id]) {
      const s = subjects.value.find(s => s.id === kp.subject_id);
      const wp = weeklyProgress.value.find(w => w.subject_id === kp.subject_id);
      map[kp.subject_id] = {
        subject_id: kp.subject_id,
        name: s?.name || kp.subject_id,
        color: s?.color || '#94a3b8',
        total: 0, mastered: 0,
        weeklyGoal: wp?.goal_kps || 0,
        newKps: wp?.new_kps || 0,
        weeklyPct: (wp && wp.goal_kps > 0) ? Math.round(Math.min(wp.new_kps / wp.goal_kps, 1) * 100) : 0,
      };
    }
    map[kp.subject_id].total++;
    if (kp.is_mastered) map[kp.subject_id].mastered++;
  }
  return Object.values(map).map(b => ({
    ...b,
    percent: b.total > 0 ? Math.round(b.mastered / b.total * 100) : 0,
  }));
});

function toggleGroup(id: string) {
  if (collapsedGroups.value.has(id)) {
    collapsedGroups.value.delete(id);
  } else {
    collapsedGroups.value.add(id);
  }
  // trigger reactivity
  collapsedGroups.value = new Set(collapsedGroups.value);
}

function masteryColor(level: number) {
  if (level >= 0.8) return '#22c55e';
  if (level >= 0.5) return '#f59e0b';
  return '#ef4444';
}

function isOverdue(kp: KnowledgePoint): boolean {
  return !kp.is_mastered && !!kp.next_review_at && kp.next_review_at <= now();
}

function isDueSoon(kp: KnowledgePoint): boolean {
  return !kp.is_mastered && !!kp.next_review_at && kp.next_review_at > now() && kp.next_review_at <= now() + 86400;
}

function severityDot(severity: string) {
  return severity === 'success' ? 'bg-green-400' : 'bg-primary-400';
}

function relativeDate(ts: number) {
  const diff = Date.now() / 1000 - ts;
  if (diff < 60) return '刚刚';
  if (diff < 3600) return `${Math.floor(diff / 60)} 分钟前`;
  if (diff < 86400) return `${Math.floor(diff / 3600)} 小时前`;
  const days = Math.floor(diff / 86400);
  if (days === 1) return '昨天';
  if (days < 7) return `${days} 天前`;
  if (days < 30) return `${Math.floor(days / 7)} 周前`;
  return new Date(ts * 1000).toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' });
}

async function loadData() {
  try {
    const results = await Promise.allSettled([
      getAllKnowledgePoints(),
      getAllSubjects(),
      getMilestones(20),
      getWeeklyGoalProgress(),
    ]);
    if (results[0].status === 'fulfilled') kps.value = results[0].value;
    if (results[1].status === 'fulfilled') subjects.value = results[1].value;
    if (results[2].status === 'fulfilled') milestones.value = results[2].value;
    if (results[3].status === 'fulfilled') weeklyProgress.value = results[3].value;
  } catch (e) { console.error('加载进度数据失败:', e); }
}

onMounted(() => loadData());
</script>
