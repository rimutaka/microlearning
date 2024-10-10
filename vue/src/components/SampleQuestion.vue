<template>
  <h3 class="mt-8 mb-4">Sample question about <span class="italic" >{{ props.topic }}</span></h3>
  <p class="mb-4">{{ questionMarkdown?.question }}</p>

  <ul>
    <li class="mb-2" v-for="(answer, index) in questionMarkdown?.answers" :key="index">
      <input type="radio" :name="questionMarkdown?.qid" :value="index" />
      {{ answer.a }}
    </li>
  </ul>
</template>


<script setup lang="ts">
import { ref,  watchEffect } from "vue";

const props = defineProps<{
  topic: string
}>()

// as fetched from the server
const questionMarkdown = ref<Question | undefined>();
watchEffect(async () => {
  console.log("fetching question for topic", props.topic);
  // only fetch if topic is set
  if (!props.topic) return;

  try {
    const response = await fetch(`https://bitesized.info/q?topic=${props.topic}`);
    const question = <Question>await response.json();
    console.log("question", question);
    questionMarkdown.value = question;
  } catch (error) {
    console.error(error);
  }
});

/// A mirror of the Rust's type
interface Answer {
  a: string,
  e: string,
  c: boolean,
}

/// A mirror of the Rust's type
interface Question {
  qid: string,
  topic: string,
  question: string,
  answers: Array<Answer>,
  correct: number,
}

</script>
