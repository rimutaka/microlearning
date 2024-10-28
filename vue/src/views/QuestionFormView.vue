<template>
  <h3 v-if="route.query.qid" class="mb-8">Edit question</h3>
  <h3 v-else class="mb-8">Add new question</h3>
  <QuestionForm v-if="token" :topic="<string>route.query.topic" :qid="<string>route.query.qid" />
  <div class=" mt-12 mb-8 p-8 border-2 border-slate-300 rounded-md" v-else>
    <h3 class=" mt-4 mb-8">Sorry, I have not coded this part yet, but ...</h3>
    <p class="mb-4">I'd love to get your help with adding more questions.</p>
    <p>Can you email yours to <a class="link" href="mailto:max@onebro.me">max@onebro.me</a>?</p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import QuestionForm from '../components/QuestionForm.vue';
import { TOKEN_HEADER_NAME } from "@/constants";

const route = useRoute();

const token = computed(() => localStorage.getItem(TOKEN_HEADER_NAME));

</script>