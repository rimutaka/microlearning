<template>
  <h1 class="mb-4 md:mb-8 text-2xl text-start">Questions about <em class="italic">{{ topicName }}</em></h1>
  <QuestionList />
  <div v-if="!email" class="mb-12 mt-12 cta-box">
    <QuestionListCTA />
  </div>
</template>

<script setup lang="ts">
import { computed, watch, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router'
import * as constants from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { useAuth0 } from '@auth0/auth0-vue';

import QuestionListCTA from '@/components/QuestionListCTA.vue';
import QuestionList from '@/components/QuestionList.vue';

const route = useRoute();
const router = useRouter();

const store = useMainStore();
const { email } = storeToRefs(store);

const topicName = computed(() => {
  return (constants.TOPICS.find((t) => t.id == route.query.topic))?.t;
});

</script>