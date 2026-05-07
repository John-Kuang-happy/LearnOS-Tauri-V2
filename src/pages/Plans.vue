<template>
  <div class="space-y-5">
    <!-- 页头：标题 + 视图切换 + 新建 -->
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white">学习计划</h2>
        <p class="text-gray-500 dark:text-gray-400 text-sm mt-0.5">规划每周学习，高效利用时间</p>
      </div>
      <div class="flex items-center gap-2">
        <div class="flex bg-gray-100 dark:bg-gray-700 rounded-lg p-0.5">
          <button @click="viewMode = 'month'"
            class="px-3 py-1.5 text-xs font-medium rounded-md transition-colors"
            :class="viewMode === 'month' ? 'bg-white dark:bg-dark-surface text-primary-600 shadow-sm' : 'text-gray-500 dark:text-gray-400'">
            📆 月视图
          </button>
          <button @click="viewMode = 'week'"
            class="px-3 py-1.5 text-xs font-medium rounded-md transition-colors"
            :class="viewMode === 'week' ? 'bg-white dark:bg-dark-surface text-primary-600 shadow-sm' : 'text-gray-500 dark:text-gray-400'">
            📅 周视图
          </button>
          <button @click="viewMode = 'list'"
            class="px-3 py-1.5 text-xs font-medium rounded-md transition-colors"
            :class="viewMode === 'list' ? 'bg-white dark:bg-dark-surface text-primary-600 shadow-sm' : 'text-gray-500 dark:text-gray-400'">
            📋 列表
          </button>
        </div>
        <button @click="openCreate()" class="btn-primary text-sm">+ 新建计划</button>
      </div>
    </div>

    <!-- ======================== 周视图 ======================== -->
    <template v-if="viewMode === 'week'">
      <!-- 周选择器 -->
      <div class="flex items-center justify-center gap-4">
        <button @click="prevWeek" class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors">
          <span class="text-lg">←</span>
        </button>
        <div class="text-center">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            {{ weekLabel }}
          </h3>
          <p class="text-xs text-gray-500 dark:text-gray-400">{{ weekRange }}</p>
        </div>
        <button @click="nextWeek" class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors">
          <span class="text-lg">→</span>
        </button>
        <button @click="goToday" v-if="!isCurrentWeek" class="text-xs text-primary-500 hover:text-primary-600 ml-2">
          回到本周
        </button>
      </div>

      <!-- 7 列网格 -->
      <div class="grid grid-cols-7 gap-3">
        <div v-for="(day, idx) in weekDays" :key="idx"
          @click="!day.isBeforeToday && openCreate(day.date)"
          class="rounded-xl border min-h-48 p-2 transition-colors group"
          :class="[day.isBeforeToday
            ? 'border-gray-100 dark:border-gray-800 bg-gray-50/50 dark:bg-gray-800/30 opacity-50 cursor-default'
            : day.isToday
              ? 'border-primary-300 dark:border-primary-700 bg-primary-50/50 dark:bg-primary-900/10 cursor-pointer'
              : 'border-gray-200 dark:border-gray-700 bg-white dark:bg-dark-surface hover:border-primary-200 dark:hover:border-primary-800 cursor-pointer']">
          <!-- 日期头 -->
          <div class="text-center mb-2">
            <p class="text-xs font-medium"
              :class="day.isToday ? 'text-primary-600 dark:text-primary-400' : 'text-gray-500 dark:text-gray-400'">
              {{ day.dayOfWeek }}
            </p>
            <p class="text-lg font-bold"
              :class="day.isToday ? 'text-primary-600 dark:text-primary-400' : 'text-gray-900 dark:text-white'">
              {{ day.dayNum }}
            </p>
            <p class="text-xs" :class="day.isToday ? 'text-primary-500' : 'text-gray-400'">
              {{ day.monthLabel }}
              <span v-if="dayTotalHours(day) > 0" class="text-gray-300 dark:text-gray-600">
                · {{ dayTotalHours(day).toFixed(1) }}h
              </span>
            </p>
          </div>

          <!-- 当日计划卡片 -->
          <div class="space-y-1">
            <div v-for="plan in day.plans" :key="plan.id"
              @click.stop="openEdit(plan)"
              class="text-xs px-2 py-1.5 rounded-lg border-l-[3px] cursor-pointer hover:shadow-sm transition-shadow truncate"
              :style="{ borderLeftColor: getSubjectColor(plan.subject_id), backgroundColor: getSubjectColor(plan.subject_id) + '10' }"
              :title="plan.title">
              <span class="font-medium text-gray-900 dark:text-white">{{ plan.title }}</span>
              <span class="text-gray-400 ml-1">{{ plan.estimated_hours }}h</span>
            </div>
          </div>

          <!-- 空状态：点击添加（仅今天及以后） -->
          <div v-if="day.plans.length === 0 && !day.isBeforeToday"
            class="flex items-center justify-center h-16 opacity-0 group-hover:opacity-100 transition-opacity">
            <span class="text-2xl text-gray-300 dark:text-gray-600">+</span>
          </div>
        </div>
      </div>

    </template>

    <!-- ======================== 月视图 ======================== -->
    <template v-if="viewMode === 'month'">
      <!-- 月选择器 -->
      <div class="flex items-center justify-center gap-4">
        <button @click="prevMonth" class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors">
          <span class="text-lg">←</span>
        </button>
        <div class="text-center">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">{{ monthLabel }}</h3>
        </div>
        <button @click="nextMonth" class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors">
          <span class="text-lg">→</span>
        </button>
        <button @click="goTodayMonth" v-if="!isCurrentMonth" class="text-xs text-primary-500 hover:text-primary-600 ml-2">回到本月</button>
      </div>

      <!-- 星期头 -->
      <div class="grid grid-cols-7 gap-1 text-center text-xs font-medium text-gray-400 dark:text-gray-500 mb-1">
        <div>一</div><div>二</div><div>三</div><div>四</div><div>五</div><div>六</div><div>日</div>
      </div>

      <!-- 月日历网格 -->
      <div class="grid grid-cols-7 gap-1">
        <div v-for="(day, idx) in monthDays" :key="idx"
          @click="day.date && !day.isBeforeToday && openCreate(day.date)"
          class="rounded-lg border min-h-28 p-1.5 transition-colors group"
          :class="[day.isBeforeToday
            ? 'border-gray-100 dark:border-gray-800 bg-gray-50/50 dark:bg-gray-800/30 opacity-50 cursor-default'
            : day.isToday
              ? 'border-primary-300 dark:border-primary-700 bg-primary-50/50 dark:bg-primary-900/10 cursor-pointer'
              : day.date
                ? 'border-gray-200 dark:border-gray-700 bg-white dark:bg-dark-surface hover:border-primary-200 dark:hover:border-primary-800 cursor-pointer'
                : 'border-transparent bg-transparent']">
          <p class="text-xs font-medium mb-1"
            :class="day.isToday ? 'text-primary-600 dark:text-primary-400' : day.date ? 'text-gray-900 dark:text-white' : 'text-gray-300 dark:text-gray-600'">
            {{ day.dayNum || '' }}
            <span v-if="day.plans.length > 0" class="text-[10px] font-normal text-gray-400">
              {{ monthDayHours(day) }}h
            </span>
          </p>
          <div class="space-y-0.5">
            <div v-for="plan in day.plans" :key="plan.id"
              @click.stop="openEdit(plan)"
              class="text-xs px-1 py-0.5 rounded truncate leading-tight"
              :style="{ backgroundColor: getSubjectColor(plan.subject_id) + '20', color: getSubjectColor(plan.subject_id) }">
              {{ plan.title }}
            </div>
          </div>
          <!-- 空状态提示（仅今天及以后） -->
          <div v-if="day.date && !day.isBeforeToday && day.plans.length === 0"
            class="flex items-center justify-center h-10 opacity-0 group-hover:opacity-100 transition-opacity">
            <span class="text-lg text-gray-300 dark:text-gray-600">+</span>
          </div>
        </div>
      </div>
    </template>

    <!-- ======================== 列表视图 ======================== -->
    <template v-if="viewMode === 'list'">
      <!-- 筛选栏（紧凑内联） -->
      <div class="flex flex-wrap items-center gap-2">
        <select v-model="filter.subject_id" @change="loadPlans" class="input w-auto text-sm py-1.5">
          <option value="">全部科目</option>
          <option v-for="s in subjects" :key="s.id" :value="s.id">{{ s.icon }} {{ s.name }}</option>
        </select>
        <select v-model="filter.status" @change="loadPlans" class="input w-auto text-sm py-1.5">
          <option value="">全部状态</option>
          <option value="pending">待开始</option>
          <option value="in_progress">进行中</option>
          <option value="completed">已完成</option>
        </select>
      </div>

      <div v-if="plans.length === 0" class="card text-center py-16">
        <span class="text-5xl">📋</span>
        <p class="text-gray-500 dark:text-gray-400 mt-3">暂无计划</p>
      </div>

      <!-- 按状态分组 -->
      <div v-for="group in groupedPlans" :key="group.status" class="space-y-2">
        <div class="flex items-center gap-2 mb-1">
          <span class="text-xs font-medium px-2 py-0.5 rounded-full" :class="group.badgeClass">{{ group.label }}</span>
          <span class="text-xs text-gray-400">{{ group.items.length }}</span>
        </div>
        <div v-for="plan in group.items" :key="plan.id"
          @click="openEdit(plan)"
          class="card p-4 hover:shadow-md transition-shadow cursor-pointer border-l-[3px]"
          :style="{ borderLeftColor: getSubjectColor(plan.subject_id) }">
          <div class="flex items-center justify-between">
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2">
                <h3 class="font-semibold text-gray-900 dark:text-white truncate">{{ plan.title }}</h3>
                <span class="text-xs px-1.5 py-0.5 rounded font-medium"
                  :style="{ backgroundColor: getSubjectColor(plan.subject_id) + '20', color: getSubjectColor(plan.subject_id) }">
                  {{ getSubjectName(plan.subject_id) }}
                </span>
              </div>
              <div class="flex items-center gap-3 mt-1 text-xs text-gray-400">
                <span>📅 {{ fmtDate(plan.start_date) }} ~ {{ fmtDate(plan.end_date) }}</span>
                <span>⏱️ {{ plan.estimated_hours }}h</span>
                <span v-if="plan.plan_type !== 'normal'" class="text-gray-400">· {{ typeLabel(plan.plan_type) }}</span>
              </div>
              <div v-if="plan.subject_id" class="flex items-center gap-2 mt-1 text-xs">
                <span class="text-gray-400">🎯</span>
                <span
                  :class="getGoalContributionPct(plan.subject_id) >= 80 ? 'text-amber-500' : 'text-gray-400'">
                  {{ getSubjectName(plan.subject_id) }}本周已排 {{ (weeklyPlannedHours[plan.subject_id] || 0).toFixed(1) }}/{{ getGoalHours(plan.subject_id) }}h
                  <span v-if="getGoalContributionPct(plan.subject_id) >= 80" class="ml-1">⚠️ 接近上限</span>
                </span>
              </div>
            </div>
            <button @click.stop="confirmDelete(plan)"
              class="ml-2 p-1.5 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 text-gray-400 hover:text-red-500 transition-colors text-sm">
              🗑️
            </button>
          </div>
        </div>
      </div>

    </template>

    <!-- ======================== 统一创建/编辑弹窗 ======================== -->
    <div v-if="showModal" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="absolute inset-0 bg-black/40" @click="closeModal"></div>
      <div class="relative bg-white dark:bg-dark-surface rounded-2xl shadow-xl p-5 w-full max-w-sm mx-4 max-h-[90vh] overflow-y-auto">
        <h3 class="font-semibold text-gray-900 dark:text-white mb-3">
          {{ editingPlan ? '编辑计划' : '新建计划' }}
        </h3>

        <!-- 科目目标感知（仅新建时显示） -->
        <div v-if="!editingPlan && selectedSubjectGoal" class="mb-3 p-2.5 rounded-xl bg-gray-50 dark:bg-gray-800 text-xs">
          <div class="flex items-center justify-between mb-1.5">
            <span class="text-gray-500 dark:text-gray-400">📐 {{ selectedSubjectGoal.name }} 本周目标</span>
          </div>
          <div class="space-y-1">
            <div class="flex items-center gap-1.5">
              <span class="text-gray-500 w-8">⏱️</span>
              <div class="flex-1 h-1.5 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
                <div class="h-full rounded-full transition-all" :style="{ width: selectedSubjectGoal.timePct + '%', backgroundColor: selectedSubjectGoal.color }"></div>
              </div>
              <span class="text-gray-500 w-16 text-right">{{ selectedSubjectGoal.weekly_hours.toFixed(1) }}/{{ selectedSubjectGoal.goal_hours }}h</span>
            </div>
            <div v-if="selectedSubjectGoal.goal_kps > 0" class="flex items-center gap-1.5">
              <span class="text-gray-500 w-8">📖</span>
              <div class="flex-1 h-1.5 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
                <div class="h-full rounded-full transition-all" :style="{ width: selectedSubjectGoal.kpPct + '%', backgroundColor: selectedSubjectGoal.color }"></div>
              </div>
              <span class="text-gray-500 w-16 text-right">+{{ selectedSubjectGoal.new_kps }}/{{ selectedSubjectGoal.goal_kps }}个</span>
            </div>
          </div>
        </div>

        <div class="space-y-3">
          <!-- 标题 -->
          <div>
            <input v-model="form.title" class="input" placeholder="计划标题（例如：完成数学第三章练习）" @keydown.enter="submitForm" />
          </div>

          <!-- 类型 -->
          <div>
            <label class="text-xs text-gray-500 dark:text-gray-400 mb-1 block">计划类型</label>
            <div class="flex gap-1.5 flex-wrap">
              <button v-for="pt in planTypeOptions" :key="pt.value"
                @click="form.plan_type = pt.value"
                class="px-2.5 py-1.5 text-xs rounded-lg border transition-colors"
                :class="form.plan_type === pt.value
                  ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20 text-primary-700 dark:text-primary-300'
                  : 'border-gray-200 dark:border-gray-600 text-gray-500 dark:text-gray-400 hover:border-gray-400'">
                {{ pt.icon }} {{ pt.label }}
              </button>
            </div>
          </div>

          <!-- 科目 + 优先级 -->
          <div class="grid grid-cols-2 gap-2">
            <div>
              <label class="text-xs text-gray-500 dark:text-gray-400 mb-1 block">科目</label>
              <select v-model="form.subject_id" class="input">
                <option value="">选择科目</option>
                <option v-for="s in subjects" :key="s.id" :value="s.id">{{ s.icon }} {{ s.name }}</option>
              </select>
            </div>
            <div>
              <label class="text-xs text-gray-500 dark:text-gray-400 mb-1 block">优先级</label>
              <select v-model.number="form.priority" class="input">
                <option :value="2">高</option>
                <option :value="3">中</option>
                <option :value="4">低</option>
              </select>
            </div>
          </div>

          <!-- 日期 -->
          <div class="grid grid-cols-2 gap-2">
            <div>
              <label class="text-xs text-gray-500 dark:text-gray-400 mb-1 block">开始日期</label>
              <input v-model="form.start_date" type="date" class="input" :min="todayStr" />
            </div>
            <div>
              <label class="text-xs text-gray-500 dark:text-gray-400 mb-1 block">结束日期</label>
              <input v-model="form.end_date" type="date" class="input" :min="todayStr" />
            </div>
          </div>

          <!-- 预计时长（步进器） -->
          <div>
            <label class="text-xs text-gray-500 dark:text-gray-400 mb-1 block">预计时长</label>
            <div class="flex items-center gap-2">
              <button @click="adjustHours(-0.1)"
                class="w-8 h-8 rounded-lg border border-gray-200 dark:border-gray-600 flex items-center justify-center text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors text-lg font-medium select-none">
                −
              </button>
              <div class="flex-1 text-center">
                <span class="text-lg font-bold text-gray-900 dark:text-white">{{ form.estimated_hours.toFixed(1) }}</span>
                <span class="text-sm text-gray-400 ml-1">小时</span>
              </div>
              <button @click="adjustHours(+0.1)"
                class="w-8 h-8 rounded-lg border border-gray-200 dark:border-gray-600 flex items-center justify-center text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors text-lg font-medium select-none">
                +
              </button>
            </div>
            <!-- 快捷时长 -->
            <div class="flex gap-1 mt-1.5">
              <button v-for="h in [0.5, 1, 1.5, 2, 3]" :key="h"
                @click="form.estimated_hours = h"
                class="px-2 py-0.5 text-xs rounded-md transition-colors"
                :class="Math.abs(form.estimated_hours - h) < 0.01 ? 'bg-primary-500 text-white' : 'bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 hover:bg-gray-200'">
                {{ h }}h
              </button>
            </div>
          </div>

          <!-- 知识点 -->
          <div>
            <label class="text-xs text-gray-500 dark:text-gray-400 mb-1 block">
              知识点
              <span v-if="!editingPlan" class="text-gray-400 font-normal">（选填，将关联到该科目下）</span>
            </label>
            <!-- 已添加列表 -->
            <div v-if="form.kpItems.length > 0" class="space-y-1 mb-2">
              <div v-for="(item, idx) in form.kpItems" :key="idx"
                class="flex items-center gap-1.5 px-2 py-1 rounded-lg bg-gray-50 dark:bg-gray-800 text-sm">
                <span class="flex-1 text-gray-900 dark:text-white truncate">{{ item }}</span>
                <button @click="removeKpItem(idx)" class="text-gray-400 hover:text-red-500 flex-shrink-0">✕</button>
              </div>
            </div>
            <!-- AI 建议标签 -->
            <div v-if="kpSuggestions.length > 0" class="flex flex-wrap gap-1 mb-2">
              <span class="text-[10px] text-gray-400">推荐：</span>
              <button v-for="s in kpSuggestions" :key="s"
                @click="addKpItem(s)"
                :disabled="form.kpItems.includes(s)"
                class="px-1.5 py-0.5 text-xs rounded border transition-colors"
                :class="form.kpItems.includes(s)
                  ? 'border-gray-200 dark:border-gray-600 text-gray-300 dark:text-gray-600'
                  : 'border-primary-200 dark:border-primary-800 text-primary-600 dark:text-primary-400 hover:bg-primary-50 dark:hover:bg-primary-900/20'">
                {{ s }}
              </button>
            </div>
            <!-- 输入框 -->
            <div class="flex gap-1.5">
              <input v-model="kpInput" @keydown.enter="addKpItem(kpInput)"
                class="flex-1 px-3 py-1.5 text-sm border border-gray-200 dark:border-gray-600 rounded-lg bg-white dark:bg-dark-card text-gray-900 dark:text-gray-100 outline-none"
                placeholder="输入知识点名称，回车添加" />
              <button @click="addKpItem(kpInput)" :disabled="!kpInput.trim()"
                class="px-3 py-1.5 text-xs rounded-lg bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 hover:bg-gray-200 disabled:opacity-50">
                添加
              </button>
            </div>
          </div>

        </div>

        <p v-if="formError" class="text-red-500 text-sm mb-2">{{ formError }}</p>
        <div class="flex items-center justify-between mt-4">
          <button v-if="editingPlan" @click="confirmDelete(editingPlan)" class="btn-ghost text-sm text-red-500 hover:text-red-600">删除</button>
          <div class="flex items-center gap-2 ml-auto">
            <button @click="closeModal" class="btn-ghost text-sm">取消</button>
            <button @click="submitForm" class="btn-primary text-sm">
              {{ editingPlan ? '保存' : '创建' }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 删除确认 -->
    <div v-if="showDeleteConfirm" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="absolute inset-0 bg-black/40" @click="showDeleteConfirm = false"></div>
      <div class="relative bg-white dark:bg-dark-surface rounded-2xl shadow-xl p-5 w-full max-w-xs mx-4">
        <h3 class="font-semibold text-gray-900 dark:text-white mb-2">确认删除</h3>
        <p class="text-sm text-gray-500 dark:text-gray-400">删除「{{ deletingPlan?.title }}」？</p>
        <div class="flex justify-end gap-2 mt-4">
          <button @click="showDeleteConfirm = false" class="btn-ghost text-sm">取消</button>
          <button @click="doDelete" class="btn-danger text-sm">删除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import {
  getAllPlans, createPlan, updatePlan, deletePlan, getAllSubjects,
  getWeeklyGoalProgress, suggestKnowledgePoints, createKpsBatch,
  getAllKnowledgePoints, deleteKnowledgePoint,
} from '../composables/api';
import type { Plan, Subject, WeeklyGoalProgress } from '../types';

const viewMode = ref<'week' | 'month' | 'list'>('week');
const plans = ref<Plan[]>([]);
const subjects = ref<Subject[]>([]);
const showModal = ref(false);
const showDeleteConfirm = ref(false);
const editingPlan = ref<Plan | null>(null);
const deletingPlan = ref<Plan | null>(null);
const weeklyProgress = ref<WeeklyGoalProgress[]>([]);
const filter = ref({ subject_id: '', status: '' });
const formError = ref('');
const d = new Date();
const todayStr = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;

// 周视图：当前周一偏移量
const weekOffset = ref(0);
// 月视图：当前月偏移量
const monthOffset = ref(0);

// 计划类型选项
const planTypeOptions = [
  { value: 'normal', label: '学习', icon: '' },
  { value: 'exam_paper', label: '做试卷', icon: '' },
  { value: 'review', label: '复习', icon: '' },
  { value: 'homework', label: '作业', icon: '' },
  { value: 'exam_prep', label: '备考', icon: '' },
];

const form = ref({
  title: '',
  subject_id: '',
  priority: 2,
  start_date: '',
  end_date: '',
  estimated_hours: 1,
  plan_type: 'normal' as string,
  kpItems: [] as string[],
});

const kpInput = ref('');
const kpSuggestions = ref<string[]>([]);
const pendingRemoveKps = ref<string[]>([]); // 编辑时待删除的 KP 名称
const originalKpNames = ref<string[]>([]);  // 编辑时原有的 KP 名称（不在其中即为新增）

// 选中科目的目标进度
const selectedSubjectGoal = computed(() => {
  if (!form.value.subject_id) return null;
  const p = weeklyProgress.value.find(w => w.subject_id === form.value.subject_id);
  if (!p) {
    const s = subjects.value.find(su => su.id === form.value.subject_id);
    return s ? {
      name: s.name,
      color: s.color,
      weekly_hours: 0,
      goal_hours: s.weekly_goal_hours,
      new_kps: 0,
      goal_kps: s.weekly_goal_kps,
      timePct: 0,
      kpPct: 0,
    } : null;
  }
  return {
    ...p,
    timePct: p.goal_hours > 0 ? Math.min(100, Math.round((p.weekly_hours / p.goal_hours) * 100)) : 0,
    kpPct: p.goal_kps > 0 ? Math.min(100, Math.round((p.new_kps / p.goal_kps) * 100)) : 0,
  };
});

// 本周各科目已排计划时长（pending + in_progress）
const weeklyPlannedHours = computed(() => {
  const now = Math.floor(Date.now() / 1000);
  const weekStart = now - (now % 86400) - ((new Date().getDay() + 6) % 7) * 86400; // 本周一0点
  const weekEnd = weekStart + 7 * 86400;
  const map: Record<string, number> = {};
  for (const plan of plans.value) {
    if (plan.status === 'completed') continue;
    // 计划与本周有交集
    if (plan.end_date >= weekStart && plan.start_date < weekEnd) {
      map[plan.subject_id] = (map[plan.subject_id] || 0) + plan.estimated_hours;
    }
  }
  return map;
});

// 获取某科目的目标小时
function getGoalHours(subjectId: string): number {
  const p = weeklyProgress.value.find(w => w.subject_id === subjectId);
  if (p) return p.goal_hours;
  return subjects.value.find(s => s.id === subjectId)?.weekly_goal_hours ?? 0;
}

// 获取某科目标达成率
function getGoalContributionPct(subjectId: string): number {
  const planned = weeklyPlannedHours.value[subjectId] || 0;
  const goal = getGoalHours(subjectId);
  if (goal <= 0) return 0;
  return Math.min(100, Math.round((planned / goal) * 100));
}

function dayTotalHours(day: WeekDay): number {
  return day.plans.reduce((sum, p) => sum + p.estimated_hours, 0);
}

function monthDayHours(day: MonthDay): number {
  return day.plans.reduce((sum, p) => sum + p.estimated_hours, 0);
}

// 科目变化时加载 AI 建议（带防抖，避免竞态）
let kpSuggestionSeq = 0;
let kpSuggestionTimer: ReturnType<typeof setTimeout> | null = null;
watch(() => form.value.subject_id, (newId) => {
  kpSuggestions.value = [];
  if (kpSuggestionTimer) clearTimeout(kpSuggestionTimer);
  if (newId && form.value.title.trim()) {
    const seq = ++kpSuggestionSeq;
    kpSuggestionTimer = setTimeout(async () => {
      try {
        const result = await suggestKnowledgePoints(form.value.title, newId);
        if (seq === kpSuggestionSeq) {
          kpSuggestions.value = result;
        }
      } catch { /* 静默忽略，AI 建议非必需 */ }
    }, 400);
  }
});

function adjustHours(delta: number) {
  const next = Math.round((form.value.estimated_hours + delta) * 10) / 10;
  if (next >= 0.1 && next <= 24) {
    form.value.estimated_hours = next;
  }
}

function addKpItem(name: string) {
  const trimmed = name.trim();
  if (trimmed && !form.value.kpItems.includes(trimmed)) {
    form.value.kpItems.push(trimmed);
  }
  kpInput.value = '';
}

function removeKpItem(idx: number) {
  const removed = form.value.kpItems[idx];
  form.value.kpItems.splice(idx, 1);
  // 编辑模式：记录待删除的 KP
  if (editingPlan.value) {
    pendingRemoveKps.value.push(removed);
  }
}

// ------ 周视图计算 ------

interface WeekDay {
  date: string; dayOfWeek: string; dayNum: number; monthLabel: string;
  isToday: boolean; isBeforeToday: boolean; plans: Plan[];
}

const weekDays = computed<WeekDay[]>(() => {
  const today = new Date();
  const currentMonday = new Date(today);
  const dayOfWeek = today.getDay() || 7; // 星期日返回 0，视为 7
  currentMonday.setDate(today.getDate() - dayOfWeek + 1 + weekOffset.value * 7);
  currentMonday.setHours(0, 0, 0, 0);

  const dayNames = ['周一', '周二', '周三', '周四', '周五', '周六', '周日'];
  const days: WeekDay[] = [];

  for (let i = 0; i < 7; i++) {
    const d = new Date(currentMonday);
    d.setDate(currentMonday.getDate() + i);
    const dateStr = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
    const todayStr = `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(today.getDate()).padStart(2, '0')}`;

    // 筛选该日计划
    const dayStart = Math.floor(d.getTime() / 1000);
    const dayEnd = dayStart + 86400;
    const dayPlans = plans.value.filter(p => {
      return p.start_date < dayEnd && p.end_date >= dayStart;
    });

    days.push({
      date: dateStr,
      dayOfWeek: dayNames[i],
      dayNum: d.getDate(),
      monthLabel: `${d.getMonth() + 1}月`,
      isToday: dateStr === todayStr,
      isBeforeToday: dateStr < todayStr,
      plans: dayPlans,
    });
  }
  return days;
});

const weekLabel = computed(() => {
  if (weekOffset.value === 0) return '本周';
  const monday = weekDays.value[0];
  return `${monday.monthLabel}${monday.dayNum}日起`;
});

const weekRange = computed(() => {
  const m = weekDays.value[0];
  const s = weekDays.value[6];
  return `${m.monthLabel}${m.dayNum}日 - ${s.monthLabel}${s.dayNum}日`;
});

const isCurrentWeek = computed(() => weekOffset.value === 0);

function prevWeek() { weekOffset.value--; }
function nextWeek() { weekOffset.value++; }
function goToday() { weekOffset.value = 0; }

// ------ 月视图计算 ------

interface MonthDay {
  date: string | null; dayNum: number; isToday: boolean; isBeforeToday: boolean; plans: Plan[];
}

const monthDays = computed<MonthDay[]>(() => {
  const today = new Date();
  const baseDate = new Date(today.getFullYear(), today.getMonth() + monthOffset.value, 1);
  const year = baseDate.getFullYear();
  const month = baseDate.getMonth();

  // 当月第一天是星期几（JS 0=星期日），转成周一=0 排列
  const firstDayJS = baseDate.getDay();
  const startCol = firstDayJS === 0 ? 6 : firstDayJS - 1;

  const daysInMonth = new Date(year, month + 1, 0).getDate();
  const todayStr = `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(today.getDate()).padStart(2, '0')}`;

  const result: MonthDay[] = [];
  let dayNum = 1;
  const totalRows = Math.ceil((startCol + daysInMonth) / 7);
  const totalCells = totalRows * 7;

  for (let i = 0; i < totalCells; i++) {
    if (i < startCol || dayNum > daysInMonth) {
      result.push({ date: null, dayNum: 0, isToday: false, isBeforeToday: false, plans: [] });
    } else {
      const d = new Date(year, month, dayNum);
      const dateStr = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
      const dayStart = Math.floor(d.getTime() / 1000);
      const dayEnd = dayStart + 86400;

      result.push({
        date: dateStr,
        dayNum,
        isToday: dateStr === todayStr,
        isBeforeToday: dateStr < todayStr,
        plans: plans.value.filter(p => p.start_date < dayEnd && p.end_date >= dayStart),
      });
      dayNum++;
    }
  }
  return result;
});

const monthLabel = computed(() => {
  const d = new Date(new Date().getFullYear(), new Date().getMonth() + monthOffset.value, 1);
  return `${d.getFullYear()}年${d.getMonth() + 1}月`;
});

const isCurrentMonth = computed(() => monthOffset.value === 0);

function prevMonth() { monthOffset.value--; }
function nextMonth() { monthOffset.value++; }
function goTodayMonth() { monthOffset.value = 0; }

// ------ 列表视图分组 ------

const groupedPlans = computed(() => {
  const groups: Record<string, { status: string; label: string; badgeClass: string; items: Plan[] }> = {
    in_progress: { status: 'in_progress', label: '进行中', badgeClass: 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300', items: [] },
    pending: { status: 'pending', label: '待开始', badgeClass: 'bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-300', items: [] },
    completed: { status: 'completed', label: '已完成', badgeClass: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-300', items: [] },
  };
  for (const p of plans.value) {
    const g = groups[p.status];
    if (g) g.items.push(p);
  }
  return Object.values(groups).filter(g => g.items.length > 0);
});

function typeLabel(t: string) {
  const map: Record<string, string> = { normal: '学习', exam_paper: '做试卷', review: '复习', homework: '作业', exam_prep: '备考' };
  return map[t] || t;
}

// ------ 科目工具 ------

function getSubjectName(id: string) {
  if (!id) return '未归类';
  return subjects.value.find(s => s.id === id)?.name || '未归类';
}
function getSubjectColor(id: string) {
  if (!id) return '#94a3b8';
  return subjects.value.find(s => s.id === id)?.color || '#94a3b8';
}
function fmtDate(ts: number) {
  const d = new Date(ts * 1000);
  return `${d.getMonth() + 1}/${d.getDate()}`;
}

function fmtDateInput(ts: number): string {
  const d = new Date(ts * 1000);
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
}

/** 将 YYYY-MM-DD 格式的本地日期转为 Unix 时间戳（当地零点） */
function toLocalMidnight(dateStr: string): number {
  const [y, m, d] = dateStr.split('-').map(Number);
  return Math.floor(new Date(y, m - 1, d).getTime() / 1000);
}

/** 将 YYYY-MM-DD 格式的本地日期转为 Unix 时间戳（当地 23:59:59） */
function toLocalEndOfDay(dateStr: string): number {
  const [y, m, d] = dateStr.split('-').map(Number);
  return Math.floor(new Date(y, m - 1, d, 23, 59, 59).getTime() / 1000);
}

// ------ 数据加载 ------

async function loadPlans() {
  plans.value = await getAllPlans({
    subject_id: filter.value.subject_id || undefined,
    status: filter.value.status || undefined,
  });
}

async function loadData() {
  subjects.value = await getAllSubjects();
  try { weeklyProgress.value = await getWeeklyGoalProgress(); } catch (e) { console.error('加载周目标进度失败:', e); }
}

// ------ 创建/编辑 ------

function openCreate(date?: string) {
  formError.value = '';
  editingPlan.value = null;
  kpSuggestions.value = [];
  kpInput.value = '';
  const defaultDate = date && date < todayStr ? todayStr : (date || todayStr);
  form.value = {
    title: '',
    subject_id: filter.value.subject_id || '',
    priority: 2,
    start_date: defaultDate,
    end_date: defaultDate,
    estimated_hours: 1,
    plan_type: 'normal',
    kpItems: [],
  };
  showModal.value = true;
}

async function openEdit(plan: Plan) {
  formError.value = '';
  editingPlan.value = plan;
  kpSuggestions.value = [];
  kpInput.value = '';
  pendingRemoveKps.value = [];
  originalKpNames.value = [];
  // 加载已有的知识点
  const kpItems: string[] = [];
  try {
    const allKps = await getAllKnowledgePoints();
    for (const kp of allKps) {
      if (kp.source === plan.id) {
        kpItems.push(kp.name);
      }
    }
    originalKpNames.value = [...kpItems];
  } catch (e) { console.error('加载知识点失败:', e); }
  form.value = {
    title: plan.title,
    subject_id: plan.subject_id,
    priority: plan.priority,
    start_date: fmtDateInput(plan.start_date),
    end_date: fmtDateInput(plan.end_date),
    estimated_hours: plan.estimated_hours,
    plan_type: plan.plan_type,
    kpItems,
  };
  showModal.value = true;
}

function closeModal() { showModal.value = false; }

async function submitForm() {
  if (!form.value.title.trim()) { formError.value = '请填写计划标题'; return; }
  if (!form.value.subject_id) { formError.value = '请选择科目'; return; }
  const base = {
    title: form.value.title,
    subject_id: form.value.subject_id || '',
    priority: form.value.priority,
    start_date: toLocalMidnight(form.value.start_date),
    end_date: toLocalEndOfDay(form.value.end_date),
    estimated_hours: form.value.estimated_hours,
    plan_type: form.value.plan_type,
  };

  try {
    if (editingPlan.value) {
      await updatePlan(editingPlan.value.id, base);
      // 处理新增的知识点（不在原始列表中的）
      const newKps = form.value.kpItems.filter(n => !originalKpNames.value.includes(n));
      if (newKps.length > 0 && form.value.subject_id) {
        try {
          await createKpsBatch({
            names: newKps,
            subject_id: form.value.subject_id,
            plan_id: editingPlan.value.id,
          });
        } catch (e) { console.error('创建知识点失败:', e); }
      }
      // 处理删除的知识点
      if (pendingRemoveKps.value.length > 0) {
        try {
          const allKps = await getAllKnowledgePoints();
          for (const kp of allKps) {
            if (kp.source === editingPlan.value.id && pendingRemoveKps.value.includes(kp.name)) {
              await deleteKnowledgePoint(kp.id);
            }
          }
        } catch (e) { console.error('删除知识点失败:', e); }
      }
      await loadPlans();
      closeModal();
    } else {
      const plan = await createPlan(base);
      // 内联创建知识点
      if (form.value.kpItems.length > 0 && form.value.subject_id) {
        try {
          await createKpsBatch({
            names: form.value.kpItems,
            subject_id: form.value.subject_id,
            plan_id: plan.id,
          });
        } catch (e) { console.error('创建知识点失败:', e); }
      }
      closeModal();
      // 刷新目标进度
      try { weeklyProgress.value = await getWeeklyGoalProgress(); } catch (e) { console.error('刷新目标进度失败:', e); }
      await loadPlans();
    }
  } catch (e) {
    console.error('保存计划失败:', e);
  }
}

// ------ 删除 ------

function confirmDelete(plan: Plan) { deletingPlan.value = plan; showDeleteConfirm.value = true; }
async function doDelete() {
  if (deletingPlan.value) { await deletePlan(deletingPlan.value.id); await loadPlans(); }
  showDeleteConfirm.value = false;
  deletingPlan.value = null;
  closeModal();
}

onMounted(() => { loadData(); loadPlans(); });
</script>
