<template>
  <div class=" border-t-2 border-slate-300">
    <h3 v-if="currentTopic" class="mt-8 mb-8 text-center">Showing a random question about <em class="italic">{{ findTopicById(currentTopic) }}</em></h3>
    <QuestionCard :key="props.nonce" />
  </div>
</template>


<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import QuestionCard from "./QuestionCard.vue";
import { findTopicById } from "@/constants";

const props = defineProps<{
  nonce?: string,
}>()

const store = useMainStore();
const { currentTopic } = storeToRefs(store);

</script>
