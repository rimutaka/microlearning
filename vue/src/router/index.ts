import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import QuestionFormView from '@/views/QuestionFormView.vue'
import QuestionView from '@/views/QuestionView.vue'
import AboutView from '../views/AboutView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/add',
      name: 'add',
      component: QuestionFormView
    },
    {
      path: '/question',
      name: 'question',
      component: QuestionView,
    },
    {
      path: '/about',
      name: 'about',
      component: AboutView
    }
  ]
})

export default router
