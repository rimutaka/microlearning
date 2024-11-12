<template>
  <div class=" border-t-2 border-slate-300">
    <h3 v-if="currentTopic" class="mt-8 mb-8 text-center">Showing a random question about <em class="italic">{{ findTopicById(currentTopic) }}</em></h3>
    <QuestionCard :topic="topic" :key="props.nonce" />
  </div>
</template>


<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import QuestionCard from "./QuestionCard.vue";
import { findTopicById, URL_PARAM_LIST_SEPARATOR, ANY_TOPIC } from "@/constants";
import { computed } from 'vue';

const props = defineProps<{
  nonce?: string,
}>()

const store = useMainStore();
const { currentTopic, selectedTopics } = storeToRefs(store);

const topic = computed(() => selectedTopics.value.length ? selectedTopics.value.join(URL_PARAM_LIST_SEPARATOR) : ANY_TOPIC);

</script>
