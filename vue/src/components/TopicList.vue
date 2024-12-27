<template>
  <div class="flex flex-wrap items-center gap-4 justify-center my-4">
    <div class="flex" v-for="topic in TOPICS" :key="topic.id">
      <RadioButton v-if="asRadio" v-model="currentTopic" class="dark:opacity-85" :value="topic.id" :input-id="topic.id" />
      <Checkbox v-else v-model="selectedTopics" class="dark:opacity-85" :value="topic.id" :input-id="topic.id" />

      <router-link v-if="asLinks" :to="`/${PageIDs.QUESTIONS}?${URL_PARAM_TOPIC}=${topic.id}`" class="ms-2 me-4 no-decoration">{{ topic.t }}</router-link>
      <label v-else :for="topic.id" class="ms-2 me-4">{{ topic.t }}</label>
    </div>
  </div>
</template>


<script setup lang="ts">
import { watchEffect, watch } from 'vue';
import { storeToRefs } from 'pinia'
import { TOPICS, URL_PARAM_TOPIC } from "@/constants";
import { useMainStore } from '@/store';
import { PageIDs } from '@/router';
import { useRoute, useRouter } from 'vue-router'

import Checkbox from 'primevue/checkbox';
import RadioButton from 'primevue/radiobutton';

const props = defineProps<{ asRadio?: boolean, asLinks?: boolean }>();

const route = useRoute();
const router = useRouter();
const store = useMainStore();
const { selectedTopics, currentTopic } = storeToRefs(store);

// change the URL to match the current topic
watch(currentTopic, (newCurrentTopic) => {
  let urlTopic = route.query.topic ? <string>route.query[URL_PARAM_TOPIC] : "";

  if (newCurrentTopic && newCurrentTopic != urlTopic) {
    console.log(`Navigating to topic ${newCurrentTopic} from ${route.query[URL_PARAM_TOPIC]}`);
    router.push({ query: { [URL_PARAM_TOPIC]: newCurrentTopic } });
  }
});

watchEffect(() => {
  // this block is only needed to update the store when the topic is set through the URL
  if (props.asLinks) {
    currentTopic.value = route.query[URL_PARAM_TOPIC] ? <string>route.query[URL_PARAM_TOPIC] : "";
    selectedTopics.value = [currentTopic.value];
    console.log(`Selected topics: ${selectedTopics.value}`);
  }
});

</script>
