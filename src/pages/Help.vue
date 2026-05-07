<template>
  <div class="space-y-6">
    <div>
      <h2 class="text-2xl font-bold text-gray-900 dark:text-white">📖 帮助中心</h2>
      <p class="text-gray-500 dark:text-gray-400 mt-1">了解 LearnOS 的设计理念和各个功能的使用方法</p>
    </div>

    <!-- 设计理念 -->
    <div class="card">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">💡 设计理念</h3>
      <div class="prose prose-sm dark:prose-invert max-w-none text-gray-700 dark:text-gray-300 space-y-4">
        <p>
          <strong>LearnOS</strong> 是一款面向高中生的<strong>智能学习助手</strong>，核心理念是：
        </p>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 not-prose">
          <div class="p-4 rounded-xl bg-primary-50 dark:bg-primary-900/20 border border-primary-200 dark:border-primary-800">
            <p class="text-2xl mb-2">🧠</p>
            <p class="font-semibold text-primary-700 dark:text-primary-400 text-sm">科学记忆</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">基于艾宾浩斯遗忘曲线的间隔复习算法，在最佳时间点提醒你复习知识点，大幅提升长期记忆保留率。</p>
          </div>
          <div class="p-4 rounded-xl bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800">
            <p class="text-2xl mb-2">🎯</p>
            <p class="font-semibold text-amber-700 dark:text-amber-400 text-sm">目标驱动</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">以考试日期为倒计时目标，通过周目标设定、优先级排序和薄弱科目分析，确保学习方向始终正确。</p>
          </div>
          <div class="p-4 rounded-xl bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800">
            <p class="text-2xl mb-2">📊</p>
            <p class="font-semibold text-green-700 dark:text-green-400 text-sm">数据驱动</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">自动收集学习数据，生成多维度分析报告和个性化建议，帮你发现薄弱科目、优化时间分配。</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 核心概念 -->
    <div class="card">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">🏗️ 核心概念</h3>
      <div class="space-y-4">
        <div class="flex gap-3">
          <div class="w-10 h-10 rounded-lg bg-blue-100 dark:bg-blue-900/30 flex items-center justify-center flex-shrink-0 text-lg">📚</div>
          <div>
            <p class="font-medium text-gray-900 dark:text-white text-sm">科目</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">学习的顶层分类，如语文、数学、英语等。每个科目可以设置每周学习时长目标和 KP（知识点）掌握目标。</p>
          </div>
        </div>
        <div class="flex gap-3">
          <div class="w-10 h-10 rounded-lg bg-purple-100 dark:bg-purple-900/30 flex items-center justify-center flex-shrink-0 text-lg">🔖</div>
          <div>
            <p class="font-medium text-gray-900 dark:text-white text-sm">知识点（KP）</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">每个科目下的具体学习单元，如"二次函数"、"定语从句"。系统用 <strong>掌握度（0~100%）</strong> 追踪你的熟练程度。</p>
          </div>
        </div>
        <div class="flex gap-3">
          <div class="w-10 h-10 rounded-lg bg-amber-100 dark:bg-amber-900/30 flex items-center justify-center flex-shrink-0 text-lg">📋</div>
          <div>
            <p class="font-medium text-gray-900 dark:text-white text-sm">学习计划</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">安排具体的学习任务，支持普通计划、复习计划、考试备考等类型。可关联知识点以追踪掌握进度。</p>
          </div>
        </div>
        <div class="flex gap-3">
          <div class="w-10 h-10 rounded-lg bg-red-100 dark:bg-red-900/30 flex items-center justify-center flex-shrink-0 text-lg">🍅</div>
          <div>
            <p class="font-medium text-gray-900 dark:text-white text-sm">番茄钟</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">每个计划通过番茄钟计时执行。完成后对掌握度做自我评估，系统自动调整下次复习时间。</p>
          </div>
        </div>
        <div class="flex gap-3">
          <div class="w-10 h-10 rounded-lg bg-green-100 dark:bg-green-900/30 flex items-center justify-center flex-shrink-0 text-lg">🔄</div>
          <div>
            <p class="font-medium text-gray-900 dark:text-white text-sm">艾宾浩斯复习</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">系统基于 SM-2 算法自动计算每个知识点的最佳复习间隔（1天 → 3天 → 7天 → 15天 → 30天 → ...），到期时自动提醒你复习。</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 页面指南 -->
    <div class="card">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">📑 页面指南</h3>
      <div class="space-y-5">

        <!-- 首页 -->
        <div>
          <div class="flex items-center gap-2 mb-2">
            <span class="text-lg">🏠</span>
            <h4 class="font-semibold text-gray-900 dark:text-white">首页</h4>
            <span class="text-xs px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500">每日入口</span>
          </div>
          <div class="ml-8 space-y-1">
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>今日推荐：</strong>显示今天到期的复习任务和待办计划，按优先级排序。点击任意项直接开始番茄钟学习。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>今日摘要：</strong>右侧卡片展示已学时长、已完成数、待复习数、待办计划数。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>本周目标：</strong>顶部进度条展示各科目本周 KP 和时间目标的达成进度。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>考试倒计时：</strong>右上角展示最近一场考试的天数。</p>
          </div>
        </div>

        <!-- 学习计划 -->
        <div>
          <div class="flex items-center gap-2 mb-2">
            <span class="text-lg">📋</span>
            <h4 class="font-semibold text-gray-900 dark:text-white">学习计划</h4>
            <span class="text-xs px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500">任务管理</span>
          </div>
          <div class="ml-8 space-y-1">
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>创建计划：</strong>设置标题、科目、优先级（紧急/高/中/低）、估算时长。可开启"自动生成复习计划"。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>计划类型：</strong>普通计划、复习计划、作业、考试备考，不同类型在首页推荐中有不同优先级加权。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>关联知识点：</strong>计划可以绑定一个知识点，完成后系统会引导你对知识点做掌握度评估。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>完成计划：</strong>通过番茄钟执行计划，完成后计划状态自动更新。</p>
          </div>
        </div>

        <!-- 复习看板 -->
        <div>
          <div class="flex items-center gap-2 mb-2">
            <span class="text-lg">🔄</span>
            <h4 class="font-semibold text-gray-900 dark:text-white">复习看板</h4>
            <span class="text-xs px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500">艾宾浩斯复习</span>
          </div>
          <div class="ml-8 space-y-1">
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>统计卡片：</strong>今日待复习、本周待复习、连续打卡天数、本周已完成——一目了然。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>月历视图：</strong>带小红点标记的日历，显示每天有多少复习任务。点击日期查看当天的复习清单和完成情况。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>复习记录：</strong>时间线展示每次复习的时间、评分、间隔变化。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>提交反馈：</strong>复习后自评掌握度（0~100%），系统根据你的评分自动调整下次复习间隔。</p>
          </div>
        </div>

        <!-- 科目管理 -->
        <div>
          <div class="flex items-center gap-2 mb-2">
            <span class="text-lg">📚</span>
            <h4 class="font-semibold text-gray-900 dark:text-white">科目管理</h4>
            <span class="text-xs px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500">基础设置</span>
          </div>
          <div class="ml-8 space-y-1">
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>创建科目：</strong>设置科目名称、自定义颜色和图标、选择文理科分类。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>周目标：</strong>为每个科目设置每周学习时长目标（小时）和知识点掌握目标（个数）。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>知识点管理：</strong>在科目下添加具体的知识点，设置难度（1-5）和重要度（1-5），系统据此调整复习策略。</p>
          </div>
        </div>

        <!-- 学习进度 -->
        <div>
          <div class="flex items-center gap-2 mb-2">
            <span class="text-lg">📈</span>
            <h4 class="font-semibold text-gray-900 dark:text-white">学习进度</h4>
            <span class="text-xs px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500">全局总览</span>
          </div>
          <div class="ml-8 space-y-1">
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>科目掌握度：</strong>按科目展示已掌握知识点占比，以及本周 KP 目标的达成进度。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>掌握度概览：</strong>四个徽章——已掌握、学习中、待加强、待复习，快速了解全局状态。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>知识点列表：</strong>按科目分组，可折叠展开。每个知识点显示掌握度进度条、复习状态（已掌握/逾期/即将复习）。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>近期里程碑：</strong>自动生成的学习成就时间线，如"连续学习 7 天"、"累计学习 50 小时"、"全部掌握某科目"。</p>
          </div>
        </div>

        <!-- 智能分析 -->
        <div>
          <div class="flex items-center gap-2 mb-2">
            <span class="text-lg">📊</span>
            <h4 class="font-semibold text-gray-900 dark:text-white">智能分析</h4>
            <span class="text-xs px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500">数据洞察</span>
          </div>
          <div class="ml-8 space-y-1">
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>分析摘要：</strong>顶部四个数字——分析建议数、薄弱科目数、平均薄弱度、活跃科目数。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>薄弱科目分析：</strong>五因子加权评分（完成率、掌握度、学习情绪、目标达成、复习逾期），定位最需要加强的科目。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>时间分布饼图：</strong>近 30 天各科目学习时长占比，直观看出时间分配是否均衡。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>效率趋势折线图：</strong>近 8 周每日学习时长变化，观察学习节奏和效率波动。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>智能建议：</strong>基于规则引擎自动生成的学习建议（薄弱科目、时长下降、复习逾期、均衡建议）。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>AI 深度分析：</strong>（需配置 API Key）将聚合统计数据发送给 AI 模型，获取更深入的个性化建议。</p>
          </div>
        </div>

        <!-- 考试倒计时 -->
        <div>
          <div class="flex items-center gap-2 mb-2">
            <span class="text-lg">📅</span>
            <h4 class="font-semibold text-gray-900 dark:text-white">考试倒计时</h4>
            <span class="text-xs px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500">目标管理</span>
          </div>
          <div class="ml-8 space-y-1">
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>添加考试：</strong>设置考试名称（如"2026年高考"）、类型（高考/期末/期中/模拟）、日期。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>倒计时卡片：</strong>显示距考试还有多少天、进度条、颜色变化（≤7天红色，≤30天橙色）。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>优先级加成：</strong>临近考试的科目在首页推荐中自动获得更高的优先级。</p>
          </div>
        </div>

        <!-- 设置 -->
        <div>
          <div class="flex items-center gap-2 mb-2">
            <span class="text-lg">⚙️</span>
            <h4 class="font-semibold text-gray-900 dark:text-white">设置</h4>
            <span class="text-xs px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500">系统配置</span>
          </div>
          <div class="ml-8 space-y-1">
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>AI 智能分析：</strong>配置 LLM API Key 和模型，启用后可在智能分析页获取 AI 深度建议。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>开机自启动：</strong>登录时自动启动 LearnOS，确保每天不忘记打开。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>备份与还原：</strong>将数据库备份到本地文件，或从备份文件还原——换电脑或重装系统时不用担心数据丢失。</p>
            <p class="text-sm text-gray-700 dark:text-gray-300"><strong>清空数据：</strong>删除所有数据重新开始（需输入 DELETE 确认）。</p>
          </div>
        </div>

      </div>
    </div>

    <!-- 使用建议 -->
    <div class="card">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">🚀 快速上手</h3>
      <div class="space-y-3">
        <div class="flex items-start gap-3">
          <span class="w-6 h-6 rounded-full bg-primary-100 dark:bg-primary-900/30 text-primary-600 dark:text-primary-400 flex items-center justify-center text-xs font-bold flex-shrink-0 mt-0.5">1</span>
          <div>
            <p class="text-sm font-medium text-gray-900 dark:text-white">创建科目和知识点</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">在"科目管理"页添加你的高考科目，再为每个科目添加核心知识点。</p>
          </div>
        </div>
        <div class="flex items-start gap-3">
          <span class="w-6 h-6 rounded-full bg-primary-100 dark:bg-primary-900/30 text-primary-600 dark:text-primary-400 flex items-center justify-center text-xs font-bold flex-shrink-0 mt-0.5">2</span>
          <div>
            <p class="text-sm font-medium text-gray-900 dark:text-white">设置考试和目标</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">在"考试倒计时"页添加重要考试日期。在科目管理中设置每周学习时长和 KP 目标。</p>
          </div>
        </div>
        <div class="flex items-start gap-3">
          <span class="w-6 h-6 rounded-full bg-primary-100 dark:bg-primary-900/30 text-primary-600 dark:text-primary-400 flex items-center justify-center text-xs font-bold flex-shrink-0 mt-0.5">3</span>
          <div>
            <p class="text-sm font-medium text-gray-900 dark:text-white">创建和执行计划</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">在"学习计划"页创建每日学习计划，关联知识点。从首页点击计划开始番茄钟学习。</p>
          </div>
        </div>
        <div class="flex items-start gap-3">
          <span class="w-6 h-6 rounded-full bg-primary-100 dark:bg-primary-900/30 text-primary-600 dark:text-primary-400 flex items-center justify-center text-xs font-bold flex-shrink-0 mt-0.5">4</span>
          <div>
            <p class="text-sm font-medium text-gray-900 dark:text-white">坚持每日复习</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">每天打开"复习看板"，完成到期复习任务并提交掌握度评分。系统会自动安排下次复习时间。</p>
          </div>
        </div>
        <div class="flex items-start gap-3">
          <span class="w-6 h-6 rounded-full bg-primary-100 dark:bg-primary-900/30 text-primary-600 dark:text-primary-400 flex items-center justify-center text-xs font-bold flex-shrink-0 mt-0.5">5</span>
          <div>
            <p class="text-sm font-medium text-gray-900 dark:text-white">定期查看分析和进度</p>
            <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">每周查看"智能分析"了解薄弱科目，查看"学习进度"追踪掌握度变化，及时调整学习策略。</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 底部 -->
    <div class="text-center py-6">
      <p class="text-sm text-gray-400 dark:text-gray-500">LearnOS v0.2.0 · 为高中生打造的智能学习助手</p>
    </div>
  </div>
</template>

<script setup lang="ts">
// 纯展示页面，无需逻辑
</script>
