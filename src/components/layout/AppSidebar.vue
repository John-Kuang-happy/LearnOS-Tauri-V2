<template>
  <aside
    class="fixed top-0 left-0 z-40 h-screen w-64 bg-white dark:bg-dark-surface border-r border-gray-200 dark:border-gray-700 transition-transform overflow-y-auto"
    :class="appStore.isSidebarOpen ? 'translate-x-0' : '-translate-x-full'"
  >
    <!-- Logo -->
    <div class="p-6">
      <h1 class="text-2xl font-bold text-primary-500">LearnOS</h1>
      <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">高中生智能学习助手</p>
    </div>

    <!-- 导航 -->
    <nav class="mt-2 px-3 space-y-1">
      <router-link
        to="/"
        class="flex items-center gap-3 px-4 py-3 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700/50 transition-colors"
        active-class="bg-primary-50 dark:bg-primary-900/20 text-primary-600 dark:text-primary-400 font-medium"
      >
        <span class="text-lg">🏠</span>
        首页
      </router-link>

      <router-link
        to="/subjects"
        class="flex items-center gap-3 px-4 py-3 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700/50 transition-colors"
        active-class="bg-primary-50 dark:bg-primary-900/20 text-primary-600 dark:text-primary-400 font-medium"
      >
        <span class="text-lg">📚</span>
        科目管理
      </router-link>

      <router-link
        to="/plans"
        class="flex items-center gap-3 px-4 py-3 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700/50 transition-colors"
        active-class="bg-primary-50 dark:bg-primary-900/20 text-primary-600 dark:text-primary-400 font-medium"
      >
        <span class="text-lg">📋</span>
        学习计划
      </router-link>

      <router-link
        to="/review-board"
        class="flex items-center gap-3 px-4 py-3 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700/50 transition-colors"
        active-class="bg-primary-50 dark:bg-primary-900/20 text-primary-600 dark:text-primary-400 font-medium"
      >
        <span class="text-lg">🔄</span>
        复习看板
      </router-link>

      <div class="my-3 border-t border-gray-200 dark:border-gray-700"></div>

      <router-link
        to="/progress"
        class="flex items-center gap-3 px-4 py-3 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700/50 transition-colors"
        active-class="bg-primary-50 dark:bg-primary-900/20 text-primary-600 dark:text-primary-400 font-medium"
      >
        <span class="text-lg">📈</span>
        学习进度
      </router-link>

      <router-link
        to="/analysis"
        class="flex items-center gap-3 px-4 py-3 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700/50 transition-colors"
        active-class="bg-primary-50 dark:bg-primary-900/20 text-primary-600 dark:text-primary-400 font-medium"
      >
        <span class="text-lg">📊</span>
        智能分析
      </router-link>

      <router-link
        to="/exams"
        class="flex items-center gap-3 px-4 py-3 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700/50 transition-colors"
        active-class="bg-primary-50 dark:bg-primary-900/20 text-primary-600 dark:text-primary-400 font-medium"
      >
        <span class="text-lg">📅</span>
        考试倒计时
      </router-link>

      <div class="my-3 border-t border-gray-200 dark:border-gray-700"></div>

      <router-link
        to="/settings"
        class="flex items-center gap-3 px-4 py-3 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700/50 transition-colors"
        active-class="bg-primary-50 dark:bg-primary-900/20 text-primary-600 dark:text-primary-400 font-medium"
      >
        <span class="text-lg">⚙️</span>
        设置
      </router-link>

      <router-link
        to="/help"
        class="flex items-center gap-3 px-4 py-3 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-700/50 transition-colors"
        active-class="bg-primary-50 dark:bg-primary-900/20 text-primary-600 dark:text-primary-400 font-medium"
      >
        <span class="text-lg">❓</span>
        帮助
      </router-link>
    </nav>

    <!-- 底部版本 -->
    <div class="absolute bottom-0 left-0 right-0 p-6">
      <p class="text-xs text-gray-400 dark:text-gray-600 text-center">LearnOS v0.2.0</p>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { useAppStore } from '../../stores/appStore';

const appStore = useAppStore();
</script>
