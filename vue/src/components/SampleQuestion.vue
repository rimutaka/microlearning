<template>
  <h3 class="mt-8 mb-4">Sample question about <em class="italic">{{ props.topic }}</em></h3>
  <p class="mb-4" v-html="questionMarkdown?.question"></p>

  <div class="mb-2" v-for="(answer, index) in questionMarkdown?.answers" :key="index">
    <input type="radio" :name="questionMarkdown?.qid" :value="index" />
    <div v-html="answer.a"></div>
  </div>
</template>


<script setup lang="ts">
import { ref, watchEffect } from "vue";
// import { Writr } from 'writr';

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

    // https://www.npmjs.com/package/writr?activeTab=readme
    // const writr = new Writr(question.question, {
    //   renderOptions: {

    //     emoji: false,
    //     toc: false,
    //     slug: false,
    //     highlight: false,
    //     gfm: true,
    //     math: false,
    //     mdx: false,
    //     caching: false,
    //   }
    // });
    // const q = await writr.render();
    // console.log("q", q);

    // question.question = q;


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
