<template>
  <TransitionSlot>
    <div class="flex" v-if="questionMarkdown">
      <div class="q-card">
        <div class="q-text">
          <div class="" v-html="questionMarkdown?.question"></div>
        </div>

        <div v-for="(answer, index) in questionMarkdown?.answers" :key="index">
          <h3 v-if="isAnswered && index === 0 && questionMarkdown?.correct == 1" class="mb-4">Your answer</h3>
          <h3 v-if="isAnswered && index === 0 && questionMarkdown?.correct > 1" class="mb-4">Your answers</h3>
          <h3 v-else-if="!isAnswered && index === 0" class="mb-4">Answers</h3>
          <h3 v-else-if="isAnswered && index === questionMarkdown?.correct" class="mb-4">Other options</h3>

          <div class="mb-8 border-2 rounded-md" :class="{ 'border-4': isAnswered, 'border-green-100': answer?.c, 'border-red-100': !answer?.c && isAnswered, 'border-slate-100': !isAnswered, 'hide-explanation': isAnswered && !answer.c && !answer.sel }">
            <div class="flex items-center" :class="{ 'border-b-2': isAnswered, 'border-green-100': answer?.c, 'border-red-100': !answer?.c && isAnswered }">
              <input type="radio" v-if="questionMarkdown?.correct == 1" :name="questionMarkdown?.qid" :value="index" :disabled="isAnswered" v-model="learnerAnswerRadio" />
              <input type="checkbox" v-if="questionMarkdown?.correct && questionMarkdown.correct > 1" :name="questionMarkdown?.qid" :disabled="isAnswered" :value="index" v-model="learnerAnswersCheck" />
              <div class="q-answer" v-html="answer.a"></div>
            </div>
            <div v-if="answer?.c" class="px-2 my-2">Correct.</div>
            <div v-else-if="isAnswered" class="px-2 my-2">
              <Tag value="Incorrect" severity="secondary" class="me-2 incorrect-tag" />
              <Tag value="Explain" icon="pi pi-sort-down-fill" severity="secondary" class="explain-link" @click="showExplanation" />
              <span class="incorrect-label">Incorrect.</span>
            </div>
            <div class="q-explain" v-if="answer?.e" v-html="answer.e"></div>
          </div>
        </div>

        <div class="flex">
          <div v-if="hasToken" class="flex-shrink text-start">
            <Button label="Edit" icon="pi pi-pencil" severity="secondary" rounded class="whitespace-nowrap" @click="navigateToEditPage" />
          </div>
          <div v-if="!isAnswered" class="flex-grow text-end my-auto me-4">
            <p class="text-xs text-slate-500">{{ optionsToSelect }}</p>
          </div>
          <div v-if="!isAnswered" class="flex-shrink text-end">
            <Button label="Submit" icon="pi pi-check" raised rounded class="font-bold px-24 py-4 my-auto whitespace-nowrap" :disabled="!isQuestionReady" @click="submitQuestion()" />
          </div>
        </div>
      </div>
    </div>
  </TransitionSlot>
  <div v-if="!questionMarkdown">
    <p>Loading...</p>
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect, computed, watch } from "vue";
import router from "@/router";
import type { Question } from "@/constants";
import { QUESTION_HANDLER_URL, URL_PARAM_QID, URL_PARAM_TOPIC, TOKEN_HEADER_NAME, URL_PARAM_LIST_SEPARATOR, URL_PARAM_ANSWERS } from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import Button from 'primevue/button';
import Tag from "primevue/tag";
import TransitionSlot from "./TransitionSlot.vue";

const props = defineProps<{
  topic: string,
  qid?: string,
}>()

const store = useMainStore();
const { token } = storeToRefs(store);

// as fetched from the server
const questionMarkdown = ref<Question | undefined>();
const learnerAnswersCheck = ref<string[]>([]);
const learnerAnswerRadio = ref<string | undefined>();

// a temporary solution to enable editing links
const hasToken = computed(() => {
  return localStorage.getItem(TOKEN_HEADER_NAME) ? true : false;
});

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

/// Changes the parent class of the answer to toggle the explanation
/// Explanations for incorrect answers NOT selected by the user are hidden by default
function showExplanation(evt: Event | undefined) {
  const src = <HTMLElement>evt?.currentTarget;
  console.log("showExplanation", src);
  if (src) {
    src.parentElement?.parentElement?.classList.remove("hide-explanation");
  }
}

async function submitQuestion() {
  // double-check there are answers to submit
  if (!isQuestionReady.value) {
    console.error("Must select answers:", questionMarkdown.value?.correct);
    return;
  }

  // the lambda expects a list of answers in the URL
  const answers = questionMarkdown.value?.correct == 1 ? learnerAnswerRadio.value : learnerAnswersCheck.value.join(URL_PARAM_LIST_SEPARATOR);

  // calculate the hash of the request body for x-amz-content-sha256 header
  // as required by CloudFront
  // const hash = new Sha256();
  // hash.update(answers);
  // const bodyHash = toHex(await hash.digest());

  const url = `${QUESTION_HANDLER_URL}${URL_PARAM_TOPIC}=${questionMarkdown.value?.topic}&${URL_PARAM_QID}=${questionMarkdown.value?.qid}&${URL_PARAM_ANSWERS}=${answers}`;
  // add a token with the email, if there is one (logged in users)
  const headers = new Headers();
  if (token.value) headers.append(TOKEN_HEADER_NAME, token.value);

  try {
    const response = await fetch(url,
      {
        headers: headers,
      });

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      try {
        // update the question with the full details
        const question = <Question>await response.json();
        questionMarkdown.value = question;
        console.log("Full question received", questionMarkdown.value);

        // reset the user selection because the answers got rearranged with the correct ones at the top
        learnerAnswersCheck.value = [];
        question.answers.forEach((answer, index) => {
          if (answer.sel) {
            if (question.correct == 1) {
              learnerAnswerRadio.value = index.toString();
              console.log("learnerAnswerRadio", learnerAnswerRadio.value);
            } else {
              learnerAnswersCheck.value.push(index.toString());
              console.log("learnerAnswersCheck", learnerAnswersCheck.value);
            }
          }
        });

      } catch (error) {
        console.error(error);
      }
    } else {
      console.error("Failed to submit answers: ", response.status);
    }
  } catch (error) {
    console.error("Failed to submit answers.");
    console.error(error);
  }
}

/// navigates to edit page, but it should only work if the user has a token
function navigateToEditPage() {
  router.push(`/add?${URL_PARAM_TOPIC}=${questionMarkdown.value?.topic}&${URL_PARAM_QID}=${questionMarkdown.value?.qid}`);
}

watchEffect(async () => {
  console.log(`Fetching question for: ${props.topic}/${props.qid}`);
  // only fetch if topic is set
  if (!props.topic) return;

  // make sure nothing is showing if the component is reused
  questionMarkdown.value = undefined;

  // add a token with the email, if there is one (logged in users)
  const headers = new Headers();
  if (token.value) headers.append(TOKEN_HEADER_NAME, token.value);

  try {

    // fetching by topic returns a random question
    // fetching with qid returns a specific question
    const fetchParams = `${URL_PARAM_TOPIC}=${props.topic}`.concat(props.qid ? `&${URL_PARAM_QID}=${props.qid}` : "");
    console.log("fetchParams", fetchParams);

    const response = await fetch(`${QUESTION_HANDLER_URL}${fetchParams}`,
      {
        headers: headers,
      });
    console.log(`Fetched. Status: ${response.status}`);

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

</script>
