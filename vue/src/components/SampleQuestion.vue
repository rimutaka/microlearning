<template>
  <div class=" border-t-2 border-slate-300">
    <h3 class="mt-8 mb-8 text-center">Showing a random question about <em class="italic">{{ findTopicById(currentTopic) }}</em></h3>
    <QuestionCard />
    <ContributorCard />
  </div>
</template>


<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { findTopicById } from "@/constants";

import QuestionCard from "./QuestionCard.vue";
import ContributorCard from "./ContributorCard.vue";

const store = useMainStore();
const { currentTopic } = storeToRefs(store);

</script>
