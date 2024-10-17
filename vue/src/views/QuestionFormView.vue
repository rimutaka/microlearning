<script setup lang="ts">
import QuestionForm from '../components/QuestionForm.vue';

</script>

<template>
  <h3 v-if="$route.query.qid" class="mb-8">Edit question</h3>
  <h3 v-else class="mb-8">Add new question</h3>
  <QuestionForm :topic="<string>$route.query.topic" :qid="<string>$route.query.qid" />
</template>
