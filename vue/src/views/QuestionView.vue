<template>
  <h1 class="mb-4 md:mb-8 text-2xl text-start">Question about <em class="italic">{{ topicName }}</em></h1>
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
import { TOPICS, ANY_TOPIC, findTopicById } from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { LoadingStatus } from '@/interfaces';

import QuestionCard from "../components/QuestionCard.vue";
import PostAnswerCTA from '@/components/PostAnswerCTA.vue';
import LoadingMessage from '@/components/LoadingMessage.vue';
import ContributorCard from "../components/ContributorCard.vue";

const route = useRoute();
const router = useRouter();

const store = useMainStore();
const { question, email, questionStatus } = storeToRefs(store);

const topicName = computed(() => {
  return findTopicById(route.query.topic as string | undefined);
});

// save the initial values to maintain state later because the topic and qid will change
const initialTopic = route.query.topic ? <string>route.query.topic : ANY_TOPIC;
const initialQid = route.query.qid ? <string>route.query.qid : undefined;
// this flag tells to not store pages that have no question ID in history because they redirect to a new 
// random question catching the user in a loop
let replaceRouter = initialQid ? false : true;

// route.query.topic and .qid can potentially be an array, but it should not happen in this app,
// so it is safe to cast them into a string
const topic = ref<string>(initialTopic);
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
  // if the initial topic is ANY then we should fetch the next question for ANY
  store.loadQuestion(initialTopic, undefined);
};

// update the query string with the next question topic and id when the question changes in the store
watch(question, (newQuestion) => {
  // console.log("newQuestion: ", newQuestion);
  if (newQuestion) {
    topic.value = newQuestion.topic;
    qid.value = newQuestion.qid;
    console.log("question topic/qid: ", topic.value, qid.value);

    // update page title
    if (question.value?.title) {
      console.log("setting title to ", question.value?.title);
      window.document.title = question.value?.title;
      console.log("title set to ", window.document.title);
    }

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

// needed to load questions on back/forward navigation
watch(route, (newQuery) => {
  const newQid = newQuery.query.qid as string | undefined;
  const newTopic = newQuery.query.topic as string | undefined;
  console.log(`route.query ${newTopic} / ${newQid}`);
  if (newTopic && (newTopic != topic.value || newQid != qid.value)) {
    store.loadQuestion(newTopic, newQid);
  }
});

// load the question when the component is mounted
(async () => await store.loadQuestion(initialTopic, initialQid))();

</script>
