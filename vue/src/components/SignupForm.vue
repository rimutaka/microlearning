<template>
  <div class="card mt-12">
    <h3>Topics</h3>
    <div class="flex flex-wrap items-center gap-4 justify-center my-4">
      <div class="flex" v-for="topic in topics" :key="topic.id">
        <Checkbox v-model="selectedTopics" name="topics" :value="topic.id" />
        <label :for="topic.id" class="ms-2 me-4">{{ topic.t }}</label>
      </div>
    </div>
    <div class="flex flex-wrap items-center gap-4 justify-center my-8">
      <InputText type="text" v-model="email" placeholder="Your email" />
      <Button label="Subscribe" icon="pi pi-discord" raised rounded class="font-bold px-8 py-4 whitespace-nowrap" />
    </div>
    <SampleQuestion v-if="currentTopic" :topic="currentTopic" />
  </div>
</template>


<script setup lang="ts">
import { ref, watch } from "vue";
import Button from 'primevue/button';
import Checkbox from 'primevue/checkbox';
import InputText from 'primevue/inputtext';
import SampleQuestion from "./SampleQuestion.vue";
import { TOPICS } from "@/constants";

const topics = ref(TOPICS);
const selectedTopics = ref<Array<string>>([]);
const email = ref("");
const currentTopic = ref<string>("");

watch(selectedTopics, (newVal, oldVal) => {
  if (newVal.length > 0) currentTopic.value = newVal[newVal.length - 1];
  console.log("currentTopic", currentTopic.value);
});

</script>
