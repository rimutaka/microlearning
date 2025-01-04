<template>
  <LoadingMessage v-if="!user" msg="Loading your subscription ..." />
  <h3 v-if="user?.topics.length">Your subscribed topics</h3>
  <h3 v-else-if="user && !user.topics.length">Select topics to subscribe</h3>
  <div v-if="user" :key="user?.updated" class="flex flex-wrap items-center gap-4 justify-center my-4">
    <div class="flex" v-for="topic in TOPICS" :key="topic.id">
      <Checkbox v-model="selectedTopics" class="dark:opacity-85" :value="topic.id" :input-id="topic.id" />
      <span class="ms-2 me-4 link" @click.capture="router.push({ query: { [URL_PARAM_TOPIC]: topic.id } })">{{ topic.t }}</span>
    </div>
  </div>

  <p v-if="user?.topics.length" class="mb-4 text-xs text-slate-500 dark:text-slate-200">Last updated: {{ updateDate }} </p>
  <SubscriptionForm class="mb-12" />
  <TransitionSlot>
    <SubscriptionCTA v-if="user && !user?.topics.length && !questionsWithHistory?.length" />
  </TransitionSlot>
  <TransitionSlot>
    <SubscriptionCompleted v-if="firstTimeSub && user?.topics.length" />
  </TransitionSlot>
  <h3 v-if="topic && questionsWithHistory?.length" class="my-8">Questions about {{ topicName }}</h3>
  <h3 v-else-if="user && questionsWithHistory?.length" class="my-8">Your questions</h3>
  <QuestionList v-if="user" :topic="topic" :key="topic" />
</template>

<script setup lang="ts">
import { ref, computed, watch, watchEffect } from 'vue';
import { storeToRefs } from 'pinia'
import { useRoute, useRouter } from 'vue-router'
import { useMainStore } from '@/store';
import { TOPICS, URL_PARAM_TOPIC, findTopicById } from "@/constants";


import TransitionSlot from "@/components/TransitionSlot.vue";
import SubscriptionCTA from '@/components/SubscriptionCTA.vue';
import SubscriptionCompleted from '@/components/SubscriptionCompleted.vue';
import SubscriptionForm from '@/components/SubscriptionForm.vue';
import TopicList from '@/components/TopicList.vue';
import LoadingMessage from '@/components/LoadingMessage.vue';
import QuestionList from '@/components/QuestionList.vue';
import Checkbox from 'primevue/checkbox';

const route = useRoute();
const router = useRouter();

const store = useMainStore();
const { user, questionsWithHistory, selectedTopics } = storeToRefs(store);

const firstTimeSub = ref(false); // true when the user changes from no sub to sub with topics
const topic = ref<string | undefined>();
const topicName = computed(() => findTopicById(topic.value));

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

watchEffect(() => {
  topic.value = route.query.topic ? <string>route.query.topic : undefined;
});

</script>
