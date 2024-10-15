<template>
  <div class="flex" v-if="questionMarkdown">
    <div></div>
    <div class="q-card">

      <div class="q-text" v-html="questionMarkdown?.question"></div>

      <h3 class="mb-4">Answers</h3>

      <div class="mb-8 border-2" :class="{ 'border-green-100': answer?.c, 'border-red-100': !answer?.c && isAnswered, 'border-slate-100': !isAnswered }" v-for="(answer, index) in questionMarkdown?.answers" :key="index">
        <div class="flex items-center" :class="{ 'bg-green-100': answer?.c, 'bg-red-100': !answer?.c && isAnswered, 'bg-slate-100': !isAnswered }">

          <input type="radio" v-if="questionMarkdown?.correct == 1" :name="questionMarkdown?.qid" :value="index" :disabled="isAnswered" v-model="learnerAnswerRadio" />
          <input type="checkbox" v-if="questionMarkdown?.correct && questionMarkdown.correct > 1" :name="questionMarkdown?.qid" :disabled="isAnswered" :value="index" v-model="learnerAnswersCheck" />
          <div class="q-answer" v-html="answer.a"></div>

        </div>

        <div v-if="answer?.c" class="px-2">Correct.</div>
        <div v-else-if="isAnswered" class="px-2">Incorrect.</div>
        <div class="q-explain" v-if="answer?.e" v-html="answer.e"></div>
      </div>
      <div class="flex" v-if="!isAnswered">
        <div class="flex-grow text-end my-auto me-4">
          <p class="">{{ optionsToSelect }}</p>
        </div>
        <div class=" flex-shrink text-end mx-4">
          <Button label="Submit" icon="pi pi-check" raised rounded class="font-bold px-24 py-4 my-auto whitespace-nowrap" :disabled="!isQuestionReady" @click="submitQuestion()" />
        </div>
      </div>
    </div>
    <div></div>
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect, computed, watch } from "vue";
import type { Question } from "@/constants";
import { QUESTION_HANDLER_URL, URL_PARAM_QID, URL_PARAM_TOPIC } from "@/constants";
import { Sha256 } from '@aws-crypto/sha256-js';
import { toHex } from "uint8array-tools";
import Button from 'primevue/button';

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

const isQuestionReady = computed(() => {
  // console.log("isQuestionReady", learnerAnswerRadio.value, learnerAnswersCheck.value, questionMarkdown.value?.correct);
  // must be either a single radio answer or multiple checkbox answers matching the number of correct answers
  return learnerAnswerRadio.value !== undefined && questionMarkdown.value?.correct == 1 || learnerAnswersCheck.value.length == questionMarkdown.value?.correct;
});

// this fn abstracts the complicated logic of informing the user
// how many answers they need to select
const optionsToSelect = computed(() => {
  // this line is needed for type checking
  if (!questionMarkdown.value) return "";

  // is this a single choice question with radio buttons?
  if (questionMarkdown.value.correct == 1) {
    // if a radio button is selected it can be submitted
    // the user can change the answer, but not deselect it
    if (learnerAnswerRadio.value == undefined) {
      return "Select one of the options";
    } else {
      return "Check your selection and submit";
    }
  }

  // assume it is a multichoice question from now on

  // perform an exhaustive match by how many answers were selected
  const remainingNumber = questionMarkdown.value.correct - learnerAnswersCheck.value.length;
  if (remainingNumber == 0) {
    // if the required number of answers is selected, the user can submit
    return "Check your selection and submit";
  }
  else if (remainingNumber < 0) {
    // too many answers selected
    return `Only ${questionMarkdown.value.correct} options should be selected`;
  }
  else {
    // more answers should be selected
    const wordMore = learnerAnswersCheck.value.length ? "more" : "";
    const wordAnswers = remainingNumber > 1 ? "options" : "option";
    return `Select ${remainingNumber} ${wordMore} ${wordAnswers}`;
  }
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
  } catch (error) {
    console.error("Failed to get question.");
    console.error(error);
  }
});

async function submitQuestion() {
  // double-check there are answers to submit
  if (!isQuestionReady.value) {
    console.error("Must select answers:", questionMarkdown.value?.correct);
    return;
  }

  // the lambda expects a string array of answers
  const answers = JSON.stringify(questionMarkdown.value?.correct == 1 ? [learnerAnswerRadio.value] : learnerAnswersCheck.value);

  console.log("Answers for POST", answers);

  // calculate the hash of the request body for x-amz-content-sha256 header
  // as required by CloudFront
  const hash = new Sha256();
  hash.update(answers);
  const bodyHash = toHex(await hash.digest());

  const response = await fetch(`${QUESTION_HANDLER_URL}${URL_PARAM_TOPIC}=${questionMarkdown.value?.topic}&${URL_PARAM_QID}=${questionMarkdown.value?.qid}`, {
    method: "POST",
    body: answers,
    headers: {
      "x-amz-content-sha256": bodyHash,
    },
  });

  // a successful response should contain the saved question
  // an error may contain JSON or plain text, depending on where the errror occurred
  if (response.status === 200) {
    try {
      // update the question with the full details
      questionMarkdown.value = <Question>await response.json();
      console.log("Full question received", questionMarkdown.value);
    } catch (error) {
      console.error(error);
    }
  } else {
    console.error("Failed to save the question: ", response.status);
  }
}

</script>
