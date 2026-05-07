# LearnOS — Smart Study Assistant for High School Students

A cross-platform desktop learning app built with **Tauri v2 + Vue 3**, featuring an Ebbinghaus spaced-repetition system, Pomodoro execution tracking, SM-2 knowledge mastery tracking, and intelligent analytics — designed to help high school students plan and review efficiently.

## Features

| Module | Description |
|------|------|
| 🏠 **Dashboard** | Daily check-in, AI-recommended study order, pending reviews & plans overview, weekly goal progress |
| 📚 **Subjects** | Custom subjects (name/icon/color/category), weekly hour & knowledge-point goals |
| 📅 **Study Plans** | Week / month / list views, create plans with linked knowledge points, subject validation |
| ⏱️ **Pomodoro Timer** | 25-minute focus sessions, auto-records execution duration, triggers mastery feedback on completion |
| 🔄 **Ebbinghaus Review** | SM-2 based adaptive interval review board, overdue reminders, heatmap, mastery stats |
| 📊 **Progress** | Knowledge-point mastery milestones, consecutive study days streak, subject goal tracking |
| 🧠 **Analysis** | Weak-subject identification, time distribution, efficiency trends, AI-powered suggestions (optional, bring your own API key) |
| 📝 **Reviews** | Lightweight post-study reflections, mood & energy tracking |
| 📆 **Exam Countdown** | Gaokao / finals / midterms / mocks with visual countdown progress bars |
| ⚙️ **Settings** | Dark mode, AI config (Anthropic / OpenAI compatible), data backup & restore, auto-launch |
| 💬 **NLP Commands** | Built-in rule engine — type "review math chapter 1" to auto-create a review plan (works offline) |

## Tech Stack

| Layer | Technology |
|------|------|
| Desktop Framework | [Tauri v2](https://tauri.app/) (Rust backend) |
| Frontend | Vue 3 (Composition API + `<script setup>`) + TypeScript |
| Styling | Tailwind CSS 3 (dark mode support) |
| State Management | Pinia 3 (`pinia-plugin-persistedstate` persistence) |
| Charts | Chart.js 4 + vue-chartjs |
| Database | SQLite (Rust `sqlx` 0.8 + Tauri SQL Plugin) |
| NLP | Pure rule-engine based on `regex-lite` (works offline) |
| AI (optional) | Direct Anthropic / OpenAI-compatible API calls from frontend |

## Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) ≥ 18
- [Rust](https://www.rust-lang.org/tools/install) ≥ 1.70
- Windows / macOS / Linux

### Install Dependencies

```bash
# Frontend dependencies
npm install

# Rust dependencies are auto-downloaded on first build
```

### Development

```bash
npm run tauri:dev
```

The frontend dev server starts at `http://localhost:1420`, and the Tauri window opens automatically.

### Production Build

```bash
npm run tauri:build
```

Build artifacts are located in `src-tauri/target/release/bundle/`:
- Windows: `.msi` / `.exe` (NSIS installer)
- macOS: `.dmg`
- Linux: `.AppImage`

## Project Structure

```
LearnOS-Tauri-V2/
├── src/                        # Vue frontend source
│   ├── main.ts                 # Entry point
│   ├── App.vue                 # Root component (sidebar + header + router)
│   ├── router/index.ts         # Route config (14 routes)
│   ├── stores/appStore.ts      # Global state (dark mode / sidebar)
│   ├── composables/
│   │   ├── api.ts              # Typed Tauri invoke wrappers
│   │   └── llm.ts              # LLM API integration
│   ├── types/index.ts          # TypeScript type definitions
│   ├── styles/main.css         # Global styles
│   ├── components/
│   │   ├── layout/             # Layout components (sidebar / header)
│   │   └── home/               # Dashboard components (Pomodoro / mastery card)
│   └── pages/                  # 14 page components
├── src-tauri/                  # Tauri Rust backend source
│   ├── Cargo.toml              # Rust dependencies
│   ├── tauri.conf.json         # Tauri configuration
│   ├── capabilities/           # Permission policies
│   ├── migrations/             # SQLite database migrations (5 files)
│   └── src/
│       ├── main.rs             # Entry point
│       ├── lib.rs              # Command registration + plugin setup
│       ├── models/             # Entities & DTOs
│       ├── commands/           # Tauri command layer (13 modules)
│       ├── services/           # Business logic layer (13 modules)
│       ├── db/repositories/    # Data access layer (9 modules)
│       ├── errors/             # Error handling
│       └── state/              # Global state
├── index.html
├── package.json
├── vite.config.ts
├── tailwind.config.js
└── tsconfig.json
```

## Architecture

Classic three-tier architecture with frontend-backend communication via Tauri IPC:

```
Vue Pages (pages/)
  → Composables (api.ts) via invoke()
    → Tauri Commands (commands/)
      → Services (services/)
        → Repositories (db/repositories/)
          → SQLite Database
```

- **NLP Engine**: Pure rule matching, no external dependencies, works offline
- **SM-2 Algorithm**: Dynamically adjusts review intervals based on mastery scores, with configurable intervals (1/3/7/15/30 days)
- **AI Analysis**: Optional — sends aggregated statistics (no private notes) to an LLM for deep insights

## Database

Application data is stored in `learnos_v2.db` (SQLite) under the user's app data directory, with 11 core tables:

`subjects` · `plans` · `knowledge_points` · `review_sessions` · `executions` · `reviews` · `insights` · `settings` · `exams` · `daily_logs` · `records`

Backup and restore via **Settings → Data Management**.

## License

MIT

---

**Author**: John Kuang  
**Version**: 0.2.0
