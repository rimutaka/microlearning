<template>
  <div class="card mt-12">
    <h3>Select your topics of interest</h3>
    <div class="flex flex-wrap items-center gap-4 justify-center my-4">
      <div class="flex" v-for="topic in TOPICS" :key="topic.id">
        <Checkbox v-model="selectedTopics" name="topics" :value="topic.id" :input-id="topic.id" />
        <label :for="topic.id" class="ms-2 me-4">{{ topic.t }}</label>
      </div>
    </div>

    <div class="flex flex-wrap md:gap-12 my-12">
      <div class="flex-grow md:flex-shrink text-center md:text-start mb-4 md:mb-auto">
        <Button label="Try a random question" icon="pi pi-sparkles" severity="secondary" rounded class="whitespace-nowrap" @click="showRandomQuestion" />
      </div>
      <p class="md:hidden w-full text-center mb-4">or subscribe</p>
      <div class="flex-shrink text-end">
        <div class="flex">
          <div class="flex-shrink">
            <Button label="Subscribe" icon="pi pi-envelope" raised rounded class="font-bold px-8 py-4 me-4 whitespace-nowrap" :disabled="!canSubscribe" @click="login" />
          </div>
          <div class=flex-grow>
            <p v-if="!selectedTopics.length" class="text-xs text-end text-slate-500">Select at least one topic</p>
            <p v-else class="text-xs text-start md:mb-auto text-slate-500">One new question per day.<br />Auto-paused if you get a backlog.<br />One-click unsubscribe.</p>
          </div>
        </div>
      </div>
    </div>
    <TransitionSlot>
      <SampleQuestion v-if="currentTopic" :topic="currentTopic" />
    </TransitionSlot>
  </div>
</template>


<script setup lang="ts">
import { useAuth0 } from '@auth0/auth0-vue';
import { ref, computed } from "vue";
import { TOPICS } from "@/constants";

import Button from 'primevue/button';
import Checkbox from 'primevue/checkbox';
import SampleQuestion from "./SampleQuestion.vue";
import TransitionSlot from "./TransitionSlot.vue";

const { loginWithRedirect, isAuthenticated } = useAuth0();

const selectedTopics = ref<Array<string>>([]);
const currentTopic = ref<string>("");

const canSubscribe = computed(() => selectedTopics.value.length > 0);

/// Show a random question from the selected topics or all topics
function showRandomQuestion() {
  if (selectedTopics.value.length) {
    currentTopic.value = selectedTopics.value[Math.floor(Math.random() * selectedTopics.value.length)];
  } else {
    currentTopic.value = TOPICS[Math.floor(Math.random() * TOPICS.length)].id;
  }
}

/// Auth0 login
function login() {
  if (!isAuthenticated.value) {
    loginWithRedirect();
  } else {
    console.log("Already authenticated");
  }
}

</script>
