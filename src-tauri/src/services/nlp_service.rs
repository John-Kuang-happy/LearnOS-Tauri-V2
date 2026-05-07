use sqlx::{SqlitePool, Row};
use std::sync::OnceLock;
use crate::errors::{AppError, Result};
use crate::models::dto::*;

/// 预编译正则，避免每次调用重新编译
static HOUR_RE: OnceLock<regex_lite::Regex> = OnceLock::new();
static MIN_RE: OnceLock<regex_lite::Regex> = OnceLock::new();
static POMO_RE: OnceLock<regex_lite::Regex> = OnceLock::new();
static TIME_RE: OnceLock<regex_lite::Regex> = OnceLock::new();
static ACTION_RE: OnceLock<regex_lite::Regex> = OnceLock::new();
static CHAPTER_RE: OnceLock<regex_lite::Regex> = OnceLock::new();

/// 意图类型
#[derive(Debug, Clone)]
pub enum ParsedIntent {
    StartStudy {
        subject_id: Option<String>,
        subject_name: Option<String>,
        title: String,
        duration_min: Option<i32>,
    },
    StopStudy {
        feedback: Option<String>,
        mastery_label: Option<String>,
    },
    LogFeedback {
        subject_hint: Option<String>,
        sentiment: f64,
        notes: String,
    },
    QueryStatus,
    QuerySuggestion,
    QuickPlan {
        subject_id: Option<String>,
        subject_name: Option<String>,
        title: String,
        date_hint: Option<String>,
    },
    QuickReview {
        mood: Option<i32>,
        notes: String,
    },
    Unknown {
        raw: String,
    },
}

/// 情感词 → 掌握度映射
fn sentiment_to_mastery(word: &str) -> Option<f64> {
    match word {
        "掌握" | "熟练" | "完全掌握" | "很好" | "非常好" | "很熟" => Some(0.9),
        "不错" | "还行" | "可以" | "基本掌握" | "还好" | "还可以" | "挺好的" => Some(0.7),
        "一般" | "普通" | "凑合" | "马马虎虎" => Some(0.5),
        "不太熟" | "不熟" | "不太会" | "有点难" | "不太行" | "不熟练" => Some(0.35),
        "不会" | "太难" | "记不住" | "记不牢" | "完全不会" | "很差" | "忘了" | "忘记了" | "全忘了" => Some(0.15),
        _ => None,
    }
}

/// 心情关键词 → 评分映射
fn mood_to_score(word: &str) -> Option<i32> {
    match word {
        "很好" | "非常好" | "充实" | "高效" | "超棒" | "开心" => Some(5),
        "不错" | "还行" | "可以" | "正常" | "挺好" | "良好" => Some(4),
        "一般" | "普通" | "凑合" | "平平" => Some(3),
        "不太好" | "有点累" | "效率低" | "不太好" | "比较差" => Some(2),
        "很差" | "糟糕" | "不想学" | "没状态" | "很差劲" => Some(1),
        _ => None,
    }
}

/// 提取时长（分钟）
fn extract_duration(input: &str) -> Option<i32> {
    // "1小时" "1.5小时" "2个半小时" "30分钟" "1小时30分钟"
    let mut total = 0;
    let hour_re = HOUR_RE.get_or_init(|| regex_lite::Regex::new(r"(\d+(?:\.\d+)?)\s*小?时").unwrap());
    let min_re = MIN_RE.get_or_init(|| regex_lite::Regex::new(r"(\d+)\s*分[钟钟]?").unwrap());
    let pomo_re = POMO_RE.get_or_init(|| regex_lite::Regex::new(r"(\d+)\s*(个)?番茄钟?").unwrap());

    if let Some(caps) = hour_re.captures(input) {
        if let Some(h) = caps.get(1) {
            total += (h.as_str().parse::<f64>().unwrap_or(0.0) * 60.0) as i32;
        }
    }
    if let Some(caps) = min_re.captures(input) {
        if let Some(m) = caps.get(1) {
            total += m.as_str().parse::<i32>().unwrap_or(0);
        }
    }
    if let Some(caps) = pomo_re.captures(input) {
        if let Some(p) = caps.get(1) {
            total += p.as_str().parse::<i32>().unwrap_or(0) * 25; // 1 个番茄钟 = 25min
        }
    }

    if total > 0 { Some(total) } else { None }
}

/// 从文本中尝试匹配科目名
fn match_subject(input: &str, subjects: &[(String, String)]) -> Option<(String, String)> {
    for (id, name) in subjects {
        if input.contains(name.as_str()) {
            return Some((id.clone(), name.clone()));
        }
    }
    None
}

/// 快速提取标题的核心名词短语
fn extract_title(input: &str, action_word_end: usize) -> String {
    let after = &input[action_word_end..];
    // 去掉时间表达式
    let re = Some(TIME_RE.get_or_init(|| regex_lite::Regex::new(r"(\d+(?:\.\d+)?)\s*(小?时|分[钟钟]?|个番茄钟?)").unwrap()));
    let cleaned = if let Some(re) = &re {
        re.replace_all(after, "")
    } else {
        std::borrow::Cow::Borrowed(after)
    };
    let title = cleaned.trim().trim_start_matches(['的', '个', '下', '一']);
    if title.is_empty() { "学习任务".to_string() } else { title.to_string() }
}

/// 核心解析函数
pub async fn parse_intent(
    pool: &SqlitePool,
    input: &str,
) -> Result<(ParsedIntent, ChatResponse)> {
    let input_trimmed = input.trim();
    if input_trimmed.is_empty() {
        return Err(AppError::Internal("输入为空".to_string()));
    }

    // 加载科目列表用于匹配
    let subjects: Vec<(String, String)> = sqlx::query(
        "SELECT id, name FROM subjects WHERE is_active = 1"
    ).fetch_all(pool).await?.iter().map(|r| {
        (r.get("id"), r.get("name"))
    }).collect();

    let result = parse_inner(input_trimmed, &subjects);
    let response = build_response(&result, pool).await?;
    Ok((result, response))
}

fn parse_inner(input: &str, subjects: &[(String, String)]) -> ParsedIntent {
    // ============================================
    // 第 1 层：精确命令匹配
    // ============================================

    // --- start_study ---
    let action_re = Some(ACTION_RE.get_or_init(|| regex_lite::Regex::new(r"(开始|复习|学习|做|背|写|看|继续|练)").unwrap()));
    let action_match = action_re.as_ref().and_then(|r| r.find(input));
    // 只有句子以动作词开头、或前面没有否定词时才识别为 StartStudy
    let is_start = action_match.map_or(false, |m| {
        let prefix = &input[..m.start()];
        !prefix.contains("不") && !prefix.contains("没") && !prefix.contains("结束")
            && !prefix.contains("完成") && !prefix.contains("完了")
    });

    if is_start {
        let m = action_match.unwrap();
        let title = extract_title(input, m.end());
        let duration_min = extract_duration(input);
        let subj = match_subject(input, subjects);

        return ParsedIntent::StartStudy {
            subject_id: subj.as_ref().map(|s| s.0.clone()),
            subject_name: subj.map(|s| s.1),
            title,
            duration_min,
        };
    }

    // --- stop_study ---
    if input.contains("结束") || input.contains("完成") || input.contains("学完了")
        || input.contains("搞定了") || input.contains("做完了") || input.contains("不学了")
        || input.contains("停了")
    {
        // 尝试提取反馈
        let feedback = if input.len() > 5 { Some(input.to_string()) } else { None };
        let mastery_label = {
            for kw in ["完全不熟", "勉强记得", "基本掌握", "完全掌握"] {
                if input.contains(kw) { return ParsedIntent::StopStudy { feedback, mastery_label: Some(kw.to_string()) }; }
            }
            None
        };
        return ParsedIntent::StopStudy { feedback, mastery_label };
    }

    // --- query_status ---
    if (input.contains("今天") || input.contains("今日") || input.contains("今天学了"))
        && (input.contains("什么") || input.contains("怎么样") || input.contains("进度")
            || input.contains("汇总") || input.contains("总结"))
    {
        return ParsedIntent::QueryStatus;
    }
    if input == "今天怎么样" || input == "今天学了什么" || input == "今日总结" {
        return ParsedIntent::QueryStatus;
    }

    // --- query_suggestion ---
    if input.contains("建议") || input.contains("推荐") || input.contains("接下来")
        || input.contains("该学什么") || input.contains("学什么好")
        || input == "学什么" || input.contains("给我推荐")
    {
        return ParsedIntent::QuerySuggestion;
    }

    // ============================================
    // 第 2 层：关键词 + 实体提取
    // ============================================

    // --- quick_plan ---（未来的计划，不立即开始）
    let time_words = ["明天", "后天", "周末", "下周", "星期", "周一", "周二", "周三", "周四", "周五", "周六", "周日"];
    let has_future_time = time_words.iter().any(|&t| input.contains(t));
    let has_plan_word = input.contains("要") || input.contains("计划") || input.contains("打算")
        || input.contains("安排") || input.contains("准备");

    if has_future_time && has_plan_word {
        let mut title = input.to_string();
        for &tw in &time_words { title = title.replace(tw, ""); }
        for &pw in &["要", "计划", "打算", "安排", "准备"] { title = title.replace(pw, ""); }
        let subj = match_subject(input, subjects);
        let date_hint = time_words.iter().find(|&&t| input.contains(t)).map(|s| s.to_string());
        return ParsedIntent::QuickPlan {
            subject_id: subj.as_ref().map(|s| s.0.clone()),
            subject_name: subj.map(|s| s.1),
            title: title.trim().to_string(),
            date_hint,
        };
    }

    // --- quick_review ---（独立复盘，不含 start/stop 动作）
    let has_mood = ["效率", "状态", "心情", "感觉"].iter().any(|&w| input.contains(w));
    if has_mood && !is_start {
        let notes = input.to_string();
        let mut mood = None;
        for (kw, score) in [
            ("高效", 5), ("充实", 5), ("不错", 4), ("还行", 3),
            ("一般", 3), ("不太好", 2), ("比较差", 2), ("很差", 1),
        ] {
            if input.contains(kw) { mood = Some(score); break; }
        }
        return ParsedIntent::QuickReview { mood, notes };
    }

    // --- log_feedback ---（关于知识点的情绪反馈）
    let subj_match = match_subject(input, subjects);
    let mut sentiment = 0.5; // 默认中性
    let mut found_sentiment = false;
    for (kw, score) in [
        ("掌握了", 0.9), ("很好", 0.9), ("熟练", 0.9),
        ("不错", 0.7), ("还行", 0.7), ("基本掌握", 0.7), ("还可以", 0.7),
        ("一般", 0.5), ("凑合", 0.5),
        ("不太熟", 0.35), ("不熟", 0.35), ("有点难", 0.35), ("记不住", 0.15),
        ("太难", 0.15), ("不会", 0.15), ("忘了", 0.15), ("全忘了", 0.15),
    ] {
        if input.contains(kw) { sentiment = score; found_sentiment = true; break; }
    }
    if found_sentiment || (subj_match.is_some() && input.len() < 30) {
        return ParsedIntent::LogFeedback {
            subject_hint: subj_match.map(|s| s.1),
            sentiment,
            notes: input.to_string(),
        };
    }

    // ============================================
    // 第 3 层：未识别（前端可尝试 LLM 兜底）
    // ============================================
    ParsedIntent::Unknown { raw: input.to_string() }
}

async fn build_response(intent: &ParsedIntent, pool: &SqlitePool) -> Result<ChatResponse> {
    match intent {
        ParsedIntent::StartStudy { subject_name, title, duration_min, .. } => {
            let subj_str = subject_name.as_deref().unwrap_or("");
            let dur_str = duration_min.map_or(String::new(), |d| format!("{}分钟", d));
            let reply = if !subj_str.is_empty() && !dur_str.is_empty() {
                format!("✅ 已创建计划「{}」({}，{})\n⏱️ 番茄钟已开始!", title, subj_str, dur_str)
            } else if !subj_str.is_empty() {
                format!("✅ 已创建计划「{}」({})\n⏱️ 番茄钟已开始!", title, subj_str)
            } else {
                format!("✅ 已创建计划「{}」\n⏱️ 番茄钟已开始!", title)
            };

            Ok(ChatResponse {
                reply,
                intent_type: "start_study".into(),
                actions: vec![
                    ActionItem {
                        action_type: "create_plan".into(),
                        description: format!("创建计划「{}」", title),
                        data: serde_json::json!({
                            "title": title,
                            "subject_name": subj_str,
                            "duration_min": duration_min,
                        }),
                    },
                    ActionItem {
                        action_type: "start_timer".into(),
                        description: "启动番茄钟".into(),
                        data: serde_json::json!({"duration": duration_min.unwrap_or(25)}),
                    },
                ],
            })
        }

        ParsedIntent::StopStudy { mastery_label, .. } => {
            let reply = if let Some(label) = mastery_label {
                format!("⏰ 番茄钟结束！\n已记录掌握度：{}", label)
            } else {
                "⏰ 番茄钟结束！\n这次学习掌握得怎么样？".into()
            };
            Ok(ChatResponse {
                reply,
                intent_type: "stop_study".into(),
                actions: vec![ActionItem {
                    action_type: "stop_timer".into(),
                    description: "结束番茄钟，弹出掌握度卡片".into(),
                    data: serde_json::json!({"mastery_label": mastery_label}),
                }],
            })
        }

        ParsedIntent::LogFeedback { sentiment, notes, .. } => {
            let desc = if *sentiment >= 0.7 { "掌握良好" }
            else if *sentiment >= 0.4 { "需要加强" }
            else { "需要重点复习" };
            Ok(ChatResponse {
                reply: format!("已记录反馈（{}）：{}", desc, notes),
                intent_type: "log_feedback".into(),
                actions: vec![ActionItem {
                    action_type: "update_mastery".into(),
                    description: format!("更新知识点掌握度 ({})", desc),
                    data: serde_json::json!({"sentiment": sentiment, "notes": notes}),
                }],
            })
        }

        ParsedIntent::QueryStatus => {
            let now = chrono::Utc::now().timestamp();
            let today_start = now - (now % 86400);
            let today_minutes: i64 = sqlx::query_scalar(
                "SELECT COALESCE(CAST(SUM(time_spent_seconds) AS INTEGER), 0) FROM review_sessions WHERE actual_date >= ?"
            ).bind(today_start).fetch_one(pool).await?;
            let today_completed: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM executions WHERE end_time >= ? AND completion_rate IS NOT NULL"
            ).bind(today_start).fetch_one(pool).await?;
            let due_today: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM review_sessions WHERE scheduled_date <= ? AND actual_date IS NULL AND was_skipped = 0"
            ).bind(now).fetch_one(pool).await?;

            let reply = format!(
                "📊 今日学习情况：\n- 已完成 {} 个番茄钟\n- 总复习时长 {} 分钟\n- {} 个知识点待复习",
                today_completed, today_minutes / 60, due_today
            );
            Ok(ChatResponse {
                reply,
                intent_type: "query_status".into(),
                actions: vec![],
            })
        }

        ParsedIntent::QuerySuggestion => {
            let due_today: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM review_sessions WHERE scheduled_date <= ? AND actual_date IS NULL AND was_skipped = 0"
            ).bind(chrono::Utc::now().timestamp()).fetch_one(pool).await?;

            let reply = if due_today > 0 {
                format!("💡 今日有 {} 个知识点待复习，建议优先处理！\n打开「艾宾浩斯复习」页面查看详情。", due_today)
            } else {
                "💡 今日暂无待复习项。建议查看「智能分析」页面，看看哪些科目需要加强。".into()
            };
            Ok(ChatResponse { reply, intent_type: "query_suggestion".into(), actions: vec![] })
        }

        ParsedIntent::QuickPlan { title, subject_name, date_hint, .. } => {
            let date_str = date_hint.as_deref().unwrap_or("待定");
            Ok(ChatResponse {
                reply: format!("📋 已创建计划「{}」({})，安排于 {}。", title, subject_name.as_deref().unwrap_or("未归类"), date_str),
                intent_type: "quick_plan".into(),
                actions: vec![ActionItem {
                    action_type: "create_plan".into(),
                    description: format!("创建计划「{}」", title),
                    data: serde_json::json!({"title": title, "subject_name": subject_name, "date_hint": date_hint}),
                }],
            })
        }

        ParsedIntent::QuickReview { mood, notes } => {
            let mood_str = mood.map(|m| format!("心情 {} 分", m)).unwrap_or_default();
            Ok(ChatResponse {
                reply: format!("📝 已记录复盘：{} {}", mood_str, notes),
                intent_type: "quick_review".into(),
                actions: vec![ActionItem {
                    action_type: "create_review".into(),
                    description: "创建复盘记录".into(),
                    data: serde_json::json!({"mood": mood, "notes": notes}),
                }],
            })
        }

        ParsedIntent::Unknown { raw } => {
            Ok(ChatResponse {
                reply: format!("抱歉，我暂时无法理解「{}」。\n试试这样说：\n- 「复习数学1小时」\n- 「今天学了什么」\n- 「给我建议」", raw),
                intent_type: "unknown".into(),
                actions: vec![],
            })
        }
    }
}

/// 从计划标题拆分知识点候选列表
///
/// 规则：
/// 1. 按「和」「与」「、」「及」「,」分割
/// 2. 去掉「复习」「学习」「完成」「练习」「背诵」等动作词
/// 3. 去掉「第X章」「Unit X」「第X课」等编号前缀
/// 4. 去掉空格，过滤空白结果
/// 5. 去重
pub fn split_kp_from_title(title: &str) -> Vec<String> {
    // 去掉动作词
    let mut cleaned = title.to_string();
    for prefix in &["复习", "学习", "完成", "练习", "背诵", "预习", "做", "背", "写", "看", "读", "继续"] {
        if cleaned.starts_with(prefix) {
            cleaned = cleaned[prefix.len()..].to_string();
            break;
        }
    }

    // 多字分隔符先替换为单字
    let cleaned = cleaned.replace("以及", "、").replace("还有", "、");
    // 分割符（单字符）
    let parts: Vec<&str> = cleaned
        .split(&['和', '与', '、', ',', '，', '及'][..])
        .collect();

    let mut result: Vec<String> = Vec::new();
    for part in parts {
        let mut s = part.trim().to_string();

        // 去掉编号前缀
        let re_num = Some(CHAPTER_RE.get_or_init(|| regex_lite::Regex::new(r"^(第?\d+[章节课].?|Unit\s*\d+\.?|Ch\.?\s*\d+)").unwrap()));
        if let Some(re) = &re_num {
            s = re.replace(&s, "").to_string();
        }

        s = s.trim().to_string();

        // 过滤太短或纯数字的结果
        if s.len() >= 2 && !s.chars().all(|c| c.is_ascii_digit() || c == '.') {
            // 去重
            if !result.iter().any(|r| r == &s || s.contains(r.as_str()) || r.contains(s.as_str())) {
                result.push(s);
            }
        }
    }

    result
}

/// 为计划标题生成知识点建议（含去重检查）
pub async fn suggest_kps(
    pool: &SqlitePool,
    title: &str,
    _subject_id: &str,
) -> Result<Vec<String>> {
    let candidates = split_kp_from_title(title);

    // 查重：过滤掉已存在的同名知识点
    let mut suggestions = Vec::new();
    for c in candidates {
        let exists: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM knowledge_points WHERE name = ?"
        ).bind(&c).fetch_one(pool).await?;

        if exists == 0 {
            suggestions.push(c);
        }
    }

    Ok(suggestions)
}
