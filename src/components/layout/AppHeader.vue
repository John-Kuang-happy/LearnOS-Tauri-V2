<template>
  <header class="sticky top-0 z-30 bg-white/80 dark:bg-dark-surface/80 backdrop-blur-md border-b border-gray-200 dark:border-gray-700">
    <div class="flex items-center justify-between px-6 py-3">
      <button
        @click="appStore.toggleSidebar"
        class="p-2 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
      >
        <svg class="w-5 h-5 text-gray-600 dark:text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
        </svg>
      </button>

      <div class="flex items-center gap-3">
        <!-- 当前页面标题 -->
        <span class="text-sm font-medium text-gray-600 dark:text-gray-400">
          {{ route.meta?.title || '' }}
        </span>

        <!-- 暗黑模式切换 -->
        <button
          @click="appStore.toggleDarkMode"
          class="p-2 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          :title="appStore.isDarkMode ? '切换到亮色模式' : '切换到暗黑模式'"
        >
          <span class="text-lg">
            {{ appStore.isDarkMode ? '🌙' : '☀️' }}
          </span>
        </button>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useAppStore } from '../../stores/appStore';

const route = useRoute();
const appStore = useAppStore();
</script>
