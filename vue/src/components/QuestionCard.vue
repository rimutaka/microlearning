<template>
  <TransitionSlot>
    <div v-if="question && loadingStatus == constants.LoadingStatus.Loaded" class="flex">
      <div class="q-card">
        <div class="q-text">
          <div class="" v-html="question?.question"></div>
        </div>

        <div v-for="(answer, index) in question?.answers" :key="index">
          <h3 v-if="isAnswered && index === 0 && question?.correct == 1" class="mb-4">Your answer</h3>
          <h3 v-if="isAnswered && index === 0 && question?.correct > 1" class="mb-4">Your answers</h3>
          <h3 v-else-if="!isAnswered && index === 0" class="mb-4">Answers</h3>
          <h3 v-else-if="isAnswered && index === question?.correct" class="mb-4">Other options</h3>

          <div class="mb-8 border-2 rounded-md" :class="{ 'border-4': isAnswered, 'border-green-100 dark:border-green-700': answer?.c, 'border-red-100 dark:border-red-700': !answer?.c && isAnswered, 'border-slate-100': !isAnswered, 'hide-explanation': isAnswered && !answer.c && !answer.sel }">
            <div class="flex items-center" :class="{ 'border-b-2': isAnswered, 'border-green-100 dark:border-green-700': answer?.c, 'border-red-100 dark:border-red-700': !answer?.c && isAnswered }">
              <input type="radio" v-if="question?.correct == 1" :name="question?.qid" :value="index" :disabled="isAnswered" v-model="answerRadio" />
              <input type="checkbox" v-if="question?.correct && question.correct > 1" :name="question?.qid" :disabled="isAnswered" :value="index" v-model="answersCheckbox" />
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

        <div v-if="!props.useStore" class="flex">
          <div class="flex-grow text-start">
            <Button v-if="hasToken" label="Edit" size="small" icon="pi pi-pencil" severity="secondary" class="whitespace-nowrap me-2" @click="navigateToEditPage" />
            <LinkButton :href="questionTopicAndPageUrl" label="Copy link" class="me-2 mb-2" icon="pi pi-share-alt" @click="copyLinkToClipboard" />
            <LinkButton v-if="!isAnswered && next" :href="questionTopicOnlyUrl" label="Skip" class="me-2 mb-2" icon="pi pi-angle-double-right" @click="getNextQuestion" />
            <LinkButton v-if="isAnswered && next" :href="questionTopicOnlyUrl" label="Try one more question" class="mb-2" icon="pi pi-sparkles" @click="getNextQuestion" />
            <p v-if="linkCopiedFlag" class="text-xs text-slate-500">Link copied to the clipboard</p>
            <p v-if="!linkCopiedFlag">&nbsp;</p>
          </div>
          <div class="flex-shrink text-end">
            <Button v-if="!isAnswered" label="Submit" :icon="isQuestionReady ? 'pi pi-check' : 'pi pi-ellipsis-h'" raised class="font-bold px-24 py-4 my-auto whitespace-nowrap" :disabled="!isQuestionReady" @click="submitQuestion()" />
            <p v-if="!isAnswered" class="text-xs text-slate-500 dark:text-slate-300">{{ howManyOptionsLeftToSelect }}</p>
          </div>
        </div>
      </div>
    </div>
    <h3 v-else-if="loadingStatus == constants.LoadingStatus.Loading" class="mt-8 mb-8 text-slate-500 dark:text-slate-100">Loading...</h3>
    <h3 v-else class="mt-8 mb-8 text-slate-500 dark:text-slate-100">Sorry, something went wrong. Try again.</h3>
  </TransitionSlot>
</template>

<script setup lang="ts">
import { ref, watchEffect, computed, watch } from "vue";
import router, { PageIDs } from "@/router";
import type { Question, LoadingStatus } from "@/constants";
import * as constants from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import Button from 'primevue/button';
import Tag from "primevue/tag";
import TransitionSlot from "./TransitionSlot.vue";
import LinkButton from "./LinkButton.vue";

const props = defineProps<{
  topic: string,// must have a value or "any" for any topic
  qid?: string, // returns a random question if omitted
  next?: boolean, // true if the question can show the next button
  useStore?: boolean, // true if the question should be taken from the store and not fetched
}>()

const store = useMainStore();
const { token, currentTopic, question } = storeToRefs(store);

// as fetched from the server
const answersCheckbox = ref<string[]>([]);
const answerRadio = ref<string | undefined>();
const loadingStatus = ref<LoadingStatus>(constants.LoadingStatus.Loading);
const linkCopiedFlag = ref(false); // controls share button: f: Copy link, t: Link copied

// a temporary solution to enable editing links
const hasToken = computed(() => {
  return localStorage.getItem(constants.TOKEN_HEADER_NAME) ? true : false;
});

const isAnswered = computed(() => {
  if (question.value?.answers?.[0].e) { return true } else { return false };
});

const isQuestionReady = computed(() => {
  // console.log("isQuestionReady", answerRadio.value, answersCheckbox.value, question.value?.correct);
  // must be either a single radio answer or multiple checkbox answers matching the number of correct answers
  return answerRadio.value !== undefined && question.value?.correct == 1 || answersCheckbox.value.length == question.value?.correct;
});

// this fn abstracts the complicated logic of informing the user
// how many answers they need to select
const howManyOptionsLeftToSelect = computed(() => {
  // this line is needed for type checking
  if (!question.value) return "";

  // is this a single choice question with radio buttons?
  if (question.value.correct == 1) {
    // if a radio button is selected it can be submitted
    // the user can change the answer, but not deselect it
    if (answerRadio.value == undefined) {
      return "Select one of the options";
    } else {
      return "Check your selection and submit";
    }
  }

  // assume it is a multi-choice question from now on

  // perform an exhaustive match by how many answers were selected
  const remainingNumber = question.value.correct - answersCheckbox.value.length;
  if (remainingNumber == 0) {
    // if the required number of answers is selected, the user can submit
    return "Check your selection and submit";
  }
  else if (remainingNumber < 0) {
    // too many answers selected
    return `Only ${question.value.correct} options should be selected`;
  }
  else {
    // more answers should be selected
    const wordMore = answersCheckbox.value.length ? "more" : "";
    const wordAnswers = remainingNumber > 1 ? "options" : "option";
    return `Select ${remainingNumber} ${wordMore} ${wordAnswers}`;
  }
});

/// A URL to the page with the question on its own
/// without the question ID to display a random question
const questionTopicOnlyUrl = computed(() => `${document.location.origin}/${PageIDs.QUESTION}?${constants.URL_PARAM_TOPIC}=${question.value?.topic}`);

/// A URL to the page with the question on its own
/// for sharing or opening separately
const questionTopicAndPageUrl = computed(() => `${questionTopicOnlyUrl.value}&${constants.URL_PARAM_QID}=${question.value?.qid}`);

// change the status as the question is rendered and loaded from the store in preview mode
watch(question, (newQuestion) => {
  // console.log("newQuestion: ", newQuestion);
  if (props.useStore && newQuestion) {
    loadingStatus.value = constants.LoadingStatus.Loaded;
  }
});

/// Copies the direct link to the question to the clipboard
/// and changes the button message flag to display link copied msg
const copyLinkToClipboard = (e: MouseEvent) => {
  e.preventDefault();
  navigator.clipboard.writeText(questionTopicAndPageUrl.value);
  linkCopiedFlag.value = true;
  setTimeout(() => {
    linkCopiedFlag.value = false;
  }, 3000);
}

/// Copies the direct link to the question to the clipboard
/// and changes the button message flag to display link copied msg
const getNextQuestion = (e: MouseEvent) => {
  e.preventDefault();
  console.log("getNextQuestion");
  loadingStatus.value = constants.LoadingStatus.Loading;
  question.value = undefined;
}


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
    console.error("Must select answers:", question.value?.correct);
    return;
  }

  // the lambda expects a list of answers in the URL
  const answers = question.value?.correct == 1 ? answerRadio.value : answersCheckbox.value.join(constants.URL_PARAM_LIST_SEPARATOR);

  // calculate the hash of the request body for x-amz-content-sha256 header
  // as required by CloudFront
  // const hash = new Sha256();
  // hash.update(answers);
  // const bodyHash = toHex(await hash.digest());

  const url = `${constants.QUESTION_HANDLER_URL}${constants.URL_PARAM_TOPIC}=${question.value?.topic}&${constants.URL_PARAM_QID}=${question.value?.qid}&${constants.URL_PARAM_ANSWERS}=${answers}`;
  // add a token with the email, if there is one (logged in users)
  const headers = new Headers();
  if (token.value) headers.append(constants.TOKEN_HEADER_NAME, token.value);

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
        question.value = <Question>await response.json();
        console.log("Full question received", question.value);

        // reset the user selection because the answers got rearranged with the correct ones at the top
        answersCheckbox.value = [];
        question.value.answers.forEach((answer, index) => {
          if (answer.sel) {
            if (question.value?.correct == 1) {
              answerRadio.value = index.toString();
              console.log("answerRadio", answerRadio.value);
            } else {
              answersCheckbox.value.push(index.toString());
              console.log("answersCheckbox", answersCheckbox.value);
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
  router.push(`/add?${constants.URL_PARAM_TOPIC}=${question.value?.topic}&${constants.URL_PARAM_QID}=${question.value?.qid}`);
}

// Store the qid in a comma-separated list in the local storage.
// Remove old entries if the list gets longer than 10 items.
function storeRecentQuestionsInLS(qid: string) {
  const recent = localStorage.getItem(constants.RECENT_HEADER_NAME);
  if (recent) {
    const recentList = recent.split(",");
    if (recentList.includes(qid)) {
      // remove the old entry
      recentList.splice(recentList.indexOf(qid), 1);
    }
    recentList.unshift(qid);
    if (recentList.length > 10) {
      console.log("Removing old entries from recent questions list");
      recentList.pop();
    }
    localStorage.setItem(constants.RECENT_HEADER_NAME, recentList.join(","));
  } else {
    localStorage.setItem(constants.RECENT_HEADER_NAME, qid);
  }
}

/// The topic always comes from props.topic
/// The qid comes from props.qid if random is false.
const loadQuestion = async (random?: boolean) => {
  // preview questions are loaded from the store and should never be fetched from the server
  if (props.useStore) {
    console.log("Using store question");
    loadingStatus.value = question.value ? constants.LoadingStatus.Loaded : constants.LoadingStatus.NoData;
    return;
  }

  console.log(`Fetching question for: ${props.topic} / ${props.qid} / random: ${random}`);

  // make sure nothing is showing if the component is reused
  loadingStatus.value = constants.LoadingStatus.Loading;
  // question.value = undefined;
  currentTopic.value = undefined;

  // add a token with the email, if there is one (logged in users)
  const headers = new Headers();
  if (token.value) headers.append(constants.TOKEN_HEADER_NAME, token.value);

  // add list of recently viewed questions to the request
  const recent = localStorage.getItem(constants.RECENT_HEADER_NAME);
  if (recent) headers.append(constants.RECENT_HEADER_NAME, recent);

  try {
    // fetching by topic returns a random question
    // fetching with qid returns a specific question
    // ignore qid if random is true
    const fetchParams = `${constants.URL_PARAM_TOPIC}=${props.topic}`.concat(props.qid && !random ? `&${constants.URL_PARAM_QID}=${props.qid}` : "");
    console.log("fetchParams", fetchParams);

    const response = await fetch(`${constants.QUESTION_HANDLER_URL}${fetchParams}`,
      {
        headers: headers,
        signal: AbortSignal.timeout(5000),
      });
    console.log(`Fetched. Status: ${response.status}`);

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      try {
        question.value = <Question>await response.json();
        // console.log(question);
        // console.log(question.topic);
        console.log(`Loaded ${question.value.topic} / ${question.value.qid}`);

        currentTopic.value = question.value.topic;
        loadingStatus.value = constants.LoadingStatus.Loaded;
        storeRecentQuestionsInLS(question.value.qid); // add qid to the list of recent questions

      } catch (error) {
        console.error(error);
      }
    }
    else if (response.status === 404) {
      loadingStatus.value = constants.LoadingStatus.NoData;
    }
    else {
      loadingStatus.value = constants.LoadingStatus.Error;
      console.error("Failed to get question. Status: ", response.status);
    }
  } catch (error) {
    loadingStatus.value = constants.LoadingStatus.Error;
    console.error("Failed to get question.");
    console.error(error);
  }
};

console.log("About to load question");
loadQuestion();

</script>
