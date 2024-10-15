<template>
  <h3 class="mt-8 mb-4 text-start">Random question about <em class="italic">{{ findTopicById(props.topic) }}</em></h3>
  <QuestionCard :topic="props.topic" />
</template>


<script setup lang="ts">
import QuestionCard from "./QuestionCard.vue";
import { findTopicById } from "@/constants";

const props = defineProps<{
  topic: string
}>()

</script>
