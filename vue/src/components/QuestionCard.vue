<template>
  <div class="flex">
    <div></div>
    <div class="q-card">

      <div class="q-text" v-html="questionMarkdown?.question"></div>

      <h3 class="mb-4">Answers</h3>

      <div class="mb-8 border-2" :class="{ 'border-green-100': answer?.c, 'border-red-100': !answer?.c && isAnswered, 'border-slate-300': !isAnswered }" v-for="(answer, index) in questionMarkdown?.answers" :key="index">
        <div class="flex items-center" :class="{ 'bg-green-100': answer?.c, 'bg-red-100': !answer?.c && isAnswered, 'bg-slate-300': !isAnswered }">

          <input type="radio" v-if="!isAnswered && questionMarkdown?.correct == 1" :name="questionMarkdown?.qid" :value="index" v-model="learnerAnswerRadio" />
          <input type="checkbox" v-if="!isAnswered && questionMarkdown?.correct && questionMarkdown.correct > 1" :name="questionMarkdown?.qid" :value="index" v-model="learnerAnswersCheck" />
          <div class="q-answer" v-html="answer.a"></div>

        </div>

        <div v-if="answer?.c" class="px-2">Correct.</div>
        <div v-else-if="isAnswered" class="px-2">Incorrect.</div>
        <div class="q-explain" v-if="answer?.e" v-html="answer.e"></div>
      </div>
    </div>
    <div></div>
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect, computed, watch } from "vue";
import type { Question } from "@/constants";
import { QUESTION_HANDLER_URL, URL_PARAM_QID, URL_PARAM_TOPIC } from "@/constants";
// import { Writr } from 'writr';

const props = defineProps<{
  topic: string,
  qid?: string,
  withAnswers?: boolean
}>()

// as fetched from the server
const questionMarkdown = ref<Question | undefined>();
const learnerAnswersCheck = ref<string[]>([]);
const learnerAnswerRadio = ref<string | undefined>();

const isAnswered = computed(() => {
  if (questionMarkdown.value?.answers?.[0].e) { return true } else { return false };
});

watchEffect(async () => {
  console.log("fetching question for topic", props.topic);
  // only fetch if topic is set
  if (!props.topic) return;

  try {

    // fetching by topic returns a random question
    // fetching with qid returns a specific question
    const fetchParams = `${URL_PARAM_TOPIC}=${props.topic}`.concat(props.qid ? `&${URL_PARAM_QID}=${props.qid}` : "");
    console.log("fetchParams", fetchParams);

    // by default, lambda removes answer details (correct, explanation) from the response
    const getHeaders = new Headers();
    if (props.withAnswers) {
      getHeaders.append("x-bitie-w-answers", "true");
    }

    const response = await fetch(`${QUESTION_HANDLER_URL}${fetchParams}`, {
      method: "GET",
      headers: getHeaders,
    });

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      try {
        const question = <Question>await response.json();
        console.log(question);
        // console.log(question.topic);
        // console.log(question.qid);

        questionMarkdown.value = question;

        // useRouter().push(`/question/${savedQuestion.topic}/${savedQuestion.qid}`);
      } catch (error) {
        console.error(error);
      }
    } else {
      console.error("Failed to get question. Status: ", response.status);
    }

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

  } catch (error) {
    console.error("Failed to get question.");
    console.error(error);
  }
});

</script>
