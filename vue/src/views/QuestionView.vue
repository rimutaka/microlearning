<template>
  <h2 class="mt-8 mb-2 text-start">Question about: <em class="italic">{{ topicName }}</em></h2>
  <QuestionCard :topic="topic" :qid="qid" :standalone="true" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router'
import QuestionCard from "../components/QuestionCard.vue";
// import SignupForm from '../components/SignupForm.vue';
// import SignupPitch from '../components/SignupPitch.vue';
import { TOPICS, ANY_TOPIC } from "@/constants";

const route = useRoute();

const topicName = computed(() => {
  return (TOPICS.find((t) => t.id == route.query.topic))?.t;
});

// route.query.topic and .qid can potentially be an array, but it should not happen in this app,
// so it is safe to cast them into a string
const topic = computed(() => route.query.topic ? <string>route.query.topic : ANY_TOPIC);
const qid = computed(() => route.query.qid ? <string>route.query.qid : undefined);

</script>
