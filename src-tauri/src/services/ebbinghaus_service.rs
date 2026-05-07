use sqlx::SqlitePool;
use crate::errors::Result;
use crate::models::entity::{KnowledgePoint, ReviewSession};
use crate::models::dto::{ReviewSessionWithKp, ReviewStats, SubmitReviewFeedbackRequest};
use crate::db::repositories::knowledge_point_repo::KnowledgePointRepository;
use crate::db::repositories::review_session_repo::ReviewSessionRepository;

/// SM-2 简化版自适应间隔计算（不含难度参数）
///
/// 参数：
/// - current_interval: 当前复习间隔（天）
/// - consecutive_correct: 连续正确次数
/// - mastery_score: 本次复习掌握评分 (0.0-1.0)
///
/// 返回：(新间隔天数, 新易度因子, 新连续正确次数, 是否已掌握, 新掌握度)
fn calculate_next_review(
    current_interval: i32,
    consecutive_correct: i32,
    mastery_score: f64,
) -> (i32, f64, i32, bool, f64) {
    let mut new_consecutive = consecutive_correct;

    let new_interval = if mastery_score >= 0.9 {
        let ef = 2.5 + 0.1 * (consecutive_correct as f64);
        let ef = ef.clamp(1.3, 3.0);
        new_consecutive += 1;
        std::cmp::max(1, ((current_interval as f64) * ef).round() as i32)
    } else if mastery_score >= 0.5 {
        new_consecutive += 1;
        std::cmp::max(1, ((current_interval as f64) * 0.7).round() as i32)
    } else {
        new_consecutive = 0;
        std::cmp::max(1, ((current_interval as f64) * 0.3).round() as i32)
    };

    let new_ef = {
        let ef = 2.5 + 0.1 * (new_consecutive as f64);
        ef.clamp(1.3, 3.0)
    };

    let is_mastered = new_consecutive >= 3 && mastery_score >= 0.9;
    let new_mastery = mastery_score;

    (new_interval, new_ef, new_consecutive, is_mastered, new_mastery)
}

/// 提交复习反馈：更新复习记录 + 触发 SM-2 算法更新知识点掌握度
pub async fn submit_review_feedback(
    pool: &SqlitePool, req: SubmitReviewFeedbackRequest,
) -> Result<(ReviewSession, KnowledgePoint)> {
    let session = ReviewSessionRepository::find_by_id(pool, &req.session_id).await?;
    let kp = KnowledgePointRepository::find_by_id(pool, &session.knowledge_point_id).await?;

    let time_spent = req.time_spent_seconds.unwrap_or(0);
    let feedback = req.feedback.unwrap_or_default();

    let updated_session = ReviewSessionRepository::submit_feedback(
        pool, &req.session_id, req.mastery_score, time_spent, &feedback,
    ).await?;

    let (new_interval, new_ef, new_consecutive, is_mastered, new_mastery) = calculate_next_review(
        kp.review_interval_days, kp.consecutive_correct,
        req.mastery_score,
    );

    let next_review_at = chrono::Utc::now().timestamp() + (new_interval as i64) * 86400;

    let updated_kp = KnowledgePointRepository::update_mastery(
        pool, &kp.id, new_mastery, new_interval as i64, new_ef,
        new_consecutive, next_review_at, is_mastered,
    ).await?;

    // 未掌握时自动创建下一次复习记录
    if !is_mastered {
        ReviewSessionRepository::create(
            pool, &kp.id, next_review_at, None::<&str>,
        ).await?;
    }

    Ok((updated_session, updated_kp))
}

/// 获取今日到期复习清单
pub async fn get_due_reviews(pool: &SqlitePool) -> Result<Vec<ReviewSessionWithKp>> {
    ReviewSessionRepository::find_due(pool).await
}

/// 获取未来 N 天待复习清单
pub async fn get_upcoming_reviews(pool: &SqlitePool, days: i64) -> Result<Vec<ReviewSessionWithKp>> {
    ReviewSessionRepository::find_upcoming(pool, days).await
}

/// 获取知识点复习历史
pub async fn get_kp_review_history(pool: &SqlitePool, kp_id: &str) -> Result<Vec<ReviewSession>> {
    ReviewSessionRepository::find_by_kp_id(pool, kp_id).await
}

/// 跳过复习
pub async fn skip_review_session(pool: &SqlitePool, session_id: &str) -> Result<ReviewSession> {
    ReviewSessionRepository::skip(pool, session_id).await
}

/// 获取复习统计数据
pub async fn get_review_stats(pool: &SqlitePool) -> Result<ReviewStats> {
    let (due_today, due_this_week, completed_this_week, skipped_this_week, avg_mastery_score) =
        ReviewSessionRepository::get_stats(pool).await?;

    let total_kps = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM knowledge_points"
    ).fetch_one(pool).await?;

    let mastered_kps = KnowledgePointRepository::count_mastered(pool).await?;

    Ok(ReviewStats {
        due_today, due_this_week, completed_this_week, skipped_this_week,
        avg_mastery_score, total_kps, mastered_kps,
    })
}

/// 知识点掌握后为其生成首次复习记录（已有待处理会话则跳过）
pub async fn generate_initial_review(pool: &SqlitePool, kp: &KnowledgePoint) -> Result<ReviewSession> {
    // 检查是否已有未完成的复习会话
    let existing: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM review_sessions WHERE knowledge_point_id = ? AND actual_date IS NULL AND was_skipped = 0"
    ).bind(&kp.id).fetch_one(pool).await?;
    if existing > 0 {
        let existing_session = sqlx::query_as::<_, ReviewSession>(
            "SELECT * FROM review_sessions WHERE knowledge_point_id = ? AND actual_date IS NULL AND was_skipped = 0 ORDER BY scheduled_date ASC LIMIT 1"
        ).bind(&kp.id).fetch_one(pool).await?;
        return Ok(existing_session);
    }
    let first_review_at = chrono::Utc::now().timestamp() + 86400; // 1 天后
    ReviewSessionRepository::create(pool, &kp.id, first_review_at, None::<&str>).await
}
