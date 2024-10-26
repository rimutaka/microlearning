<template>
  <div class="card mt-12">
    <h3>Select your topics of interest</h3>
    <TopicsList />

    <div class="flex flex-wrap md:gap-12 my-12">
      <div class="flex-grow md:flex-shrink text-center md:text-start mb-4 md:mb-auto">
        <Button label="Try a random question" icon="pi pi-sparkles" severity="secondary" rounded class="whitespace-nowrap" @click="showRandomQuestion" />
      </div>
      <p class="md:hidden w-full text-center mb-4">or</p>
      <SubscribeBlock class="flex-shrink text-start mx-auto" />
    </div>
    <TransitionSlot>
      <SampleQuestion v-if="currentTopic" :topic="currentTopic" />
    </TransitionSlot>
  </div>
</template>


<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { TOPICS } from "@/constants";

import Button from 'primevue/button';
import TopicsList from './TopicsList.vue';
import TransitionSlot from "./TransitionSlot.vue";
import SubscribeBlock from './SubscribeBlock.vue';
import SampleQuestion from "./SampleQuestion.vue";

const store = useMainStore();
const { selectedTopics, currentTopic } = storeToRefs(store);

/// Show a random question from the selected topics or all topics
function showRandomQuestion() {
  if (selectedTopics.value.length) {
    currentTopic.value = selectedTopics.value[Math.floor(Math.random() * selectedTopics.value.length)];
  } else {
    currentTopic.value = TOPICS[Math.floor(Math.random() * TOPICS.length)].id;
  }
  console.log("showRandomQuestion", currentTopic.value);
}

</script>
