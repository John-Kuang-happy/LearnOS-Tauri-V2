<template>
  <div v-if="active" class="card border-2 border-primary-200 dark:border-primary-800">
    <div class="flex items-center justify-between mb-3">
      <h3 class="font-semibold text-primary-600">⏱️ 番茄钟</h3>
      <span v-if="studyTitle" class="text-sm text-gray-500 dark:text-gray-400 truncate max-w-48">{{ studyTitle }}</span>
    </div>

    <div class="text-center mb-3">
      <span class="text-4xl font-bold tabular-nums text-gray-900 dark:text-white">{{ displayTime }}</span>
      <span class="text-sm text-gray-400 ml-1">/ {{ presetLabel }}</span>
    </div>

    <!-- 进度条 -->
    <div class="w-full h-2 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden mb-4">
      <div class="h-full bg-primary-500 rounded-full transition-all duration-1000"
        :style="{ width: progress + '%' }"></div>
    </div>

    <div class="flex items-center justify-center gap-3">
      <button v-if="!running && !paused" @click="start" class="btn-primary text-sm">
        ▶ 开始
      </button>
      <button v-if="running" @click="pause" class="btn-secondary text-sm">
        ⏸ 暂停
      </button>
      <button v-if="paused" @click="resume" class="btn-primary text-sm">
        ▶ 继续
      </button>
      <button v-if="running || paused" @click="stop" class="btn-ghost text-sm text-red-500">
        ⏹ 结束
      </button>
    </div>

    <!-- 预设时长 -->
    <div v-if="!running && !paused" class="flex justify-center gap-2 mt-3">
      <button v-for="opt in presets" :key="opt.min"
        @click="setDuration(opt.min)"
        class="text-xs px-2.5 py-1 rounded-full transition-colors"
        :class="durationMin === opt.min ? 'bg-primary-100 text-primary-700 dark:bg-primary-900/30 dark:text-primary-300' : 'bg-gray-100 text-gray-500 dark:bg-gray-700 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-600'">
        {{ opt.label }}
      </button>
    </div>
  </div>

  <!-- 未激活时不显示 -->
</template>

<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue';

const emit = defineEmits<{
  complete: [durationSeconds: number];
}>();

defineProps<{
  studyTitle?: string;
}>();

const presets = [
  { min: 15, label: '15分' },
  { min: 25, label: '25分' },
  { min: 45, label: '45分' },
  { min: 60, label: '60分' },
];

const active = ref(false);
const running = ref(false);
const paused = ref(false);
const durationMin = ref(25);
const elapsedSeconds = ref(0);
let timerInterval: ReturnType<typeof setInterval> | null = null;

const durationSeconds = computed(() => durationMin.value * 60);
const displayTime = computed(() => {
  const remaining = Math.max(0, durationSeconds.value - elapsedSeconds.value);
  const m = Math.floor(remaining / 60);
  const s = remaining % 60;
  return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
});
const progress = computed(() => Math.min(100, (elapsedSeconds.value / durationSeconds.value) * 100));
const presetLabel = computed(() => `${durationMin.value}分钟`);

function setDuration(min: number) {
  durationMin.value = min;
  elapsedSeconds.value = 0;
}

function start() {
  active.value = true;
  running.value = true;
  paused.value = false;
  timerInterval = setInterval(() => {
    elapsedSeconds.value++;
    if (elapsedSeconds.value >= durationSeconds.value) {
      stop();
    }
  }, 1000);
}

function pause() {
  running.value = false;
  paused.value = true;
  if (timerInterval) { clearInterval(timerInterval); timerInterval = null; }
}

function resume() {
  running.value = true;
  paused.value = false;
  timerInterval = setInterval(() => {
    elapsedSeconds.value++;
    if (elapsedSeconds.value >= durationSeconds.value) {
      stop();
    }
  }, 1000);
}

function stop() {
  running.value = false;
  paused.value = false;
  if (timerInterval) { clearInterval(timerInterval); timerInterval = null; }
  emit('complete', elapsedSeconds.value);
  active.value = false;
  elapsedSeconds.value = 0;
}

function startWithTitle(_title: string, duration?: number) {
  durationMin.value = duration || 25;
  elapsedSeconds.value = 0;
  // Let parent update studyTitle via props
  start();
}

defineExpose({ startWithTitle, active, running });

onUnmounted(() => {
  if (timerInterval) clearInterval(timerInterval);
});
</script>
