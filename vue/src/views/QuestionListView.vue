<template>
  <h1 class="mb-4 md:mb-8 text-2xl text-start">Questions about <em class="italic">{{ topicName }}</em></h1>
  <QuestionList />
  <div v-if="!isLoading && ctaBlockVisible" class="mb-12 md:mt-12 cta-box">
    <PostAnswerCTA />
  </div>
</template>

<script setup lang="ts">
import { computed, watch, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router'
import * as constants from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { useAuth0 } from '@auth0/auth0-vue';
import { LoadingStatus } from '@/interfaces';
import type { QuestionWithHistory } from '@/interfaces';

import PostAnswerCTA from '@/components/PostAnswerCTA.vue';
import QuestionList from '@/components/QuestionList.vue';

const route = useRoute();
const router = useRouter();

const store = useMainStore();
const { token, email, user } = storeToRefs(store);
const { isLoading } = useAuth0();

const topicName = computed(() => {
  return (constants.TOPICS.find((t) => t.id == route.query.topic))?.t;
});

// save the initial values to maintain state later because the topic and qid will change
const initialTopic = route.query.topic ? <string>route.query.topic : "";

const ctaBlockVisible = computed(() => {
  // checking user subs does not work because this page does not load user details
  // if  (!user.value?.topics.length && question.value?.answers?.[0].e) { return true } else { return false };
  if (email.value && user.value?.topics?.length) { return false } else { return true };
});


</script>
