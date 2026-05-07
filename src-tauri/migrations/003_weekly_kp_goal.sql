-- 003: 添加每周知识点目标字段
-- 支持方案B：时间+知识点复合目标统计

ALTER TABLE subjects ADD COLUMN weekly_goal_kps INTEGER NOT NULL DEFAULT 3;
