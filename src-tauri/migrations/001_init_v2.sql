-- ============================================================
-- LearnOS V2 数据库初始化迁移
-- 面向高中生智能学习助手
-- 表数量：10 张业务表 + 1 张设置表
-- ============================================================

-- 1. 科目表
CREATE TABLE IF NOT EXISTS subjects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL DEFAULT '#0ea5e9',
    icon TEXT DEFAULT '📚',
    category TEXT NOT NULL DEFAULT 'other',       -- 'liberal_arts' | 'science' | 'other'
    weekly_goal_hours REAL NOT NULL DEFAULT 10.0,
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 2. 考试/截止日期表
CREATE TABLE IF NOT EXISTS exams (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    exam_type TEXT NOT NULL DEFAULT 'final',      -- 'gaokao' | 'final' | 'midterm' | 'mock'
    target_date INTEGER NOT NULL,
    remarks TEXT DEFAULT '',
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 3. 考试-科目关联表
CREATE TABLE IF NOT EXISTS exam_subjects (
    exam_id TEXT NOT NULL REFERENCES exams(id) ON DELETE CASCADE,
    subject_id TEXT NOT NULL REFERENCES subjects(id) ON DELETE CASCADE,
    target_score REAL,
    current_score REAL,
    PRIMARY KEY (exam_id, subject_id)
);

-- 4. 学习计划表（增强版）
CREATE TABLE IF NOT EXISTS plans (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    subject_id TEXT NOT NULL REFERENCES subjects(id) ON DELETE CASCADE,
    plan_type TEXT NOT NULL DEFAULT 'normal',     -- 'normal' | 'review' | 'homework' | 'exam_prep'
    source_plan_id TEXT,                          -- FK to self：复习计划的原始计划
    source_kp_id TEXT,                            -- FK to knowledge_points：关联知识点
    priority INTEGER NOT NULL DEFAULT 2,          -- 1=紧急 2=高 3=中 4=低
    status TEXT NOT NULL DEFAULT 'pending',       -- 'pending'|'in_progress'|'completed'|'paused'
    start_date INTEGER NOT NULL,
    end_date INTEGER NOT NULL,
    estimated_hours REAL NOT NULL DEFAULT 0.0,
    time_slot TEXT DEFAULT 'any',                 -- 'morning'|'afternoon'|'evening'|'night'|'any'
    auto_review_enabled INTEGER NOT NULL DEFAULT 0,
    review_rule TEXT,                             -- JSON: [1,3,7,15,30]
    tags TEXT,                                    -- JSON 字符串数组
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (source_plan_id) REFERENCES plans(id) ON DELETE SET NULL
);

-- 5. 知识点表（艾宾浩斯复习核心）
CREATE TABLE IF NOT EXISTS knowledge_points (
    id TEXT PRIMARY KEY,
    subject_id TEXT NOT NULL REFERENCES subjects(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT DEFAULT '',
    difficulty_level INTEGER NOT NULL DEFAULT 3,   -- 1-5 难度
    importance_level INTEGER NOT NULL DEFAULT 3,   -- 1-5 重要度（考试相关度）
    mastery_level REAL NOT NULL DEFAULT 0.0,       -- 0.0-1.0 当前掌握度
    last_studied_at INTEGER,
    last_reviewed_at INTEGER,
    next_review_at INTEGER,
    review_count INTEGER NOT NULL DEFAULT 0,
    consecutive_correct INTEGER NOT NULL DEFAULT 0, -- 连续正确次数
    review_interval_days INTEGER NOT NULL DEFAULT 1, -- 当前复习间隔（天）
    ease_factor REAL NOT NULL DEFAULT 2.5,          -- SM-2 易度因子
    review_history TEXT DEFAULT '[]',               -- JSON：[{date, score, interval}]
    is_mastered INTEGER NOT NULL DEFAULT 0,         -- 0=学习中 1=已掌握
    source TEXT DEFAULT '',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 6. 复习记录表
CREATE TABLE IF NOT EXISTS review_sessions (
    id TEXT PRIMARY KEY,
    knowledge_point_id TEXT NOT NULL REFERENCES knowledge_points(id) ON DELETE CASCADE,
    plan_id TEXT REFERENCES plans(id) ON DELETE SET NULL,
    scheduled_date INTEGER NOT NULL,
    actual_date INTEGER,
    mastery_score REAL,                             -- 0.0-1.0 复习后自评掌握度
    time_spent_seconds INTEGER DEFAULT 0,
    feedback TEXT DEFAULT '',
    was_skipped INTEGER NOT NULL DEFAULT 0,         -- 0=已完成 1=跳过
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 7. 执行记录表（番茄钟）
CREATE TABLE IF NOT EXISTS executions (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    start_time INTEGER NOT NULL,
    end_time INTEGER,
    actual_hours REAL,
    completion_rate REAL,
    notes TEXT,
    pomodoro_count INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 8. 学习记录表
CREATE TABLE IF NOT EXISTS records (
    id TEXT PRIMARY KEY,
    execution_id TEXT NOT NULL REFERENCES executions(id) ON DELETE CASCADE,
    content_type TEXT NOT NULL,
    content_summary TEXT NOT NULL,
    difficulty_level INTEGER NOT NULL DEFAULT 3,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 9. 复盘表
CREATE TABLE IF NOT EXISTS reviews (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    review_date INTEGER NOT NULL,
    what_went_well TEXT NOT NULL DEFAULT '',
    what_to_improve TEXT NOT NULL DEFAULT '',
    action_items TEXT NOT NULL DEFAULT '',
    mood_score INTEGER NOT NULL DEFAULT 3,
    energy_level INTEGER NOT NULL DEFAULT 3,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 10. 智能分析缓存表
CREATE TABLE IF NOT EXISTS insights (
    id TEXT PRIMARY KEY,
    insight_type TEXT NOT NULL,                   -- 'weak_subject'|'efficiency_trend'|'review_reminder'|'suggestion'
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    related_subject_id TEXT REFERENCES subjects(id) ON DELETE SET NULL,
    severity TEXT DEFAULT 'info',                  -- 'info'|'warning'|'critical'|'success'
    is_read INTEGER NOT NULL DEFAULT 0,
    generated_at INTEGER NOT NULL,
    expires_at INTEGER,
    metadata TEXT DEFAULT '{}',                    -- JSON 额外数据
    created_at INTEGER NOT NULL
);

-- 11. 应用设置表
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- ============================================================
-- 索引
-- ============================================================
CREATE INDEX IF NOT EXISTS idx_subjects_category ON subjects(category);
CREATE INDEX IF NOT EXISTS idx_subjects_active ON subjects(is_active);
CREATE INDEX IF NOT EXISTS idx_exams_target_date ON exams(target_date);
CREATE INDEX IF NOT EXISTS idx_exams_active ON exams(is_active);
CREATE INDEX IF NOT EXISTS idx_plans_subject_id ON plans(subject_id);
CREATE INDEX IF NOT EXISTS idx_plans_status ON plans(status);
CREATE INDEX IF NOT EXISTS idx_plans_priority ON plans(priority);
CREATE INDEX IF NOT EXISTS idx_plans_type ON plans(plan_type);
CREATE INDEX IF NOT EXISTS idx_plans_source ON plans(source_plan_id);
CREATE INDEX IF NOT EXISTS idx_plans_source_kp ON plans(source_kp_id);
CREATE INDEX IF NOT EXISTS idx_plans_start_date ON plans(start_date);
CREATE INDEX IF NOT EXISTS idx_kp_subject_id ON knowledge_points(subject_id);
CREATE INDEX IF NOT EXISTS idx_kp_next_review ON knowledge_points(next_review_at);
CREATE INDEX IF NOT EXISTS idx_kp_mastery ON knowledge_points(mastery_level);
CREATE INDEX IF NOT EXISTS idx_kp_mastered ON knowledge_points(is_mastered);
CREATE INDEX IF NOT EXISTS idx_review_sessions_kp ON review_sessions(knowledge_point_id);
CREATE INDEX IF NOT EXISTS idx_review_sessions_scheduled ON review_sessions(scheduled_date);
CREATE INDEX IF NOT EXISTS idx_review_sessions_plan ON review_sessions(plan_id);
CREATE INDEX IF NOT EXISTS idx_executions_plan_id ON executions(plan_id);
CREATE INDEX IF NOT EXISTS idx_executions_start_time ON executions(start_time);
CREATE INDEX IF NOT EXISTS idx_records_execution_id ON records(execution_id);
CREATE INDEX IF NOT EXISTS idx_reviews_plan_id ON reviews(plan_id);
CREATE INDEX IF NOT EXISTS idx_reviews_review_date ON reviews(review_date);
CREATE INDEX IF NOT EXISTS idx_insights_type ON insights(insight_type);
CREATE INDEX IF NOT EXISTS idx_insights_generated ON insights(generated_at);
CREATE INDEX IF NOT EXISTS idx_insights_read ON insights(is_read);

-- ============================================================
-- 默认设置
-- ============================================================
INSERT OR IGNORE INTO settings (key, value, created_at, updated_at) VALUES
    ('review_enabled', 'true', strftime('%s', 'now'), strftime('%s', 'now')),
    ('review_presets', '[1, 3, 7, 15, 30]', strftime('%s', 'now'), strftime('%s', 'now')),
    ('review_prefix', '复习：', strftime('%s', 'now'), strftime('%s', 'now')),
    ('ebbinghaus_default_intervals', '[1, 3, 7, 15, 30]', strftime('%s', 'now'), strftime('%s', 'now')),
    ('ebbinghaus_mastery_threshold', '0.8', strftime('%s', 'now'), strftime('%s', 'now')),
    ('analysis_weeks_to_analyze', '4', strftime('%s', 'now'), strftime('%s', 'now')),
    ('analysis_weak_subject_threshold', '0.6', strftime('%s', 'now'), strftime('%s', 'now')),
    ('gaokao_date', '', strftime('%s', 'now'), strftime('%s', 'now')),
    ('theme', 'system', strftime('%s', 'now'), strftime('%s', 'now')),
    ('llm_enabled', 'false', strftime('%s', 'now'), strftime('%s', 'now')),
    ('llm_api_key', '', strftime('%s', 'now'), strftime('%s', 'now')),
    ('llm_model', 'claude-sonnet-4-6', strftime('%s', 'now'), strftime('%s', 'now'));
