<template>
  <div class="flex flex-wrap items-center gap-4 justify-center my-4">
    <div class="flex" v-for="topic in TOPICS" :key="topic.id">
      <Checkbox v-model="selectedTopics" class="dark:opacity-85" :value="topic.id" :input-id="topic.id" />
      <label :for="topic.id" class="ms-2 me-4">{{ topic.t }}</label>
    </div>
  </div>
</template>


<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { TOPICS } from "@/constants";
import { useMainStore } from '@/store';

import Checkbox from 'primevue/checkbox';

const store = useMainStore();
const { selectedTopics } = storeToRefs(store);

</script>
