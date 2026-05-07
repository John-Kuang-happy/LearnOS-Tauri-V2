<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-2xl font-bold text-gray-900 dark:text-white">考试倒计时</h2>
        <p class="text-gray-500 dark:text-gray-400 mt-1">管理考试日期和倒计时</p>
      </div>
      <button @click="openCreateModal" class="btn-primary">+ 添加考试</button>
    </div>

    <!-- 考试列表 -->
    <div v-if="exams.length === 0" class="card text-center py-16">
      <span class="text-6xl">📅</span>
      <h3 class="text-xl font-semibold text-gray-900 dark:text-white mt-4">暂无考试</h3>
      <p class="text-gray-500 dark:text-gray-400 mt-2">添加高考、期末等考试日期，开始倒计时！</p>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div v-for="exam in exams" :key="exam.id" class="card hover:shadow-md transition-shadow"
        :class="daysUntil(exam.target_date) <= 30 ? 'border-l-4 border-l-red-500' : ''">
        <div class="flex items-start justify-between mb-3">
          <div>
            <div class="flex items-center gap-2 mb-1">
              <span class="text-xs px-2 py-0.5 rounded-full font-medium"
                :class="typeBadge(exam.exam_type)">
                {{ typeLabel(exam.exam_type) }}
              </span>
            </div>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">{{ exam.name }}</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">{{ formatDate(exam.target_date) }}</p>
          </div>
          <div class="text-right">
            <p class="text-3xl font-bold"
              :class="daysUntil(exam.target_date) <= 7 ? 'text-red-500' : daysUntil(exam.target_date) <= 30 ? 'text-amber-500' : 'text-primary-500'">
              {{ daysUntil(exam.target_date) }}
            </p>
            <p class="text-xs text-gray-500 dark:text-gray-400">天</p>
          </div>
        </div>

        <!-- 倒计时进度条 -->
        <div class="w-full h-2 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden mb-3">
          <div class="h-full rounded-full transition-all" :style="{ width: progressWidth(exam.target_date), backgroundColor: progressColor(exam.target_date) }"></div>
        </div>

        <div class="flex items-center gap-2 mt-3">
          <button @click="confirmDelete(exam)" class="ml-auto text-gray-400 hover:text-red-500 text-sm">🗑️</button>
        </div>
      </div>
    </div>

    <!-- 创建/编辑弹窗 -->
    <div v-if="showModal" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="absolute inset-0 bg-black/50" @click="closeModal"></div>
      <div class="relative bg-white dark:bg-dark-surface rounded-2xl shadow-xl p-6 w-full max-w-md mx-4">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">添加考试</h3>
        <div class="space-y-4">
          <div>
            <label class="label">考试名称</label>
            <input v-model="form.name" class="input" placeholder="例如：2026年高考、期末考试" />
          </div>
          <div>
            <label class="label">类型</label>
            <select v-model="form.exam_type" class="input">
              <option value="gaokao">高考</option>
              <option value="final">期末考试</option>
              <option value="midterm">期中考试</option>
              <option value="mock">模拟考试</option>
            </select>
          </div>
          <div>
            <label class="label">考试日期</label>
            <div class="grid grid-cols-3 gap-2">
              <div class="flex items-center gap-1">
                <select v-model.number="form.year" class="input flex-1">
                  <option v-for="y in yearOptions" :key="y" :value="y">{{ y }}</option>
                </select>
                <span class="text-gray-400 text-sm">年</span>
              </div>
              <div class="flex items-center gap-1">
                <select v-model.number="form.month" class="input flex-1">
                  <option v-for="m in 12" :key="m" :value="m">{{ String(m).padStart(2, '0') }}</option>
                </select>
                <span class="text-gray-400 text-sm">月</span>
              </div>
              <div class="flex items-center gap-1">
                <select v-model.number="form.day" class="input flex-1">
                  <option v-for="d in maxDays" :key="d" :value="d">{{ String(d).padStart(2, '0') }}</option>
                </select>
                <span class="text-gray-400 text-sm">日</span>
              </div>
            </div>
          </div>
          <div>
            <label class="label">备注</label>
            <input v-model="form.remarks" class="input" placeholder="可选" />
          </div>
        </div>
        <p v-if="formError" class="text-red-500 text-sm mb-2">{{ formError }}</p>
        <div class="flex justify-end gap-3">
          <button @click="closeModal" class="btn-secondary">取消</button>
          <button @click="submitForm" class="btn-primary">添加</button>
        </div>
      </div>
    </div>

    <!-- 删除确认 -->
    <div v-if="showDeleteConfirm" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="absolute inset-0 bg-black/50" @click="showDeleteConfirm = false"></div>
      <div class="relative bg-white dark:bg-dark-surface rounded-2xl shadow-xl p-6 w-full max-w-sm mx-4">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">确认删除</h3>
        <p class="text-gray-500 dark:text-gray-400">确定要删除考试「{{ deletingExam?.name }}」吗？</p>
        <div class="flex justify-end gap-3 mt-6">
          <button @click="showDeleteConfirm = false" class="btn-secondary">取消</button>
          <button @click="doDelete" class="btn-danger">确认删除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { getAllExams, createExam, deleteExam } from '../composables/api';
import type { Exam } from '../types';

const exams = ref<Exam[]>([]);
const showModal = ref(false);
const showDeleteConfirm = ref(false);
const deletingExam = ref<Exam | null>(null);
const formError = ref('');

const form = ref({ name: '', exam_type: 'gaokao', year: 0, month: 0, day: 0, remarks: '' });

const now = new Date();
const yearOptions = computed(() => {
  const y = now.getFullYear();
  return [y - 1, y, y + 1, y + 2];
});
const maxDays = computed(() => {
  if (!form.value.year || !form.value.month) return 31;
  return new Date(form.value.year, form.value.month, 0).getDate();
});

function typeLabel(t: string) { return ({ gaokao: '高考', final: '期末', midterm: '期中', mock: '模拟' } as any)[t] || t; }
function typeBadge(t: string) {
  return ({ gaokao: 'bg-red-100 text-red-700', final: 'bg-amber-100 text-amber-700', midterm: 'bg-blue-100 text-blue-700', mock: 'bg-gray-100 text-gray-700' } as any)[t] || '';
}
function daysUntil(ts: number) {
  const diff = ts * 1000 - Date.now();
  return Math.max(0, Math.ceil(diff / (1000 * 60 * 60 * 24)));
}
function formatDate(ts: number) { return new Date(ts * 1000).toLocaleDateString('zh-CN'); }
function progressWidth(ts: number) {
  // 倒推一年作为起点
  const total = 365;
  const remaining = daysUntil(ts);
  return `${Math.max(0, Math.min(100, (remaining / total) * 100))}%`;
}
function progressColor(ts: number) {
  const d = daysUntil(ts);
  if (d <= 7) return '#ef4444';
  if (d <= 30) return '#f97316';
  return '#0ea5e9';
}
async function loadExams() {
  exams.value = await getAllExams();
}

function openCreateModal() {
  formError.value = '';
  form.value = { name: '', exam_type: 'gaokao', year: now.getFullYear(), month: now.getMonth() + 1, day: now.getDate(), remarks: '' };
  showModal.value = true;
}

function closeModal() { showModal.value = false; }

async function submitForm() {
  if (!form.value.name) { formError.value = '请填写考试名称'; return; }
  try {
    await createExam({
      name: form.value.name,
      exam_type: form.value.exam_type,
      target_date: Math.floor(new Date(form.value.year, form.value.month - 1, form.value.day).getTime() / 1000),
      remarks: form.value.remarks || undefined,
    });
    await loadExams();
    closeModal();
  } catch (e) {
    console.error('保存考试失败:', e);
  }
}

function confirmDelete(exam: Exam) { deletingExam.value = exam; showDeleteConfirm.value = true; }
async function doDelete() {
  if (deletingExam.value) { await deleteExam(deletingExam.value.id); await loadExams(); }
  showDeleteConfirm.value = false;
}

onMounted(() => { loadExams(); });
</script>
