<template>
  <Button v-if="selectedTopics.length == 1" :label="`${partOfButtonLabel} question about ${findTopicById(selectedTopics[0])}`" icon="pi pi-sparkles" severity="secondary" class="whitespace-nowrap" @click.prevent="loadNextQuestion" />
  <Button v-else-if="selectedTopics.length > 1" :label="`${partOfButtonLabel} question on selected topics`" icon="pi pi-sparkles" severity="secondary" class="whitespace-nowrap" @click.prevent="loadNextQuestion" />
  <Button v-else :label="`${partOfButtonLabel} random question`" icon="pi pi-sparkles" severity="secondary" class="" @click.prevent="loadNextQuestion" />
</template>


<script setup lang="ts">
import { computed } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { findTopicById, URL_PARAM_LIST_SEPARATOR, ANY_TOPIC } from "@/constants";

import Button from 'primevue/button';

const store = useMainStore();
const { selectedTopics, currentTopic, questionStatus, showingSampleQuestion } = storeToRefs(store);

const partOfButtonLabel = computed(() => showingSampleQuestion.value ? `Try another` : 'View a');

// pull together currently selected topics and reload the question to show a new random one
const loadNextQuestion = () => {
  console.log("Load next question");

  // enable the sample question component via this flag
  showingSampleQuestion.value = true;

  // send all selected topics to the server, it will decided which one to return
  const topic = selectedTopics.value.length ? selectedTopics.value.join(URL_PARAM_LIST_SEPARATOR) : ANY_TOPIC;

  console.log(`next topic: ${topic}, currentTopic: ${currentTopic.value}`);

  store.loadQuestion(topic, undefined);
};

</script>
