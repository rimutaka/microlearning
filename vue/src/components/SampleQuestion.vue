<template>
  <div class=" border-t-2 border-slate-300">
    <h3 class="mt-8 mb-8 text-center">Showing a random question about <em class="italic">{{ findTopicById(props.topic) }}</em></h3>
    <QuestionCard :topic="props.topic" :key="props.nonce" />
  </div>
</template>


<script setup lang="ts">
import QuestionCard from "./QuestionCard.vue";
import { findTopicById } from "@/constants";

const props = defineProps<{
  topic: string,
  nonce?: string,
}>()

</script>
