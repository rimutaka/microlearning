<template>
  <h2 class="mt-8 mb-2 text-start">Question about: <em class="italic">{{ (TOPICS.find((t) => t.id == route.query.topic))?.t }}</em></h2>
  <QuestionCard :topic="<string>route.query.topic" :qid="<string>route.query.qid" />
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router'
import QuestionCard from "../components/QuestionCard.vue";
// import SignupForm from '../components/SignupForm.vue';
// import SignupPitch from '../components/SignupPitch.vue';
import { TOPICS } from "@/constants";

const route = useRoute();

</script>
