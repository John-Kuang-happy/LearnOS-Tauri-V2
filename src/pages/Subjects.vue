<template>
  <div class="space-y-6">
    <!-- 页头 -->
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white">科目管理</h2>
        <p class="text-gray-500 dark:text-gray-400 mt-1">管理你的学习科目，设置每周目标</p>
      </div>
      <button @click="openCreateModal" class="btn-primary">
        + 添加科目
      </button>
    </div>

    <!-- 科目列表 -->
    <div v-if="subjects.length === 0" class="card text-center py-16">
      <span class="text-6xl">📚</span>
      <h3 class="text-xl font-semibold text-gray-900 dark:text-white mt-4">还没有科目</h3>
      <p class="text-gray-500 dark:text-gray-400 mt-2">点击右上角「添加科目」开始吧！</p>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <div
        v-for="subject in subjects"
        :key="subject.id"
        class="card hover:shadow-md transition-shadow"
      >
        <div class="flex items-start justify-between mb-3">
          <div class="flex items-center gap-3">
            <div
              class="w-10 h-10 rounded-xl flex items-center justify-center text-white text-lg"
              :style="{ backgroundColor: subject.color }"
            >
              {{ subject.icon }}
            </div>
            <div>
              <h3 class="font-semibold text-gray-900 dark:text-white">{{ subject.name }}</h3>
              <p class="text-xs text-gray-500 dark:text-gray-400">
                {{ subject.category === 'liberal_arts' ? '文科' : subject.category === 'science' ? '理科' : '其他' }}
              </p>
            </div>
          </div>
          <div class="flex items-center gap-1">
            <button @click="openEditModal(subject)" class="p-1.5 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-400 hover:text-primary-500 transition-colors">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
              </svg>
            </button>
            <button @click="confirmDelete(subject)" class="p-1.5 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 text-gray-400 hover:text-red-500 transition-colors">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
              </svg>
            </button>
          </div>
        </div>

        <div class="space-y-2">
          <!-- 时间目标 -->
          <div class="flex items-center justify-between text-xs">
            <span class="text-gray-500 dark:text-gray-400">⏱ 时间</span>
            <span class="font-medium text-gray-900 dark:text-white">
              {{ (getProgress(subject)?.weekly_hours || 0).toFixed(1) }} / {{ subject.weekly_goal_hours }}h
            </span>
          </div>
          <div class="w-full h-1.5 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
            <div class="h-full rounded-full transition-all opacity-60" :style="{ width: (getProgress(subject)?.time_rate || 0) * 100 + '%', backgroundColor: subject.color }"></div>
          </div>
          <!-- KP目标 -->
          <div v-if="subject.weekly_goal_kps > 0" class="flex items-center justify-between text-xs">
            <span class="text-gray-500 dark:text-gray-400">📖 知识点</span>
            <span class="font-medium text-gray-900 dark:text-white">
              +{{ getProgress(subject)?.new_kps || 0 }} / {{ subject.weekly_goal_kps }}个
            </span>
          </div>
          <div v-if="subject.weekly_goal_kps > 0" class="w-full h-1.5 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
            <div class="h-full rounded-full transition-all opacity-60" :style="{ width: (getProgress(subject)?.kp_rate || 0) * 100 + '%', backgroundColor: subject.color }"></div>
          </div>
          <!-- 综合达成 -->
          <div class="flex items-center justify-between text-xs pt-1 border-t border-gray-100 dark:border-gray-700">
            <span class="text-gray-500 dark:text-gray-400">综合</span>
            <span class="font-semibold text-sm" :class="getCompositePercent(subject) >= 80 ? 'text-green-500' : getCompositePercent(subject) >= 40 ? 'text-amber-500' : 'text-red-500'">
              {{ getCompositePercent(subject) }}%
            </span>
          </div>
          <div class="w-full h-2 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
            <div class="h-full rounded-full transition-all" :style="{ width: getCompositePercent(subject) + '%', backgroundColor: subject.color }"></div>
          </div>
        </div>
      </div>
    </div>

    <!-- 创建/编辑弹窗 -->
    <div v-if="showModal" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="absolute inset-0 bg-black/50" @click="closeModal"></div>
      <div class="relative bg-white dark:bg-dark-surface rounded-2xl shadow-xl p-6 w-full max-w-md mx-4">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          {{ editingSubject ? '编辑科目' : '添加科目' }}
        </h3>

        <div class="space-y-4">
          <div>
            <label class="label">科目名称</label>
            <input v-model="form.name" class="input" placeholder="例如：数学、语文、英语..." />
          </div>

          <div>
            <label class="label">图标</label>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="icon in iconOptions"
                :key="icon"
                @click="form.icon = icon"
                class="w-10 h-10 text-lg rounded-xl border-2 transition-colors"
                :class="form.icon === icon
                  ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20'
                  : 'border-gray-200 dark:border-gray-600 hover:border-gray-400'"
              >
                {{ icon }}
              </button>
            </div>
          </div>

          <div>
            <label class="label">分类</label>
            <div class="flex gap-2">
              <button
                v-for="cat in categories"
                :key="cat.value"
                @click="form.category = cat.value"
                class="flex-1 py-2 px-3 rounded-xl border-2 text-sm font-medium transition-colors"
                :class="form.category === cat.value
                  ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20 text-primary-700 dark:text-primary-300'
                  : 'border-gray-200 dark:border-gray-600 text-gray-600 dark:text-gray-400 hover:border-gray-400'"
              >
                {{ cat.label }}
              </button>
            </div>
          </div>

          <div>
            <label class="label">颜色</label>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="color in colorOptions"
                :key="color"
                @click="form.color = color"
                class="w-8 h-8 rounded-full border-2 transition-transform"
                :class="form.color === color ? 'border-gray-900 dark:border-white scale-110' : 'border-transparent hover:scale-105'"
                :style="{ backgroundColor: color }"
              ></button>
            </div>
          </div>

          <div>
            <label class="label">每周目标（小时）</label>
            <input v-model.number="form.weekly_goal_hours" type="number" min="1" max="40" step="0.5" class="input" />
          </div>

          <div>
            <label class="label">每周目标（知识点）<span class="text-gray-400 text-xs ml-1">选填，设 0 表示只统计时间</span></label>
            <input v-model.number="form.weekly_goal_kps" type="number" min="0" max="20" step="1" class="input" placeholder="例如：3" />
          </div>
        </div>

        <p v-if="formError" class="text-red-500 text-sm mb-2">{{ formError }}</p>
        <div class="flex justify-end gap-3">
          <button @click="closeModal" class="btn-secondary">取消</button>
          <button @click="submitForm" class="btn-primary">
            {{ editingSubject ? '保存修改' : '添加' }}
          </button>
        </div>
      </div>
    </div>

    <!-- 删除确认弹窗 -->
    <div v-if="showDeleteConfirm" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="absolute inset-0 bg-black/50" @click="showDeleteConfirm = false"></div>
      <div class="relative bg-white dark:bg-dark-surface rounded-2xl shadow-xl p-6 w-full max-w-sm mx-4">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">确认删除</h3>
        <p class="text-gray-500 dark:text-gray-400">
          确定要删除科目「{{ deletingSubject?.name }}」吗？关联的学习计划将一并删除，此操作不可撤销。
        </p>
        <div class="flex justify-end gap-3 mt-6">
          <button @click="showDeleteConfirm = false" class="btn-secondary">取消</button>
          <button @click="doDelete" class="btn-danger">确认删除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { getAllSubjects, createSubject, updateSubject, deleteSubject, getWeeklyGoalProgress } from '../composables/api';
import type { Subject, WeeklyGoalProgress } from '../types';

const subjects = ref<Subject[]>([]);
const weeklyProgressMap = ref<Record<string, WeeklyGoalProgress>>({});
const showModal = ref(false);
const showDeleteConfirm = ref(false);
const editingSubject = ref<Subject | null>(null);
const deletingSubject = ref<Subject | null>(null);
const formError = ref('');

const iconOptions = ['📖', '📐', '🔤', '⚡', '🧪', '🧬', '⚖️', '🏛️', '🌍', '💻', '🎨', '🏃'];
const categories = [
  { value: 'liberal_arts', label: '文科' },
  { value: 'science', label: '理科' },
  { value: 'other', label: '其他' },
];
const colorOptions = [
  '#0ea5e9', '#3b82f6', '#6366f1', '#8b5cf6',
  '#d946ef', '#ec4899', '#f43f5e', '#f97316',
  '#eab308', '#22c55e', '#14b8a6', '#64748b',
];

const defaultForm = () => ({
  name: '',
  icon: '📖',
  category: 'other',
  color: '#0ea5e9',
  weekly_goal_hours: 10,
  weekly_goal_kps: 3,
});

const form = ref(defaultForm());

function getProgress(subject: Subject): WeeklyGoalProgress | undefined {
  return weeklyProgressMap.value[subject.id];
}

function getCompositePercent(subject: Subject): number {
  const p = weeklyProgressMap.value[subject.id];
  if (!p) return 0;
  return Math.min(100, Math.round(p.composite_rate * 100));
}

async function loadSubjects() {
  subjects.value = await getAllSubjects();
  try {
    const progress = await getWeeklyGoalProgress();
    const map: Record<string, WeeklyGoalProgress> = {};
    for (const p of progress) {
      map[p.subject_id] = p;
    }
    weeklyProgressMap.value = map;
  } catch {
    // 加载失败不影响主流程
  }
}

function openCreateModal() {
  formError.value = '';
  editingSubject.value = null;
  form.value = defaultForm();
  showModal.value = true;
}

function openEditModal(subject: Subject) {
  formError.value = '';
  editingSubject.value = subject;
  form.value = {
    name: subject.name,
    icon: subject.icon,
    category: subject.category,
    color: subject.color,
    weekly_goal_hours: subject.weekly_goal_hours,
    weekly_goal_kps: subject.weekly_goal_kps,
  };
  showModal.value = true;
}

function closeModal() {
  showModal.value = false;
}

async function submitForm() {
  if (!form.value.name.trim()) { formError.value = '请填写科目名称'; return; }

  try {
    if (editingSubject.value) {
      await updateSubject(editingSubject.value.id, form.value);
    } else {
      await createSubject({
        name: form.value.name,
        color: form.value.color,
        icon: form.value.icon,
        category: form.value.category,
        weekly_goal_hours: form.value.weekly_goal_hours,
        weekly_goal_kps: form.value.weekly_goal_kps,
      });
    }
    await loadSubjects();
    closeModal();
  } catch (e) {
    console.error('保存科目失败:', e);
  }
}

function confirmDelete(subject: Subject) {
  deletingSubject.value = subject;
  showDeleteConfirm.value = true;
}

async function doDelete() {
  if (deletingSubject.value) {
    await deleteSubject(deletingSubject.value.id);
    await loadSubjects();
  }
  showDeleteConfirm.value = false;
  deletingSubject.value = null;
}

onMounted(() => {
  loadSubjects();
});
</script>
