-- 004: 添加 mastered_at 字段，精确记录知识点首次掌握时间
-- 替代 updated_at 作为「本周新掌握」的统计依据

ALTER TABLE knowledge_points ADD COLUMN mastered_at INTEGER;

-- 回填已有数据：已掌握的知识点以 updated_at 作为近似 mastered_at
UPDATE knowledge_points SET mastered_at = updated_at WHERE is_mastered = 1;
