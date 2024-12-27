<template>
  <LinkButton href="/" :label="buttonLabel" icon="pi pi-sparkles" class="whitespace-nowrap" @click.prevent="loadNextQuestion" />
</template>


<script setup lang="ts">
import { computed } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { findTopicById, URL_PARAM_LIST_SEPARATOR, ANY_TOPIC } from "@/constants";

import LinkButton from './LinkButton.vue';

const store = useMainStore();
const { selectedTopics, currentTopic, questionStatus, showingRandomQuestion } = storeToRefs(store);


const buttonLabel = computed(() => {
  const partOfButtonLabel = showingRandomQuestion.value ? `Try another` : 'View';
  return `${partOfButtonLabel} random question`;

  if (selectedTopics.value.length == 1 && selectedTopics.value[0]) {
    return `${partOfButtonLabel} question about ${findTopicById(selectedTopics.value[0])}`
  }
  else if (selectedTopics.value.length > 1) {
    return `${partOfButtonLabel} question on selected topics`;

  }
  else {
    return `${partOfButtonLabel} random question`;
  }

});

// pull together currently selected topics and reload the question to show a new random one
const loadNextQuestion = () => {
  console.log("Load next question");

  // enable the sample question component via this flag
  showingRandomQuestion.value = true;

  // send all selected topics to the server, it will decided which one to return
  const topic = selectedTopics.value.length ? selectedTopics.value.join(URL_PARAM_LIST_SEPARATOR) : ANY_TOPIC;

  console.log(`next topic: ${topic}, currentTopic: ${currentTopic.value}`);

  store.loadQuestion(topic, undefined);
};

</script>
