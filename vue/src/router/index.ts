import { createRouter, createWebHistory } from 'vue-router'
import { authGuard } from '@auth0/auth0-vue';
import { findTopicById, URL_PARAM_TOPIC } from '@/constants';

import HomeView from '../views/HomeView.vue'
import QuestionFormView from '@/views/QuestionFormView.vue'
import QuestionView from '@/views/QuestionView.vue'
import AboutView from '../views/AboutView.vue'
import SubscriptionView from '../views/SubscriptionView.vue'
import ContributeView from '@/views/ContributeView.vue';
import QuestionPreview from '@/views/QuestionPreview .vue';
import SponsorshipView from '@/views/SponsorshipView.vue';
import ThankYouView from '@/views/ThankYouView.vue';

/// A list of page names used elsewhere in the app
export const PageIDs = {
  HOME: 'home',
  CONTRIBUTE: 'contribute',
  ADD: 'add',
  QUESTION: 'question',
  ABOUT: 'about',
  SUBSCRIPTION: 'subscription',
  PREVIEW: 'preview',
  SPONSORSHIP: 'gift-a-question',
  THANKYOU: 'thankyou',
}

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  scrollBehavior() {
    return { top: 0 };  // always scroll to top
  },
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
    },
    {
      path: '/' + PageIDs.PREVIEW,
      name: PageIDs.PREVIEW,
      component: QuestionPreview,
      meta: { title: 'Preview' }
    },
    {
      path: '/' + PageIDs.SPONSORSHIP,
      name: PageIDs.SPONSORSHIP,
      component: SponsorshipView,
      meta: { title: 'Gift a question' }
    },
    {
      path: '/' + PageIDs.THANKYOU,
      name: PageIDs.THANKYOU,
      component: ThankYouView,
      meta: { title: 'Thank you!' }
    },

  ]
})

router.afterEach((to, from) => {
  const topic = findTopicById(to.query[URL_PARAM_TOPIC]?.toString());
  const metaTitle = <string>to.meta?.title;

  // question pages have the topic name added at the front of the title
  if (to.name === PageIDs.QUESTION && topic) {
    window.document.title = topic ? `${topic} ${metaTitle}` : metaTitle;
  }
  else {
    // other pages use the hardcoded meta property
    const defaultTitle = 'Bite-sized learning';
    window.document.title = metaTitle ? `${metaTitle} | ${defaultTitle}` : defaultTitle;
  }
});

export default router
