<template>
  <div class="flex flex-col items-center">
    <Button :label="buttonLabel" icon="pi pi-sparkles" size="small" severity="secondary" class="whitespace-nowrap" @click.capture="loadNextQuestion" />
    <p v-if="mustSelectTopic && !currentTopic" class="text-red-500 text-sm mt-2 max-w-fit">Select a topic first</p>
  </div>
</template>


<script setup lang="ts">
import { computed, watch, ref } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { findTopicById, URL_PARAM_LIST_SEPARATOR, ANY_TOPIC, TOPICS, DEFAULT_TOPIC } from "@/constants";
import { fetchQuestions } from '@/data-loaders/fetch-questions';

import LinkButton from './LinkButton.vue';
import Button from 'primevue/button';

const store = useMainStore();
const { selectedTopics, currentTopic, questionStatus, showingRandomQuestion, questionsWithHistory, question } = storeToRefs(store);

// True if the user requested a random question without selecting a topic
const mustSelectTopic = ref(false);

const buttonLabel = computed(() => {
  const partOfButtonLabel = showingRandomQuestion.value ? `Try another` : 'View';
  return `${partOfButtonLabel} random question`;
});

// pull together currently selected topics and reload the question to show a new random one
const loadNextQuestion = async () => {
  console.log("Load next question");

  if (!currentTopic.value) {
    mustSelectTopic.value = true;
    return;
  }

  // load the list of questions if none exists, or exists for a different topic
  if ((!questionsWithHistory.value || questionsWithHistory.value.length === 0) || questionsWithHistory.value[0].question.topic !== currentTopic.value) {
    // load the list of questions for the given topic
    questionsWithHistory.value = await fetchQuestions(currentTopic.value)
  }

  // TODO: report the error to the user
  if (!questionsWithHistory.value || questionsWithHistory.value.length === 0) {
    console.error("No questions found for the topic: ", currentTopic.value);
    return;
  }

  // find the index of the current question by ID and pick the next one from the list or start from the beginning
  const currentQuestionIndex = (question.value ? questionsWithHistory.value.findIndex(q => q.question.qid === question.value?.qid) : -1) + 1;
  const nextQuestionIndex = currentQuestionIndex < questionsWithHistory.value.length ? currentQuestionIndex : 0;
  console.log(`currentQuestionIndex: ${currentQuestionIndex}, nextQuestionIndex: ${nextQuestionIndex}`);

  // enable the sample question component via this flag
  showingRandomQuestion.value = true;

  // load the next question
  store.loadQuestion(currentTopic.value, questionsWithHistory.value[nextQuestionIndex].question.qid);
};

// change the URL to match the current topic
watch(currentTopic, (newCurrentTopic) => {
  mustSelectTopic.value = newCurrentTopic ? true : false;
  // load the list of questions if none exists, or exists for a different topic
  if (newCurrentTopic && questionsWithHistory.value?.[0].question.topic !== newCurrentTopic) {
    showingRandomQuestion.value = false;
    question.value = undefined;
    // load the list of questions for the given topic
    fetchQuestions(newCurrentTopic).then((fetchedQuestions) => {
      questionsWithHistory.value = fetchedQuestions;
    });
  }
});

</script>
