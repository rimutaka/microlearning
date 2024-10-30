<template>
  <Button v-if="selectedTopics.length == 1" :label="`${particle} question about ${findTopicById(selectedTopics[0])}`" icon="pi pi-sparkles" severity="secondary" class="whitespace-nowrap" @click="store.showRandomQuestion" />
  <Button v-else-if="selectedTopics.length > 1" :label="`${particle} question on selected topics`" icon="pi pi-sparkles" severity="secondary" class="whitespace-nowrap" @click="store.showRandomQuestion" />
  <Button v-else :label="`${particle} random question`" icon="pi pi-sparkles" severity="secondary" class="" @click="store.showRandomQuestion" />
</template>


<script setup lang="ts">
import { computed } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { findTopicById } from "@/constants";

import Button from 'primevue/button';

const store = useMainStore();
const { selectedTopics, currentTopic } = storeToRefs(store);

const particle = computed(() => currentTopic.value ? `Try another` : 'View a');

</script>
