import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export const useAppStore = defineStore('app', () => {
  const darkMode = ref(false);
  const sidebarOpen = ref(true);

  const isDarkMode = computed(() => darkMode.value);
  const isSidebarOpen = computed(() => sidebarOpen.value);

  function toggleDarkMode() {
    darkMode.value = !darkMode.value;
    if (darkMode.value) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }

  function toggleSidebar() {
    sidebarOpen.value = !sidebarOpen.value;
  }

  // 初始化主题
  function initTheme() {
    const saved = localStorage.getItem('learnos-app');
    if (saved) {
      try {
        const parsed = JSON.parse(saved);
        if (parsed.darkMode) {
          darkMode.value = true;
          document.documentElement.classList.add('dark');
        }
      } catch (e) { console.error('解析暗黑模式设置失败:', e); }
    }
    // 或者跟随系统
    if (window.matchMedia('(prefers-color-scheme: dark)').matches && !saved) {
      darkMode.value = true;
      document.documentElement.classList.add('dark');
    }
  }

  return {
    darkMode,
    sidebarOpen,
    isDarkMode,
    isSidebarOpen,
    toggleDarkMode,
    toggleSidebar,
    initTheme,
  };
}, {
  persist: {
    key: 'learnos-app',
    storage: localStorage,
    pick: ['darkMode'],
  },
});
