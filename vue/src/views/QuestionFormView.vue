<script setup lang="ts">
import QuestionForm from '../components/QuestionForm.vue';

</script>

<template>
  <QuestionForm :topic="<string>$route.query.topic" :qid="<string>$route.query.qid"/>
</template>
