<template>
  <div class="card mt-12">
    <h3>Select your topics of interest</h3>
    <TopicsList />

    <div class="flex flex-wrap my-12">
      <div class="flex-shrink text-start mx-auto">
        <Button label="Browse more questions or subscribe" icon="pi pi-envelope" raised rounded class="font-bold px-8 py-4 md:me-4 mb-2 whitespace-nowrap" @click="navigateToSubscription" />
        <p class="text-xs text-center md:mb-auto text-slate-500">You will be asked to sign in with <i class="pi pi-github ms-1 me-2"></i><i class="pi pi-google me-2"></i><i class="pi pi-microsoft me-2"></i><i class="pi pi-linkedin me-2"></i></p>
      </div>
      <p class="w-full text-center my-4">or</p>
      <div class="flex-grow text-center mb-4">
        <Button label="Try a random question now" icon="pi pi-sparkles" severity="secondary" rounded class="whitespace-nowrap" @click="showRandomQuestion" />
      </div>
    </div>
    <TransitionSlot>
      <SampleQuestion v-if="currentTopic" :topic="currentTopic" />
    </TransitionSlot>
  </div>
</template>


<script setup lang="ts">
import router from '@/router';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { TOPICS, URL_PARAM_LIST_SEPARATOR } from "@/constants";

import Button from 'primevue/button';
import TopicsList from './TopicsList.vue';
import TransitionSlot from "./TransitionSlot.vue";
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

async function navigateToSubscription() {
  console.log("Subscribing to topics: ", selectedTopics.value);
  const subTopics = selectedTopics.value.map((t) => t).join(URL_PARAM_LIST_SEPARATOR);
  if (subTopics) {
    router.push({ name: 'subscription', query: { topics: subTopics } });
  } else {
    router.push({ name: 'subscription' });
  }
}

</script>
