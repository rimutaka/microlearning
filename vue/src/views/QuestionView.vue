<template>
  <h1 v-if="question?.title" class="mb-4 md:mb-8 text-2xl text-start">{{ question?.title }}</h1>
  <h1 v-else class="mb-4 md:mb-8 text-2xl text-start">Question about <em class="italic">{{ topicName }}</em></h1>
  <QuestionDraftCTA v-if="question?.stage == PublishStage.Draft" />
  <LoadingMessage v-if="questionStatus == LoadingStatus.Loading" />
  <div v-else>
    <QuestionCard :next="true" @next-question="loadNextQuestion" />
    <div v-if="ctaBlockVisible" class="mb-12 md:mt-12 cta-box">
      <PostAnswerCTA />
    </div>
    <ContributorCard class="mb-12 mt-8 md:mt-16" />
  </div>
</template>

<script setup lang="ts">
import { computed, watch, ref, watchEffect } from 'vue';
import { useRoute, useRouter } from 'vue-router'
import { TOPICS, randomTopicId, findTopicById } from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { LoadingStatus, PublishStage } from '@/interfaces';

import QuestionCard from "../components/QuestionCard.vue";
import PostAnswerCTA from '@/components/PostAnswerCTA.vue';
import LoadingMessage from '@/components/LoadingMessage.vue';
import ContributorCard from "../components/ContributorCard.vue";
import QuestionDraftCTA from '@/components/QuestionDraftCTA.vue';

const route = useRoute();
const router = useRouter();

// the very first thing that has to be done - navigate to a random topic if none is provided
const randomTopic = randomTopicId(); // pick a random topic now in case it is needed
if (!route.query.topic) {
  // if no topic is provided, pick a random one
  console.log("Navigating to a random topic: ", randomTopic);
  router.replace({ query: { topic: randomTopic } });
}

const store = useMainStore();
const { question, email, questionStatus } = storeToRefs(store);

// save the initial values to maintain state later because the topic and qid will change
// the route change may lag behind, so it's better to reuse the random topic value from the variable
const initialTopic = route.query.topic ? <string>route.query.topic : randomTopic;
const initialQid = route.query.qid ? <string>route.query.qid : undefined;

const topicName = computed(() => {
  return findTopicById(route.query.topic as string | undefined);
});

// this flag tells to not store pages that have no question ID in history because they redirect to a new 
// random question catching the user in a loop
let replaceRouter = initialQid ? false : true;

// route.query.topic and .qid can potentially be an array, but it should not happen in this app,
// so it is safe to cast them into a string
const topic = ref<string | undefined>(initialTopic);
const qid = ref<string | undefined>(initialQid);

const ctaBlockVisible = computed(() => {
  // checking user subs does not work because this page does not load user details
  // if  (!user.value?.topics.length && question.value?.answers?.[0].e) { return true } else { return false };
  if (!email.value && question.value?.answers?.[0].e) { return true } else { return false };
});

// reset the question ID and reload to show a new question on the same topic
const loadNextQuestion = () => {
  console.log("Load next question");
  qid.value = undefined;
  replaceRouter = true;
  store.loadNextQuestion(initialTopic);
};

// update the query string with the next question topic and id when the question changes in the store
watch(question, (newQuestion) => {
  // console.log("newQuestion: ", newQuestion);
  if (newQuestion) {
    topic.value = newQuestion.topic;
    qid.value = newQuestion.qid;
    console.log("New question topic/qid: ", topic.value, qid.value);

    // update page title
    if (question.value?.title) {
      window.document.title = question.value?.title;
    }

    // update the URL query string to match the current question
    if (route.query.topic != topic.value || route.query.qid != qid.value) {
      if (replaceRouter) {
        console.log("replace-navigating to ", topic.value, qid.value);
        replaceRouter = false; // only replace the first time, subsequent calls should have qid
        router.replace({ query: { topic: topic.value, qid: qid.value } });
      } else {
        console.log("push-navigating to ", topic.value, qid.value);
        router.push({ query: { topic: topic.value, qid: qid.value } });
      }
    }
  }
  else {
    console.log("question removed from store ");
  }
});

// this watch is needed to load questions on back/forward navigation
watch(route, (newQuery) => {
  const newQid = newQuery.query.qid as string | undefined;
  const newTopic = newQuery.query.topic as string | undefined;
  console.log(`route.query changed to ${newTopic} / ${newQid}`);
  if (newTopic && (newTopic != topic.value || newQid != qid.value)) {
    // load the new question if it is different from the current one
    store.loadQuestion(newTopic, newQid);
  }
});

// load a question when the component is mounted
if (initialTopic && initialQid) {
  // a particular question was requested
  (async () => await store.loadQuestion(initialTopic, initialQid))();
} else {
  // a random question for a particular topic was requested
  // or no topic at all and it was picked at random
  loadNextQuestion();
}

</script>
