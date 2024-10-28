<template>
  <div class="card mt-8">
    <h3>Select your topics of interest</h3>
    <TopicsList />

    <p class="text-center text-red-600 text-sm" :class="topicsReminder && !canSubscribe ? 'visible': 'invisible'">Select at least one topic</p>

    <div class="mt-4 mb-12">
      <div class="text-center">
        <Button label="Subscribe" icon="pi pi-envelope" raised rounded class="font-bold px-8 py-4 md:me-4 mb-2 whitespace-nowrap" @click="subscribe" />
      </div>
      <p class="w-full text-center mt-2 mb-4 text-sm">or</p>

      <div class="text-center mb-4">
        <Button v-if="lastSelectedTopic" :label="`View a question about ${findTopicById(lastSelectedTopic)}`" icon="pi pi-sparkles" severity="secondary" rounded class="whitespace-nowrap" @click="showRandomQuestion" />
        <Button v-else label="`View questions about selected topics" icon="pi pi-sparkles" severity="secondary" rounded class="whitespace-nowrap" @click="showRandomQuestion" />
      </div>
    </div>
    <TransitionSlot>
      <SampleQuestion v-if="sampleQuestionTopic" :topic="sampleQuestionTopic" />
    </TransitionSlot>
  </div>
</template>


<script setup lang="ts">
import { computed, ref, watchEffect } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { USER_HANDLER_URL, TOKEN_HEADER_NAME, URL_PARAM_TOPICS, URL_PARAM_LIST_SEPARATOR, findTopicById } from "@/constants";

import Button from 'primevue/button';
import TopicsList from './TopicsList.vue';
import TransitionSlot from "./TransitionSlot.vue";
import SampleQuestion from "./SampleQuestion.vue";

const store = useMainStore();
const { selectedTopics, lastSelectedTopic, token } = storeToRefs(store);
const topicsReminder = ref(false); // true if attempted to subscribe without selecting topics to show a prompt
const sampleQuestionTopic = ref<string | undefined>();

const canSubscribe = computed(() => selectedTopics.value.length > 0);


/// Show a random question from the selected topics or all topics
function showRandomQuestion() {
    // if no topics are selected, show a prompt and return
    if (!canSubscribe.value) {
    topicsReminder.value = true;
    return;
  }
  
  console.log("showRandomQuestion", lastSelectedTopic.value);
  sampleQuestionTopic.value = lastSelectedTopic.value;
}

async function subscribe() {
  console.log("Subscribing to topics: ", selectedTopics.value);
  const subTopics = selectedTopics.value.map((t) => t).join(URL_PARAM_LIST_SEPARATOR);

  // if no topics are selected, show a prompt and return
  if (!canSubscribe.value) {
    topicsReminder.value = true;
    return;
  }

  // prepare the list of topics to subscribe to

  // redirect the user to login with the list of topics as a parameter
  if (!token.value) {
    console.log("No token found.");
    return; // unreachable code
  }

  // create a URL with list of topics
  const url = `${USER_HANDLER_URL}${URL_PARAM_TOPICS}=${subTopics}`;
  try {
    const response = await fetch(url, {
      headers: {
        [TOKEN_HEADER_NAME]: token.value,
      },
    });

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 204) { console.log("Subscribed successfully"); }
    else {
      console.error("Failed to subscribe: ", response.status);
    }
  } catch (error) {
    console.error(error);
  }
}

</script>
