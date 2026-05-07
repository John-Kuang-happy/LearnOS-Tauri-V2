import { createRouter, createWebHistory } from 'vue-router';
import Home from '../pages/Home.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
      meta: { title: '首页', icon: 'home' },
    },
    {
      path: '/subjects',
      name: 'subjects',
      component: () => import('../pages/Subjects.vue'),
      meta: { title: '科目管理', icon: 'book' },
    },
    {
      path: '/plans',
      name: 'plans',
      component: () => import('../pages/Plans.vue'),
      meta: { title: '学习计划', icon: 'clipboard' },
    },
    {
      path: '/plans/:id',
      name: 'planDetail',
      component: () => import('../pages/PlanDetail.vue'),
      meta: { title: '计划详情' },
    },
    {
      path: '/executions',
      name: 'executions',
      component: () => import('../pages/Executions.vue'),
      meta: { title: '执行记录', icon: 'timer' },
    },
    {
      path: '/progress',
      name: 'progress',
      component: () => import('../pages/Progress.vue'),
      meta: { title: '学习进度', icon: 'chart' },
    },
    { path: '/knowledge-points', redirect: '/progress' },
    { path: '/ebbinghaus', redirect: '/review-board' },
    {
      path: '/review-board',
      name: 'reviewBoard',
      component: () => import('../pages/ReviewBoard.vue'),
      meta: { title: '复习看板', icon: 'refresh' },
    },
    {
      path: '/reviews',
      name: 'reviews',
      component: () => import('../pages/Reviews.vue'),
      meta: { title: '复盘分析', icon: 'refresh' },
    },
    {
      path: '/analysis',
      name: 'analysis',
      component: () => import('../pages/Analysis.vue'),
      meta: { title: '智能分析', icon: 'chart' },
    },
    {
      path: '/exams',
      name: 'exams',
      component: () => import('../pages/Exams.vue'),
      meta: { title: '考试倒计时', icon: 'calendar' },
    },
    {
      path: '/help',
      name: 'help',
      component: () => import('../pages/Help.vue'),
      meta: { title: '帮助', icon: 'question' },
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../pages/Settings.vue'),
      meta: { title: '设置', icon: 'gear' },
    },
  ],
});

export default router;
