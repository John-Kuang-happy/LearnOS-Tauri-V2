<template>
  <div class="space-y-6">
    <div>
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white">设置</h2>
      <p class="text-gray-500 dark:text-gray-400 mt-1">管理应用配置和数据</p>
    </div>

    <!-- LLM AI 配置 -->
    <div class="card">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">AI 智能分析（可选）</h3>
      <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
        启用 AI 后，将发送聚合统计数据（不含隐私笔记）到 AI 模型获取深度建议。需要自行提供 API Key。
      </p>
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <p class="font-medium text-gray-900 dark:text-white">启用 AI 分析</p>
            <p class="text-sm text-gray-500 dark:text-gray-400">使用 LLM 生成个性化学习建议</p>
          </div>
          <button @click="toggleLlm"
            class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors"
            :class="settings.llm_enabled ? 'bg-primary-500' : 'bg-gray-200 dark:bg-gray-700'">
            <span class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform"
              :class="settings.llm_enabled ? 'translate-x-6' : 'translate-x-1'" />
          </button>
        </div>

        <div v-if="settings.llm_enabled" class="space-y-3 pl-4 border-l-2 border-primary-200 dark:border-primary-800">
          <div>
            <label class="label">API Key</label>
            <input v-model="llmApiKey" @change="saveLlm" type="password" class="input max-w-md" placeholder="sk-..." />
            <p class="text-xs text-gray-400 mt-1">
              兼容 Anthropic Claude API、OpenAI API 等。密钥仅存储在本地数据库中。
            </p>
          </div>
          <div>
            <label class="label">模型</label>
            <input v-model="llmModel" @change="saveLlm" class="input max-w-xs" placeholder="claude-sonnet-4-6" />
            <p class="text-xs text-gray-400 mt-1">支持的模型 ID，如 claude-sonnet-4-6、gpt-4o 等</p>
          </div>
          <div>
            <label class="label">API 端点（可选）</label>
            <input v-model="llmEndpoint" @change="saveLlm" class="input max-w-md" placeholder="https://api.anthropic.com/v1/messages" />
            <p class="text-xs text-gray-400 mt-1">自定义 API 端点，留空使用默认 Anthropic API</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 开机自启 -->
    <div class="card">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">开机自启动</h3>
          <p class="text-sm text-gray-500 dark:text-gray-400 mt-0.5">登录时自动启动 LearnOS</p>
        </div>
        <button @click="toggleAutoStart"
            class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors"
            :class="autoStartEnabled ? 'bg-primary-500' : 'bg-gray-200 dark:bg-gray-700'">
            <span class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform"
              :class="autoStartEnabled ? 'translate-x-6' : 'translate-x-1'" />
          </button>
      </div>
    </div>

    <!-- 数据管理 -->
    <div class="card">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">数据管理</h3>

      <!-- 操作反馈 -->
      <div v-if="dataMsg" class="mb-3 p-3 rounded-lg text-sm"
        :class="dataMsgType === 'success' ? 'bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-400 border border-green-200 dark:border-green-800' : 'bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-400 border border-red-200 dark:border-red-800'">
        {{ dataMsg }}
      </div>

      <!-- 备份与还原 -->
      <div class="mb-4 pb-4 border-b border-gray-200 dark:border-gray-700">
        <p class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">💾 备份与还原</p>
        <div class="flex gap-3">
          <button @click="doBackup" class="btn-secondary text-sm" :disabled="!!dataBusy">
            {{ dataBusy === 'backup' ? '备份中...' : '📤 备份数据' }}
          </button>
          <button @click="doRestore" class="btn-secondary text-sm" :disabled="!!dataBusy">
            {{ dataBusy === 'restore' ? '还原中...' : '📥 还原数据' }}
          </button>
        </div>
        <p class="text-xs text-gray-400 mt-2">备份文件可保存到任意位置；还原数据后应用将自动重启。</p>
      </div>

      <p class="text-sm text-gray-500 dark:text-gray-400 mb-3">
        清空所有数据（包括科目、计划、执行记录、复盘等）。此操作不可撤销！
      </p>
      <button @click="showDeleteConfirm = true" class="btn-danger">清空所有数据</button>
    </div>

    <!-- 确认弹窗 -->
    <div v-if="showDeleteConfirm" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="absolute inset-0 bg-black/50" @click="showDeleteConfirm = false"></div>
      <div class="relative bg-white dark:bg-dark-surface rounded-2xl shadow-xl p-6 w-full max-w-sm mx-4">
        <h3 class="text-lg font-semibold text-red-600 mb-2">⚠ 危险操作</h3>
        <p class="text-gray-500 dark:text-gray-400">
          确定要清空所有数据吗？此操作不可撤销，所有科目、计划、记录将被永久删除。
        </p>
        <p class="mt-2 text-sm text-gray-700 dark:text-gray-300">
          请输入 <strong>DELETE</strong> 确认：
        </p>
        <input v-model="deleteConfirmText" class="input mt-2" placeholder="输入 DELETE" />
        <div class="flex justify-end gap-3 mt-6">
          <button @click="showDeleteConfirm = false" class="btn-secondary">取消</button>
          <button @click="doDeleteAll" class="btn-danger" :disabled="deleteConfirmText !== 'DELETE'">确认清空</button>
        </div>
      </div>
    </div>

    <!-- 还原确认弹窗 -->
    <div v-if="showRestoreConfirm" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="absolute inset-0 bg-black/50" @click="showRestoreConfirm = false"></div>
      <div class="relative bg-white dark:bg-dark-surface rounded-2xl shadow-xl p-6 w-full max-w-sm mx-4">
        <h3 class="text-lg font-semibold text-amber-600 mb-2">⚠ 还原数据</h3>
        <p class="text-gray-500 dark:text-gray-400 text-sm">
          还原数据将<strong>覆盖</strong>当前所有数据，应用将自动重启。建议先备份当前数据。
        </p>
        <p class="text-xs text-gray-400 mt-2">点击确认后将弹出文件选择窗口，请选择之前备份的 .db 文件。</p>
        <div class="flex justify-end gap-3 mt-6">
          <button @click="showRestoreConfirm = false" class="btn-secondary">取消</button>
          <button @click="confirmRestore" class="btn-primary">确认并选择文件</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { isEnabled, enable, disable } from '@tauri-apps/plugin-autostart';
import { save, open } from '@tauri-apps/plugin-dialog';
import { getSettings, updateSettings, deleteAllData, backupDatabase, restoreDatabase } from '../composables/api';
import type { AppSettings } from '../types';

const autoStartEnabled = ref(false);

const settings = ref<AppSettings>({});

const showDeleteConfirm = ref(false);
const showRestoreConfirm = ref(false);
const deleteConfirmText = ref('');

// 备份 / 还原
const dataBusy = ref<'backup' | 'restore' | false>(false);
const dataMsg = ref('');
const dataMsgType = ref<'success' | 'error'>('success');

// LLM 参数
const llmApiKey = ref('');
const llmModel = ref('claude-sonnet-4-6');
const llmEndpoint = ref('');

async function loadSettings() {
  try {
    settings.value = await getSettings();
    if (settings.value.llm_api_key) {
      llmApiKey.value = settings.value.llm_api_key;
    }
    if (settings.value.llm_model) {
      llmModel.value = settings.value.llm_model;
    }
    if (settings.value.llm_endpoint) {
      llmEndpoint.value = settings.value.llm_endpoint;
    }
  } catch (e) {
    console.error('加载设置失败:', e);
  }
}

async function toggleLlm() {
  settings.value.llm_enabled = !settings.value.llm_enabled;
  await saveLlm();
}

async function saveLlm() {
  try {
    await updateSettings({
      llm_enabled: settings.value.llm_enabled,
      llm_api_key: llmApiKey.value || undefined,
      llm_model: llmModel.value || undefined,
      llm_endpoint: llmEndpoint.value || undefined,
    });
  } catch (e) {
    console.error('保存 LLM 配置失败:', e);
  }
}

async function doBackup() {
  dataBusy.value = 'backup';
  dataMsg.value = '';
  try {
    const now = new Date();
    const dateStr = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`;
    const destPath = await save({
      defaultPath: `LearnOS_备份_${dateStr}.db`,
      filters: [{ name: '数据库文件', extensions: ['db'] }],
    });
    if (!destPath) { dataBusy.value = false; return; }
    const msg = await backupDatabase(destPath);
    dataMsg.value = msg;
    dataMsgType.value = 'success';
  } catch (e: any) {
    dataMsg.value = `备份失败: ${e}`;
    dataMsgType.value = 'error';
  } finally {
    dataBusy.value = false;
  }
}

async function doRestore() {
  // 用确认弹窗代替文件选择后的二次确认
  showRestoreConfirm.value = true;
}

async function confirmRestore() {
  dataBusy.value = 'restore';
  dataMsg.value = '';
  showRestoreConfirm.value = false;
  try {
    const selected = await open({
      filters: [{ name: '数据库文件', extensions: ['db'] }],
      multiple: false,
    });
    if (!selected) { dataBusy.value = false; return; }
    await restoreDatabase(selected as string);
  } catch (e: any) {
    dataMsg.value = `还原失败: ${e}`;
    dataMsgType.value = 'error';
    dataBusy.value = false;
  }
}

async function doDeleteAll() {
  if (deleteConfirmText.value !== 'DELETE') return;
  try {
    await deleteAllData();
    showDeleteConfirm.value = false;
    deleteConfirmText.value = '';
    window.location.reload();
  } catch (e) {
    console.error('清空数据失败:', e);
  }
}

async function loadAutoStart() {
  try {
    autoStartEnabled.value = await isEnabled();
  } catch (e) { console.error('加载自启动状态失败:', e); }
}

async function toggleAutoStart() {
  try {
    if (autoStartEnabled.value) {
      await disable();
    } else {
      await enable();
    }
  } catch (e) {
    // 回滚 switch 状态
    autoStartEnabled.value = !autoStartEnabled.value;
    console.error('切换自启动失败:', e);
  }
}

onMounted(() => {
  loadSettings();
  loadAutoStart();
});
</script>
