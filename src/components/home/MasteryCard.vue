<template>
  <div v-if="visible" class="fixed inset-0 z-50 flex items-center justify-center">
    <div class="absolute inset-0 bg-black/50" @click="onClose"></div>
    <div class="relative bg-white dark:bg-dark-surface rounded-2xl shadow-xl p-6 w-full max-w-sm mx-4">

      <!-- ====== 输入页 ====== -->
      <template v-if="!submitted">
        <button @click="onClose" class="absolute top-3 right-3 w-8 h-8 rounded-full bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 flex items-center justify-center text-gray-500 dark:text-gray-400 transition-colors">
          ✕
        </button>
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-1 pr-8">⏰ 番茄钟结束！</h3>
        <p v-if="planTitle" class="text-sm text-gray-500 dark:text-gray-400 mb-4">{{ planTitle }}</p>

        <!-- 有 KP 关联：问掌握度 -->
        <template v-if="currentHasKp">
          <p class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">掌握得怎么样？</p>
          <div class="flex gap-2 mb-3">
            <button v-for="opt in masteryOptions" :key="opt.label"
              @click="selectMastery(opt)"
              class="flex-1 p-3 rounded-xl border-2 text-sm font-medium transition-all text-center"
              :class="selectedLabel === opt.label
                ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20 text-primary-700 dark:text-primary-300'
                : 'border-gray-200 dark:border-gray-600 text-gray-600 dark:text-gray-400 hover:border-gray-300'">
              <span class="block text-xl mb-0.5">{{ opt.icon }}</span>
              {{ opt.label }}
            </button>
          </div>
        </template>
        <template v-else>
          <p class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">这次学习感觉怎么样？</p>
        </template>

        <!-- 心情（可折叠） -->
        <div class="mb-3">
          <button @click="showMood = !showMood"
            class="flex items-center gap-1 text-xs text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors">
            <span class="text-sm">{{ moodOptions.find(m => m.score === selectedMood)?.icon || '😐' }}</span>
            {{ showMood ? '收起心情' : '记录心情（选填）' }}
            <span class="text-[10px]">{{ showMood ? '▲' : '▼' }}</span>
          </button>
          <div v-if="showMood" class="flex gap-2 mt-2">
            <button v-for="m in moodOptions" :key="m.score"
              @click="selectedMood = m.score"
              class="text-xl p-1.5 rounded-lg transition-all"
              :class="selectedMood === m.score ? 'scale-110 bg-gray-100 dark:bg-gray-700' : 'opacity-40 hover:opacity-70'"
              :title="m.label">
              {{ m.icon }}
            </button>
          </div>
        </div>

        <div class="flex justify-end gap-2">
          <button @click="onClose" class="btn-ghost text-sm">跳过</button>
          <button @click="doSubmit" class="btn-primary text-sm">记录</button>
        </div>
      </template>

      <!-- ====== 结果页 ====== -->
      <template v-else>
        <div class="text-center">
          <span class="text-4xl block mb-2">{{ resultIcon }}</span>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-1">已记录</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400 mb-3">{{ planTitle }}</p>

          <!-- 掌握状态 -->
          <div v-if="currentHasKp" class="mb-3 p-3 rounded-xl"
            :class="resultMastered ? 'bg-green-50 dark:bg-green-900/20' : 'bg-amber-50 dark:bg-amber-900/20'">
            <p class="text-sm font-medium" :class="resultMastered ? 'text-green-700 dark:text-green-400' : 'text-amber-700 dark:text-amber-400'">
              {{ resultMastered ? '✅ 已掌握' : resultMessage }}
            </p>
            <p v-if="resultNextDate" class="text-xs text-gray-400 mt-1">⏰ 下次复习：{{ resultNextDate }}</p>
          </div>

          <div class="flex gap-2 justify-center">
            <button @click="onClose" class="btn-secondary text-sm">好的</button>
            <button v-if="!resultMastered" @click="retry" class="btn-primary text-sm">🔄 再学一次</button>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

const emit = defineEmits<{
  submit: [data: { mastery_score: number; mastery_label: string; mood_score: number }];
  retry: [];
  close: [];
}>();

defineProps<{
  planTitle?: string;
}>();

const visible = ref(false);
const submitted = ref(false);
const showMood = ref(false);
const currentHasKp = ref(true);
const selectedLabel = ref('');
const selectedScore = ref(0.65);
const selectedMood = ref(3);
const resultMastered = ref(false);
const resultNextDate = ref('');
const resultMessage = ref('');

const masteryOptions = [
  { icon: '😫', label: '不太熟', score: 0.25 },
  { icon: '👍', label: '掌握了', score: 0.65 },
  { icon: '🎯', label: '很扎实', score: 0.90 },
];

const moodOptions = [
  { icon: '😫', label: '很差', score: 1 },
  { icon: '😔', label: '不太好', score: 2 },
  { icon: '😐', label: '一般', score: 3 },
  { icon: '😊', label: '不错', score: 4 },
  { icon: '😄', label: '很好', score: 5 },
];

const resultIcon = computed(() => resultMastered.value ? '🎉' : '📖');

function selectMastery(opt: typeof masteryOptions[0]) {
  selectedLabel.value = opt.label;
  selectedScore.value = opt.score;
}

function show(_title?: string, hasKp?: boolean) {
  currentHasKp.value = hasKp ?? true;
  selectedLabel.value = '';
  selectedScore.value = 0.65;
  selectedMood.value = 3;
  showMood.value = false;
  submitted.value = false;
  resultMastered.value = false;
  resultNextDate.value = '';
  resultMessage.value = '';
  visible.value = true;
}

function showResult(opts: { mastered: boolean; nextDate?: string; message?: string }) {
  resultMastered.value = opts.mastered;
  resultNextDate.value = opts.nextDate || '';
  resultMessage.value = opts.message || (opts.mastered ? '已掌握' : '还需巩固 1 次');
  submitted.value = true;
}

function doSubmit() {
  emit('submit', {
    mastery_score: selectedScore.value,
    mastery_label: selectedLabel.value || '掌握了',
    mood_score: selectedMood.value,
  });
}

function retry() {
  visible.value = false;
  emit('retry');
}

function onClose() {
  if (submitted.value) {
    emit('close');
  }
  visible.value = false;
}

function close() {
  visible.value = false;
}

defineExpose({ show, showResult, close });
</script>
