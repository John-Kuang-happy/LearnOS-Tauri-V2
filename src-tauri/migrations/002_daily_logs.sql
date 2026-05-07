-- daily_logs: 每日学习日志
CREATE TABLE IF NOT EXISTS daily_logs (
    id TEXT PRIMARY KEY,
    date TEXT NOT NULL UNIQUE,
    total_minutes INTEGER DEFAULT 0,
    completed_plans INTEGER DEFAULT 0,
    review_count INTEGER DEFAULT 0,
    summary_text TEXT DEFAULT '',
    mood_score INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
