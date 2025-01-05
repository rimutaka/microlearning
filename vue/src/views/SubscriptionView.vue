<template>
  <LoadingMessage v-if="!user" msg="Loading your subscription ..." />
  <SubscriptionTopics v-if="user" />

  <SubscriptionForm class="mb-12" />
  <TransitionSlot>
    <SubscriptionCTA v-if="user && !user?.topics.length && !questionsWithHistory?.length" />
  </TransitionSlot>
  <TransitionSlot>
    <SubscriptionCompleted v-if="user?.topics.length && !questionsWithHistory?.length" />
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
import LoadingMessage from '@/components/LoadingMessage.vue';
import QuestionList from '@/components/QuestionList.vue';
import SubscriptionTopics from '@/components/SubscriptionTopics.vue';

const route = useRoute();
const router = useRouter();

const store = useMainStore();
const { user, questionsWithHistory, selectedTopics } = storeToRefs(store);

const firstTimeSub = ref(false); // true when the user changes from no sub to sub with topics
const topic = ref<string | undefined>();
const topicName = computed(() => findTopicById(topic.value));

watch(user, (newUser) => {
  if (newUser && !newUser.topics.length) {
    firstTimeSub.value = true;
  }
});

watchEffect(() => {
  topic.value = route.query.topic ? <string>route.query.topic : undefined;
});

</script>
