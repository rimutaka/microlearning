<template>
  <h3>{{ questionMarkdown }}</h3>
</template>


<script setup lang="ts">
import { ref,  watchEffect } from "vue";

const props = defineProps<{
  topic: string
}>()

// as fetched from the server
const questionMarkdown = ref<string | undefined>();
watchEffect(async () => {
  console.log("fetching question for topic", props.topic);
  // only fetch if topic is set
  if (!props.topic) return;

  try {
    const response = await fetch(`https://bitesized.info/q?topic=${props.topic}`);
    const question = <Question>await response.json();
    console.log("question", question);
    questionMarkdown.value = question.question;
  } catch (error) {
    console.error(error);
  }
});

interface Question {
  qid: string,
  topic: string,
  question: string,
  answers: Array<string>,
  /// The list of correct answers.
  correct: Array<number>,
}

</script>
