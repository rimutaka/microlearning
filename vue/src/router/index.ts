import { createRouter, createWebHistory } from 'vue-router'
import { authGuard } from '@auth0/auth0-vue';

import HomeView from '../views/HomeView.vue'
import QuestionFormView from '@/views/QuestionFormView.vue'
import QuestionView from '@/views/QuestionView.vue'
import AboutView from '../views/AboutView.vue'
import SubscriptionView from '../views/SubscriptionView.vue'
import ContributeView from '@/views/ContributeView.vue';

/// A list of page names used elsewhere in the app
export const PageIDs = {
  HOME: 'home',
  CONTRIBUTE: 'contribute',
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
      name: PageIDs.HOME,
      component: HomeView,
    },
    {
      path: '/' + PageIDs.CONTRIBUTE,
      name: PageIDs.CONTRIBUTE,
      component: ContributeView,
      meta: { title: 'Contribute' }
    },
    {
      path: '/' + PageIDs.ADD,
      name: PageIDs.ADD,
      component: QuestionFormView,
      beforeEnter: authGuard,
      meta: { title: 'Edit question' }
    },
    {
      path: '/' + PageIDs.QUESTION,
      name: PageIDs.QUESTION,
      component: QuestionView,
      meta: { title: 'Question' }
    },
    {
      path: '/' + PageIDs.ABOUT,
      name: PageIDs.ABOUT,
      component: AboutView,
      meta: { title: 'About' }
    },
    {
      path: '/' + PageIDs.SUBSCRIPTION,
      name: PageIDs.SUBSCRIPTION,
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
