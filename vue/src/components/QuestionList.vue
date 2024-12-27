<template>
  <div>
    <LoadingMessage v-if="questionListStatus == undefined || questionListStatus == LoadingStatus.Loading" :msg="`Loading questions about ${topicName}`" />
    <div v-if="questionListStatus == LoadingStatus.Loaded" class="q-list">
      <QuestionListItem v-for="(question, index) in questions" :question="question" :key="index" />
    </div>
    <div v-if="questionListStatus == LoadingStatus.Error || questionListStatus == LoadingStatus.NoData">Cannot load the list of questions - something went wrong.</div>
  </div>
</template>

<script setup lang="ts">
import { computed, watchEffect, ref } from 'vue';
import * as constants from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { LoadingStatus } from '@/interfaces';
import type { QuestionWithHistory } from '@/interfaces';
import { fetchQuestions } from '@/data-loaders/fetch-questions';

import QuestionListItem from '@/components/QuestionListItem.vue';
import LoadingMessage from '@/components/LoadingMessage.vue';

const store = useMainStore();
const { token, questionListStatus } = storeToRefs(store);

const props = defineProps<{ topic: string }>();

const questions = ref<QuestionWithHistory[] | undefined>([]);

const topicName = computed(() => {
  return (constants.TOPICS.find((t) => t.id == props.topic))?.t;
});

/// The topic always comes from props.topic
/// The qid comes from props.qid if random is false.
const loadQuestions = async (paramTopic: string) => {
  // make sure nothing is showing while it's loading or if the component is reused
  questionListStatus.value = LoadingStatus.Loading;
  questions.value = undefined;

  // load the data
  questions.value = await fetchQuestions(paramTopic, token.value);

  // update the status
  if (questions.value && questions.value.length) {
    questionListStatus.value = LoadingStatus.Loaded;
  } else {
    questionListStatus.value = LoadingStatus.Error;
  }
};

watchEffect(async () => {
  await loadQuestions(props.topic);
});

</script>
