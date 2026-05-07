# LearnOS —— 高中生智能学习助手

基于 **Tauri v2 + Vue 3** 构建的跨平台桌面学习应用，集成艾宾浩斯遗忘曲线复习系统、番茄钟执行记录、SM-2 知识点掌握度追踪与智能分析，帮助高中生科学规划、高效复习。

## 功能概览

| 模块 | 说明 |
|------|------|
| 🏠 **首页看板** | 每日签到、今日推荐学习顺序、待复习 + 待办计划一览、周目标达成概览 |
| 📚 **科目管理** | 自定义科目（名称/图标/颜色/分类）、设定每周学习时长与知识点目标 |
| 📅 **学习计划** | 周视图 / 月视图 / 列表视图，新建计划支持关联知识点，自动校验科目归属 |
| ⏱️ **番茄钟执行** | 25 分钟专注学习，结束后自动记录执行时长，联动知识点掌握度反馈 |
| 🔄 **艾宾浩斯复习** | 基于 SM-2 算法的自适应间隔复习看板，逾期提醒、热力图、掌握度统计 |
| 📊 **学习进度** | 知识点掌握里程碑、连续学习天数、科目目标进度 |
| 🧠 **智能分析** | 薄弱科目识别、学习时长分布、效率趋势、AI 学习建议（可选，需自行提供 API Key） |
| 📝 **复盘记录** | 学习后轻量复盘，记录心得、评价状态 |
| 📆 **考试倒计时** | 高考/期末/期中/模拟考，可视化倒计时进度条 |
| ⚙️ **设置** | 暗黑模式、AI 配置（兼容 Anthropic / OpenAI API）、数据备份与还原、开机自启 |
| 💬 **NLP 自然语言** | 内置规则引擎，输入「复习数学第一章」即可自动创建复习计划（无需联网） |

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面框架 | [Tauri v2](https://tauri.app/)（Rust 后端） |
| 前端 | Vue 3（Composition API + `<script setup>`）+ TypeScript |
| 样式 | Tailwind CSS 3（暗黑模式支持） |
| 状态管理 | Pinia 3（`pinia-plugin-persistedstate` 持久化） |
| 图表 | Chart.js 4 + vue-chartjs |
| 数据库 | SQLite（Rust 端 `sqlx` 0.8 + Tauri SQL Plugin） |
| NLP | 基于 `regex-lite` 的纯规则引擎（离线可用） |
| AI（可选） | 前端直连 Anthropic / OpenAI 兼容 API |

## 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) ≥ 18
- [Rust](https://www.rust-lang.org/tools/install) ≥ 1.70
- Windows / macOS / Linux

### 安装依赖

```bash
# 前端依赖
npm install

# Rust 依赖（首次启动时自动下载编译）
```

### 开发模式

```bash
npm run tauri:dev
```

启动后前端运行在 `http://localhost:1420`，Tauri 窗口自动打开。

### 生产构建

```bash
npm run tauri:build
```

构建产物位于 `src-tauri/target/release/bundle/`：
- Windows：`.msi` / `.exe`（NSIS 安装包）
- macOS：`.dmg`
- Linux：`.AppImage`

## 项目结构

```
LearnOS-Tauri-V2/
├── src/                        # Vue 前端源码
│   ├── main.ts                 # 入口
│   ├── App.vue                 # 根组件（侧边栏 + 顶栏 + 路由）
│   ├── router/index.ts         # 路由配置
│   ├── stores/appStore.ts      # 全局状态（暗黑模式 / 侧边栏）
│   ├── composables/
│   │   ├── api.ts              # 后端命令 TS 封装（invoke 调用）
│   │   └── llm.ts              # LLM API 调用
│   ├── types/index.ts          # TypeScript 类型定义
│   ├── styles/main.css         # 全局样式
│   ├── components/
│   │   ├── layout/             # 布局组件（侧边栏 / 顶栏）
│   │   └── home/               # 首页组件（番茄钟 / 掌握度卡片）
│   └── pages/                  # 14 个页面组件
├── src-tauri/                  # Tauri Rust 后端源码
│   ├── Cargo.toml              # Rust 依赖
│   ├── tauri.conf.json         # Tauri 配置
│   ├── capabilities/           # 权限配置
│   ├── migrations/             # SQLite 数据库迁移（5 个）
│   └── src/
│       ├── main.rs             # 入口
│       ├── lib.rs              # 命令注册 + 插件加载
│       ├── models/             # 实体 & DTO
│       ├── commands/           # Tauri 命令层（13 个模块）
│       ├── services/           # 业务逻辑层（13 个模块）
│       ├── db/repositories/    # 数据访问层（9 个模块）
│       ├── errors/             # 错误处理
│       └── state/              # 全局状态
├── index.html                  # HTML 入口
├── package.json
├── vite.config.ts
├── tailwind.config.js
└── tsconfig.json
```

## 架构说明

采用经典三层架构，前后端通过 Tauri IPC 通信：

```
Vue 页面 (pages/)
  → Composables (api.ts) 通过 invoke() 调用
    → Tauri Commands (commands/)
      → Services (services/)
        → Repositories (db/repositories/)
          → SQLite 数据库
```

- **NLP 引擎**：纯规则匹配，无外部依赖，离线可用
- **SM-2 算法**：根据掌握度评分动态调整复习间隔，支持 1/3/7/15/30 天间隔配置
- **AI 分析**：可选功能，发送聚合统计数据（不含隐私笔记）到 LLM 获取深度建议

## 数据库

应用数据存储在用户数据目录下的 `learnos_v2.db` SQLite 文件中，包含 11 张核心表：

`subjects` · `plans` · `knowledge_points` · `review_sessions` · `executions` · `reviews` · `insights` · `settings` · `exams` · `daily_logs` · `records`

备份与还原功能通过「设置 → 数据管理」操作。

## License

MIT

---

**作者**：John Kuang  
**版本**：0.2.0
