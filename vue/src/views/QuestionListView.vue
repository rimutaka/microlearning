<template>
  <TopicsList :as-links="true" />
  <div v-if="topic">
    <QuestionList :topic="topic" :key="topic" class="mt-12" />
    <div v-if="!email" class="mb-12 mt-12 cta-box">
      <!-- Show it to logged out users -->
      <QuestionListCTASignin />
    </div>
    <div v-if="email && !user?.topics?.length" class="mb-12 mt-12 cta-box">
      <!-- Show it to logged in users with no subscriptions -->
      <QuestionListCTASubscribe />
    </div>
  </div>
  <LoadingMessage v-else msg="Select a topic to view the list of questions" class="mt-12" />
</template>

<script setup lang="ts">
import { computed, ref, watchEffect } from 'vue';
import { useRoute } from 'vue-router'
import * as constants from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import QuestionListCTASignin from '@/components/QuestionListCTASignin.vue';
import QuestionList from '@/components/QuestionList.vue';
import QuestionListCTASubscribe from '@/components/QuestionListCTASubscribe.vue';
import TopicsList from '@/components/TopicsList.vue';
import LoadingMessage from '@/components/LoadingMessage.vue';

const route = useRoute();

const store = useMainStore();
const { email, user } = storeToRefs(store);

const topic = ref();

const topicName = computed(() => {
  return (constants.TOPICS.find((t) => t.id == route.query.topic))?.t;
});

watchEffect(() => {
  topic.value = route.query.topic ? <string>route.query.topic : "";
});

</script>