<template>
  <div class="flex flex-wrap items-center gap-4 justify-center my-4">
    <div class="flex" v-for="topic in TOPICS" :key="topic.id">
      <RadioButton v-if="asRadio" v-model="localCurrentTopic" class="dark:opacity-85" :value="topic.id" :input-id="topic.id" />
      <Checkbox v-else v-model="selectedTopics" class="dark:opacity-85" :value="topic.id" :input-id="topic.id" />

      <router-link v-if="asLinks" :to="`/${PageIDs.QUESTIONS}?${URL_PARAM_TOPIC}=${topic.id}`" class="ms-2 me-4 no-decoration">{{ topic.t }}</router-link>
      <label v-else :for="topic.id" class="ms-2 me-4">{{ topic.t }}</label>
    </div>
  </div>
</template>


<script setup lang="ts">
import { watchEffect, watch, ref } from 'vue';
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

const initialTopic = currentTopic.value;
const localCurrentTopic = ref(initialTopic);

watch(localCurrentTopic, (newVal) => {
  console.log(`Local current topic changed to ${newVal}`);
  let urlTopic = route.query.topic ? <string>route.query[URL_PARAM_TOPIC] : "";

  if (newVal && newVal != urlTopic) {
    console.log(`Navigating to topic ${newVal} from ${route.query[URL_PARAM_TOPIC]}`);
    router.push({ query: { [URL_PARAM_TOPIC]: newVal } });
  }
});

</script>
