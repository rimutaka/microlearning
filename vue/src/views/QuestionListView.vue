<template>
  <h1 class="mb-4 md:mb-8 text-2xl text-start">Questions about <em class="italic">{{ topicName }}</em></h1>
  <QuestionList />
  <div v-if="!email" class="mb-12 mt-12 cta-box">
    <!-- Show it to logged out users -->
    <QuestionListCTASignin />
  </div>
  <div v-if="email && !user?.topics?.length" class="mb-12 mt-12 cta-box">
    <!-- Show it to logged in users with no subscriptions -->
    <QuestionListCTASubscribe />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router'
import * as constants from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import QuestionListCTASignin from '@/components/QuestionListCTASignin.vue';
import QuestionList from '@/components/QuestionList.vue';
import QuestionListCTASubscribe from '@/components/QuestionListCTASubscribe.vue';

const route = useRoute();

const store = useMainStore();
const { email, user } = storeToRefs(store);

const topicName = computed(() => {
  return (constants.TOPICS.find((t) => t.id == route.query.topic))?.t;
});

</script>