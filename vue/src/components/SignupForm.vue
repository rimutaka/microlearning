<template>
  <div class="card mt-12">
    <h3>Select your topics of interest</h3>
    <div class="flex flex-wrap items-center gap-4 justify-center my-4">
      <div class="flex" v-for="topic in TOPICS" :key="topic.id">
        <Checkbox v-model="selectedTopics" name="topics" :value="topic.id" :input-id="topic.id" />
        <label :for="topic.id" class="ms-2 me-4">{{ topic.t }}</label>
      </div>
    </div>

    <div class="flex flex-wrap items-center gap-4 justify-center my-12">
      <div class=" flex-grow text-start  mb-auto">
        <Button label="Try a random question" icon="pi pi-sparkles" severity="secondary" rounded class="whitespace-nowrap" @click="showRandomQuestion" />
      </div>
      <div class="flex-shrink">
        <InputText type="email" v-model="email" placeholder="Your email" name="email" />
        <Button label="Subscribe" icon="pi pi-envelope" raised rounded class="font-bold px-8 py-4 ms-4 whitespace-nowrap" :disabled="!canSubscribe" />
        <p v-if="email && !selectedTopics.length" class="mt-2 text-sm text-start text-slate-500">Select at least one topic</p>
      </div>

    </div>
    <TransitionSlot>
      <SampleQuestion v-if="currentTopic" :topic="currentTopic" />
    </TransitionSlot>
  </div>
</template>


<script setup lang="ts">
import { ref, watch, computed } from "vue";
import Button from 'primevue/button';
import Checkbox from 'primevue/checkbox';
import InputText from 'primevue/inputtext';
import SampleQuestion from "./SampleQuestion.vue";
import { TOPICS } from "@/constants";
import TransitionSlot from "./TransitionSlot.vue";

const selectedTopics = ref<Array<string>>([]);
const email = ref("");
const currentTopic = ref<string>("");

const canSubscribe = computed(() => selectedTopics.value.length > 0 && email.value.length > 0);

/// Show a random question from the selected topics or all topics
function showRandomQuestion() {
  if (selectedTopics.value.length) {
    currentTopic.value = selectedTopics.value[Math.floor(Math.random() * selectedTopics.value.length)];
  } else {
    currentTopic.value = TOPICS[Math.floor(Math.random() * TOPICS.length)].id;
  }
}


</script>
