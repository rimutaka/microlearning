import { createRouter, createWebHistory } from 'vue-router'
import { authGuard } from '@auth0/auth0-vue';

import HomeView from '../views/HomeView.vue'
import QuestionFormView from '@/views/QuestionFormView.vue'
import QuestionView from '@/views/QuestionView.vue'
import AboutView from '../views/AboutView.vue'
import SubscriptionView from '../views/SubscriptionView.vue'

/// A list of page names used elsewhere in the app
export const PageNames = {
  HOME: 'home',
  ADD: 'add',
  QUESTION: 'question',
  ABOUT: 'about',
  SUBSCRIPTION: 'subscription',
}

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: PageNames.HOME,
      component: HomeView,
    },
    {
      path: '/' + PageNames.ADD,
      name: PageNames.ADD,
      component: QuestionFormView,
      beforeEnter: authGuard,
      meta: { title: 'Contribute' }
    },
    {
      path: '/' + PageNames.QUESTION,
      name: PageNames.QUESTION,
      component: QuestionView,
      meta: { title: 'Question' }
    },
    {
      path: '/' + PageNames.ABOUT,
      name: PageNames.ABOUT,
      component: AboutView,
      meta: { title: 'About' }
    },
    {
      path: '/' + PageNames.SUBSCRIPTION,
      name: PageNames.SUBSCRIPTION,
      component: SubscriptionView,
      beforeEnter: authGuard,
      meta: { title: 'Subscription' }
    }
  ]
})

router.afterEach((to, from) => {
  const title = <string>to.meta?.title;
  const websiteTitle = 'Bite-sized learning';
  window.document.title = title ? `${title} | ${websiteTitle}` : websiteTitle;
});

export default router
