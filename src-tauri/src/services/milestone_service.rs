use std::collections::HashSet;
use sqlx::{SqlitePool, Row};
use chrono::{Utc, Datelike};
use crate::errors::Result;
use crate::models::dto::StreakData;
use crate::models::entity::Insight;
use crate::db::repositories::insight_repo::InsightRepository;

/// 获取里程碑列表（自动生成 + 返回已有）
pub async fn get_milestones(pool: &SqlitePool, limit: i64) -> Result<Vec<Insight>> {
    generate_milestones(pool).await?;

    let now = Utc::now().timestamp();
    let milestones = sqlx::query_as::<_, Insight>(
        "SELECT * FROM insights WHERE insight_type = 'milestone' AND (expires_at IS NULL OR expires_at > ?) ORDER BY generated_at DESC LIMIT ?"
    ).bind(now).bind(limit).fetch_all(pool).await?;

    Ok(milestones)
}

/// 获取连续学习数据
pub async fn get_streak_data(pool: &SqlitePool) -> Result<StreakData> {
    let now = Utc::now();

    let study_dates: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT DATE(start_time, 'unixepoch') FROM executions WHERE end_time IS NOT NULL ORDER BY 1 DESC"
    ).fetch_all(pool).await?;

    let total_study_days = study_dates.len() as i32;

    // --- 当前连续天数 ---
    let mut current_streak = 0i32;
    let has_today = study_dates.iter().any(|d| d == &now.format("%Y-%m-%d").to_string());
    let start_offset: i64 = if has_today { 0 } else { 1 };
    let max_check = study_dates.len() as i64;

    for offset in start_offset..=max_check {
        let date_str = (now - chrono::Duration::days(offset)).format("%Y-%m-%d").to_string();
        if study_dates.iter().any(|d| d == &date_str) {
            current_streak += 1;
        } else {
            break;
        }
    }

    // --- 最长连续天数 ---
    let mut sorted: Vec<chrono::NaiveDate> = study_dates.iter()
        .filter_map(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .collect();
    sorted.sort();
    sorted.dedup();

    let longest_streak = if sorted.is_empty() {
        0
    } else {
        let mut max_len = 1i32;
        let mut cur_len = 1i32;
        for i in 1..sorted.len() {
            let diff = sorted[i].signed_duration_since(sorted[i - 1]).num_days();
            if diff == 1 {
                cur_len += 1;
            } else {
                max_len = max_len.max(cur_len);
                cur_len = 1;
            }
        }
        max_len.max(cur_len)
    };

    // --- 本周打卡网格（周一 ~ 周日） ---
    let days_from_monday = now.weekday().num_days_from_monday() as i64;
    let monday = now - chrono::Duration::days(days_from_monday);
    let weekly_days: Vec<bool> = (0..7).map(|i| {
        let date_str = (monday + chrono::Duration::days(i)).format("%Y-%m-%d").to_string();
        study_dates.iter().any(|d| d == &date_str)
    }).collect();

    Ok(StreakData { current_streak, longest_streak, weekly_days, total_study_days })
}

/// 生成里程碑（6 条规则）
///
/// 幂等设计：先删同标题旧记录再插新记录，消除竞态导致的重复。
async fn generate_milestones(pool: &SqlitePool) -> Result<()> {
    let now = Utc::now().timestamp();

    // ---- Rule 1: 掌握度跃迁 ----
    // 只对「首次掌握」生成永久里程碑，不是时间窗口扫描，避免 updated_at 被其他操作刷新导致误触发

    let all_mastered = sqlx::query(
        "SELECT id, name, subject_id FROM knowledge_points WHERE is_mastered = 1"
    ).fetch_all(pool).await?;

    // 读取已有掌握里程碑的 metadata，提取已记录过的 kp_id
    let existing_meta: Vec<String> = sqlx::query_scalar(
        "SELECT metadata FROM insights WHERE insight_type = 'milestone' AND metadata LIKE '%mastery_jump%'"
    ).fetch_all(pool).await?;

    let existing_kp_ids: HashSet<String> = existing_meta.iter()
        .filter_map(|m| {
            serde_json::from_str::<serde_json::Value>(m).ok()
                .and_then(|v| v.get("kp_id")?.as_str().map(String::from))
        })
        .collect();

    for row in &all_mastered {
        let kp_id: String = row.get("id");
        if existing_kp_ids.contains(&kp_id) { continue; }

        let name: String = row.get("name");
        let subject_id: String = row.get("subject_id");
        let title = format!("🎯 掌握「{}」", name);
        let subj_name = get_subject_name(pool, &subject_id).await;
        upsert_permanent_milestone(pool, &title,
            &format!("知识点「{}」已掌握（{}）", name, subj_name),
            Some(subject_id.as_str()), "success",
            &format!(r#"{{"rule":"mastery_jump","kp_id":"{}"}}"#, kp_id),
        ).await?;
    }

    // ---- Rule 2: 科目全部掌握 ----
    let subjects_complete = sqlx::query(
        r#"SELECT s.id, s.name FROM subjects s
           WHERE (SELECT COUNT(*) FROM knowledge_points WHERE subject_id = s.id) > 0
           AND (SELECT COUNT(*) FROM knowledge_points WHERE subject_id = s.id AND is_mastered = 0) = 0"#
    ).fetch_all(pool).await?;

    for row in &subjects_complete {
        let id: String = row.get("id");
        let name: String = row.get("name");
        let title = format!("🏆 全部掌握「{}」", name);
        upsert_milestone(pool, &title,
            &format!("恭喜！「{}」的所有知识点已全部掌握", name),
            Some(id.as_str()), "success",
            &format!(r#"{{"rule":"subject_complete","subject_id":"{}"}}"#, id),
        ).await?;
    }

    // ---- Rule 3: 连续学习里程碑 ----
    let streak = get_streak_data(pool).await?;
    for &threshold in &[7, 14, 21, 30] {
        if streak.current_streak >= threshold {
            let title = format!("🔥 连续学习 {} 天", threshold);
            upsert_milestone(pool, &title,
                &format!("你已经连续学习了 {} 天，太棒了！继续保持！", threshold),
                None, "success",
                &format!(r#"{{"rule":"streak","days":{}}}"#, threshold),
            ).await?;
        }
    }

    // ---- Rule 4: 单科目完成计划数 ----
    let plan_counts = sqlx::query(
        "SELECT p.subject_id, COUNT(*) as cnt FROM plans p WHERE p.status = 'completed' AND COALESCE(p.plan_type, 'normal') != 'review' GROUP BY p.subject_id"
    ).fetch_all(pool).await?;

    for row in &plan_counts {
        let subject_id: String = row.get("subject_id");
        let count: i64 = row.get("cnt");
        let subj_name = get_subject_name(pool, &subject_id).await;

        for &threshold in &[5, 10, 20, 50] {
            if count >= threshold {
                let meta = format!(r#"{{"rule":"batch_complete","subject_id":"{}","threshold":{}}}"#, subject_id, threshold);
                // 先删同规则同科目同阈值的旧里程碑（处理科目改名）
                sqlx::query("DELETE FROM insights WHERE insight_type = 'milestone' AND metadata = ?")
                    .bind(&meta).execute(pool).await?;
                let title = format!("📚 {} 完成 {} 个计划", subj_name, threshold);
                InsightRepository::create_permanent(pool, "milestone", &title,
                    &format!("在「{}」中已完成 {} 个学习计划，继续加油！", subj_name, threshold),
                    Some(subject_id.as_str()), "success",
                    &meta,
                ).await?;
            }
        }
    }

    // ---- Rule 5: 总学习时长里程碑 ----
    let total_hours: f64 = sqlx::query_scalar::<_, f64>(
        "SELECT COALESCE(SUM(COALESCE(actual_hours, 0)), 0) FROM executions WHERE end_time IS NOT NULL"
    ).fetch_one(pool).await?;

    for &threshold in &[50, 100, 200, 500] {
        if total_hours >= threshold as f64 {
            let title = format!("⏱️ 累计学习 {} 小时", threshold);
            upsert_milestone(pool, &title,
                &format!("累计学习时长突破 {} 小时，每一步都算数！", threshold),
                None, "success",
                &format!(r#"{{"rule":"total_hours","hours":{},"threshold":{}}}"#, total_hours, threshold),
            ).await?;
        }
    }

    Ok(())
}

/// 幂等创建：删除同标题旧记录后再插入，避免竞态重复。
async fn upsert_milestone(
    pool: &SqlitePool, title: &str, content: &str,
    related_subject_id: Option<&str>, severity: &str, metadata: &str,
) -> Result<Insight> {
    sqlx::query("DELETE FROM insights WHERE insight_type = 'milestone' AND title = ?")
        .bind(title).execute(pool).await?;
    InsightRepository::create_permanent(pool, "milestone", title, content, related_subject_id, severity, metadata).await
}

/// 创建永不过期的成就类里程碑（用于掌握度跃迁等一次性成就）
async fn upsert_permanent_milestone(
    pool: &SqlitePool, title: &str, content: &str,
    related_subject_id: Option<&str>, severity: &str, metadata: &str,
) -> Result<Insight> {
    sqlx::query("DELETE FROM insights WHERE insight_type = 'milestone' AND title = ?")
        .bind(title).execute(pool).await?;
    InsightRepository::create_permanent(pool, "milestone", title, content, related_subject_id, severity, metadata).await
}

async fn get_subject_name(pool: &SqlitePool, subject_id: &str) -> String {
    sqlx::query_scalar::<_, String>("SELECT COALESCE(name, '未归类') FROM subjects WHERE id = ?")
        .bind(subject_id).fetch_optional(pool).await
        .ok().flatten().unwrap_or_else(|| "未归类".to_string())
}
