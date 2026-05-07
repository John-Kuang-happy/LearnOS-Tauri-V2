<template>
  <div class="min-h-screen bg-gray-50 dark:bg-dark-bg">
    <!-- 侧边导航栏 -->
    <AppSidebar />

    <!-- 主内容区 -->
    <div
      class="transition-all duration-300"
      :class="appStore.isSidebarOpen ? 'ml-64' : 'ml-0'"
    >
      <!-- 顶部栏 -->
      <AppHeader />

      <!-- 页面内容（含过渡动画） -->
      <main class="p-6">
        <router-view v-slot="{ Component }">
          <Transition name="page" mode="out-in">
            <component :is="Component" />
          </Transition>
        </router-view>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useAppStore } from './stores/appStore';
import AppSidebar from './components/layout/AppSidebar.vue';
import AppHeader from './components/layout/AppHeader.vue';

const appStore = useAppStore();

onMounted(() => {
  appStore.initTheme();
});
</script>
