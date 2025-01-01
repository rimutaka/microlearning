<template>
  <LoadingMessage v-if="!user" msg="Loading your subscription ..." />
  <h3 v-if="user?.topics.length">Your subscribed topics</h3>
  <h3 v-else-if="user && !user.topics.length">Select topics to subscribe</h3>
  <TopicList v-if="user" :key="user?.updated" />
  <p v-if="user?.topics.length" class="mb-4 text-xs text-slate-500 dark:text-slate-200">Last updated: {{ updateDate }} </p>
  <SubscriptionForm class="mb-12" />
  <TransitionSlot>
    <SubscriptionCTA v-if="user && !user?.topics.length && !questionsWithHistory?.length" />
  </TransitionSlot>
  <TransitionSlot>
    <SubscriptionCompleted v-if="firstTimeSub && user?.topics.length" />
  </TransitionSlot>
  <h3 v-if="user && questionsWithHistory?.length" class="my-8">Your questions</h3>
  <QuestionList v-if="user" />
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import TransitionSlot from "@/components/TransitionSlot.vue";
import SubscriptionCTA from '@/components/SubscriptionCTA.vue';
import SubscriptionCompleted from '@/components/SubscriptionCompleted.vue';
import SubscriptionForm from '@/components/SubscriptionForm.vue';
import TopicList from '@/components/TopicList.vue';
import LoadingMessage from '@/components/LoadingMessage.vue';
import QuestionList from '@/components/QuestionList.vue';

const store = useMainStore();
const { user, questionsWithHistory } = storeToRefs(store);
const firstTimeSub = ref(false); // true when the user changes from no sub to sub with topics

/// Format RFC3339 date string to a human-readable date
const updateDate = computed(() => {
  if (!user.value?.updated) return "";
  return new Date(user.value.updated).toLocaleString();
});

watch(user, (newUser) => {
  if (newUser && !newUser.topics.length) {
    firstTimeSub.value = true;
  }
});

</script>
