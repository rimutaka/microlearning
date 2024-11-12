<template>
  <h2 class="mt-8 mb-2 text-start">Question about: <em class="italic">{{ topicName }}</em></h2>
  <QuestionCard :topic="topic" :qid="qid" :next="true" :key="currentTopicKey"/>
</template>

<script setup lang="ts">
import { computed, watch, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router'
import QuestionCard from "../components/QuestionCard.vue";
// import SignupForm from '../components/SignupForm.vue';
// import SignupPitch from '../components/SignupPitch.vue';
import { TOPICS, ANY_TOPIC } from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

const route = useRoute();
const router = useRouter();

const store = useMainStore();
const { question, currentTopicKey } = storeToRefs(store);

const topicName = computed(() => {
  return (TOPICS.find((t) => t.id == route.query.topic))?.t;
});

const initialTopic = route.query.topic ? <string>route.query.topic : ANY_TOPIC;
const initialQid = route.query.qid ? <string>route.query.qid : undefined;

// route.query.topic and .qid can potentially be an array, but it should not happen in this app,
// so it is safe to cast them into a string
const topic = ref<string>(initialTopic);
const qid = ref<string | undefined>(initialQid);

// update the query string with the next question topic and id when the question changes in the store
watch(question, (newQuestion) => {
  console.log("newQuestion: ", newQuestion);
  if (newQuestion) {
    topic.value = newQuestion.topic;
    qid.value = newQuestion.qid;
    console.log("question topic/qid: ", topic.value, qid.value);

    if (route.query.topic != topic.value || route.query.qid != qid.value) {
      console.log("navigating");
      router.push({ query: { topic: topic.value, qid: qid.value } });
    }
  }
  else {
    console.log("question removed from store ");
    topic.value = initialTopic;
    qid.value = undefined;
    store.showRandomQuestion();
  }
});

// watch route changes
watch(() => [route.query.topic, route.query.qid], ([newTopic, newQid]) => {
  console.log("route changed: ", newTopic, newQid);
  if (newTopic != topic.value || newQid != qid.value) {
    console.log("updating store");
    topic.value = newTopic ? <string>newTopic : ANY_TOPIC;
    qid.value = newQid ? <string>newQid : undefined;
    store.showRandomQuestion();
  }
});


</script>
