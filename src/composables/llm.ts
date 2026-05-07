import { getSettings } from './api';
import type { WeakSubjectAlert, SubjectDistribution, WeeklyTrend, ReviewStats } from '../types';

/**
 * 调用 LLM API 生成深度学习建议
 * 仅发送聚合统计数据，不包含隐私笔记内容
 */
export async function requestLlmAdvice(
  weakSubjects: WeakSubjectAlert[],
  timeDist: SubjectDistribution[],
  trend: WeeklyTrend[],
  reviewStats: ReviewStats,
): Promise<string> {
  const settings = await getSettings();

  if (!settings.llm_enabled || !settings.llm_api_key) {
    throw new Error('AI 分析未启用或 API Key 未配置，请在设置中配置。');
  }

  const prompt = buildPrompt(weakSubjects, timeDist, trend, reviewStats);
  const endpoint = settings.llm_endpoint || 'https://api.anthropic.com/v1/messages';
  const model = settings.llm_model || 'claude-sonnet-4-6';

  const isAnthropic = endpoint.includes('anthropic.com');

  let response: Response;
  if (isAnthropic) {
    response = await fetch(endpoint, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-api-key': settings.llm_api_key,
        'anthropic-version': '2023-06-01',
      },
      body: JSON.stringify({
        model,
        max_tokens: 1024,
        messages: [{ role: 'user', content: prompt }],
      }),
    });
  } else {
    // OpenAI 兼容格式
    response = await fetch(endpoint, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${settings.llm_api_key}`,
      },
      body: JSON.stringify({
        model,
        max_tokens: 1024,
        messages: [{ role: 'user', content: prompt }],
      }),
    });
  }

  if (!response.ok) {
    const errText = await response.text();
    throw new Error(`AI API 调用失败 (${response.status}): ${errText}`);
  }

  const data = await response.json();

  if (isAnthropic) {
    return data?.content?.[0]?.text || '未获取到 AI 建议';
  } else {
    return data?.choices?.[0]?.message?.content || '未获取到 AI 建议';
  }
}

function buildPrompt(
  weakSubjects: WeakSubjectAlert[],
  timeDist: SubjectDistribution[],
  trend: WeeklyTrend[],
  reviewStats: ReviewStats,
): string {
  const weakInfo = weakSubjects.map(w =>
    `- ${w.subject_name}：薄弱评分 ${Math.round(w.score * 100)}%，因子：${w.factors.map(f => `${f.name}(${Math.round(f.value * 100)}%)`).join('、')}`
  ).join('\n') || '暂无薄弱科目';

  const distInfo = timeDist.map(d =>
    `- ${d.subject_name}：${d.total_hours.toFixed(1)}h（${d.execution_count}次）`
  ).join('\n') || '暂无分布数据';

  const totalHours = trend.reduce((s, t) => s + t.hours, 0).toFixed(1);
  const trendInfo = `近 ${trend.length} 天总计 ${totalHours}h`;

  return `你是一位经验丰富的高中学习规划导师。以下是一位高中生的学习数据（仅统计摘要，无隐私内容）：

【薄弱科目分析】
${weakInfo}

【近30天学习时间分布】
${distInfo}

【学习趋势】
${trendInfo}

【艾宾浩斯复习】
- 已掌握：${reviewStats.mastered_kps}/${reviewStats.total_kps}
- 今日待复习：${reviewStats.due_today}
- 本周已完成复习：${reviewStats.completed_this_week}
- 平均掌握评分：${Math.round(reviewStats.avg_mastery_score * 100)}%

请基于以上数据给出3-5条具体、可操作的学习建议，用中文回答。每条建议不超过80字，使用要点列表格式。`;
}
