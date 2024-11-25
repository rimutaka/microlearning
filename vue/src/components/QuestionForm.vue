<template>
  <div v-if="hydrated" class="card mt-12">
    <div class="flex flex-wrap gap-4 mb-8">
      <h4>Topics: </h4>
      <div class="flex" v-for="topic in topics" :key="topic.id">
        <RadioButton v-model="selectedTopic" name="topics" :value="topic.id" />
        <label :for="topic.id" class="ms-2 me-4">{{ topic.t }}</label>
      </div>
    </div>
    <div class="mb-4">
      <div class="flex flex-wrap gap-4 mb-4">
        <h4 class="mt-auto">Question </h4>
        <div class="flex-grow text-end">
          <Button :label="previewWindow ? 'Referesh preview' : 'Open preview'" icon="pi pi-receipt" severity="secondary" class="whitespace-nowrap" iconPos="right" @click="showPreviewWindow()" />
        </div>
      </div>
      <div class="w-full">
        <Textarea v-model="questionText" id="questionTextInput" class="w-full" rows="3" @keydown="formattingKeypress" @focusin="showMdPreview" />
      </div>
    </div>
    <div class="flex flex-wrap gap-4 mb-8">
      <h4>Answers</h4>
      <div class="w-full mb-6" v-for="(answer, idx) in answers" :key="idx">
        <Textarea v-model="answer.a" :value="answer.a" rows="3" :id="`answerInput${idx}`" class="w-full mb-2" placeholder="An answer options (always visible)" @keydown="formattingKeypress" @focusin="showMdPreview" />
        <Textarea v-model="answer.e" :value="answer.e" rows="5" :id="`explanationInput${idx}`" class="w-full mb-2" placeholder="A detailed explanation (visible after answering)" @keydown="formattingKeypress" @focusin="showMdPreview" />
        <div class="flex">
          <div class="flex-grow justify-start text-start ps-4">
            <input type="radio" v-model="answer.c" :name="`c${idx}`" :value="true" class="h-8 w-8 dark:bg-neutral-700 checked:bg-green-600 p-3" />
            <label class="ms-2" :for="`c${idx}`">Correct</label>
            <input type="radio" v-model="answer.c" :name="`c${idx}`" :value="false" :checked="!answer.c" class="h-8 w-8 dark:bg-neutral-700 checked:bg-red-600 p-3 ms-6" />
            <label class="ms-2" :for="`c${idx}`">Incorrect</label>
          </div>
          <div class="flex-shrink">
            <Button label="Add another answer" icon="pi pi-plus" severity="secondary" class="ms-4 whitespace-nowrap" @click="addAnswer(idx)" />
            <Button v-if="answers.length > 1" label="Delete this answer" icon="pi pi-trash" severity="secondary" class="ms-4 whitespace-nowrap" @click="deleteAnswer(idx)" />
          </div>
        </div>
      </div>
    </div>
    <div>
      <h4 class="text-start mb-4">Contributor</h4>
      <QuestionContributorForm class="mb-12" />
    </div>
    <div class="flex gap-12 mt-8">
      <div class="text-left flex-grow">
        <h4 class="mb-4">Question readiness</h4>
        <ul class="question-readiness">
          <li :class="{ 'question-ready': questionReadiness.topic, 'question-not-ready': !questionReadiness.topic }"><i></i>Topic selected</li>
          <li :class="{ 'question-ready': questionReadiness.question, 'question-not-ready': !questionReadiness.question }"><i></i>Question text entered</li>
          <li :class="{ 'question-ready': questionReadiness.answers, 'question-not-ready': !questionReadiness.answers }"><i></i>At least 2 answers</li>
          <li :class="{ 'question-ready': questionReadiness.correct, 'question-not-ready': !questionReadiness.correct }"><i></i>At least 1 correct answer</li>
          <li :class="{ 'question-ready': questionReadiness.explanations, 'question-not-ready': !questionReadiness.explanations }"><i></i>Detailed explanations for all answers</li>
          <li class="question-ready"><i></i><a href="https://creativecommons.org/licenses/by-sa/4.0/" target="_blank">CC-BY-SA 4.0</a> license</li>
        </ul>
      </div>
      <div class="flex-shrink text-end">
        <Button label="Cancel" icon="pi pi-times" raised severity="secondary" class="me-4 whitespace-nowrap" @click="router.back()" />
        <Button label="Save" icon="pi pi-check" raised class="my-auto whitespace-nowrap" :disabled="!questionReady" @click="submitQuestion()" />
      </div>
    </div>
  </div>
  <LoadingMessage v-else />
  <!--Popover v-if="!previewWindow" ref="mdPreviewPopover" class="max-w-screen-md w-screen">
    <QuestionFieldMarkdown :text="mdTextForPreview" :correct="mdCorrectForPreview" />
  </Popover -->
</template>


<script setup lang="ts">
import { ref, watch, watchEffect } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { useRouter } from "vue-router";
import { Sha256 } from '@aws-crypto/sha256-js';
import { toHex } from "uint8array-tools";
import debounce from "lodash.debounce"
import _ from "lodash";

import { TOPICS, QUESTION_HANDLER_URL, URL_PARAM_TOPIC, URL_PARAM_QID, TOKEN_HEADER_NAME, PREVIEW_QUESTION_LS_KEY, CONTRIBUTOR_DETAILS_LS_KEY } from "@/constants";
import type { Answer, Question } from "@/constants";

import Button from 'primevue/button';
import RadioButton from 'primevue/radiobutton';
import Textarea from 'primevue/textarea';
// import Popover from "primevue/popover";
// import QuestionFieldMarkdown from "./QuestionFieldMarkdown.vue";
import LoadingMessage from "./LoadingMessage.vue";
import QuestionContributorForm from "./QuestionContributorForm.vue";

const props = defineProps<{
  topic: string | undefined,
  qid?: string | undefined,
}>()

const router = useRouter();
const store = useMainStore();
const { token, question } = storeToRefs(store);

const hydrated = ref(false); // toggles the form between loading and the full form
const topics = ref(TOPICS);
const selectedTopic = ref(""); // the topic of the question from TOPICS

const questionText = ref(""); // the text of the question in markdown
const questionTextDebounced = ref(""); // for HTML conversion

const answers = ref<Array<Answer>>([{ a: "", e: "", c: false, sel: false }]); // the list of answers
const answersDebounced = ref<Array<Answer>>([{ a: "", e: "", c: false, sel: false }]); // for HTML conversion

const mdPreviewPopover = ref();
const mdTextForPreview = ref(""); // debounced markdown text from the input in focus to be displayed in the popover
const mdCorrectForPreview = ref<boolean | undefined>(undefined); // status of answer.c (correct/incorrect) from the answer in focus for the popover

// a reference to the preview window that can be opened on demand
const previewWindow = ref<Window | null>(null);

let inFocusInputId = ""; // the ID of the input field that is currently in focus to enable MD rendering

/// used to inform the user what steps are required
/// affects questionReady
/// updated via watch
const questionReadiness = ref({
  topic: false,
  question: false,
  answers: false,
  correct: false,
  explanations: false,
});
const questionReady = ref(false); // enables Submit button

/// Turns on a Popover with a Markdown preview
const showMdPreview = (event: FocusEvent) => {
  const src = event.target as HTMLInputElement; // this cast is safe because only textarea elements calls this function
  mdTextForPreview.value = src.value; // get the text from the input itself for the the initial state
  inFocusInputId = src.id; // this ID will be used to update the preview when the text changes

  // console.log(`showMdPreview id: ${inFocusInputId}`);

  // find the right v-model to get `answer.c` status from
  // only explanations need this value, others have it set to undefined
  if (inFocusInputId.startsWith("explanationInput")) {
    const index = parseInt(inFocusInputId.replace("explanationInput", ""));
    mdCorrectForPreview.value = answers.value[index].c === true;
  }
  else {
    mdCorrectForPreview.value = undefined;
  }

  // there is a bug that I cannot work around
  // the popover remains attached to the previous input if you change focus
  // to another field without clicking somewhere else first
  //
  // the popover is disabled if a live preview window is open
  mdPreviewPopover.value?.show(event);
}

/// Adds an answer block to the form
function addAnswer(index: number) {
  answers.value.splice(index + 1, 0, { a: "", e: "", c: false, sel: false });
}

/// Removes an answer block from the form
function deleteAnswer(index: number) {
  if (answers.value.length === 1) {
    // cannot delete the last remaining answer
    return;
  }
  answers.value.splice(index, 1);
}

/// Adds or removes Markdown formatting from the selected text
/// in question, answer and explanation fields
function formattingKeypress(event: KeyboardEvent) {

  // exit early if there is no selected text
  const target = <HTMLInputElement>event.target;
  const start = target.selectionStart;
  const end = target.selectionEnd;

  // check there is a selection
  if (!start || !end || start === end) {
    return;
  }

  // check how the key should be handled
  let formatSymbolStart = "", formatSymbolEnd = "";
  let formatSymbolLength = 0; // remains 0 if the selection should not be toggled
  if (event.key === "b" && event.ctrlKey) { formatSymbolStart = "**"; formatSymbolEnd = "**"; formatSymbolLength = 2; }
  if (event.key === "i" && event.ctrlKey) { formatSymbolStart = "_"; formatSymbolEnd = "_"; formatSymbolLength = 1; }
  if (event.key === "_") { formatSymbolStart = "_"; formatSymbolEnd = "_"; }
  if (event.key === "*") { formatSymbolStart = "*"; formatSymbolEnd = "*"; }
  if (event.key === "`") { formatSymbolStart = "`"; formatSymbolEnd = "`"; }
  if (event.key === "'") { formatSymbolStart = "'"; formatSymbolEnd = "'"; }
  if (event.key === "\"") { formatSymbolStart = "\""; formatSymbolEnd = "\""; }
  if (event.key === "{") { formatSymbolStart = "{"; formatSymbolEnd = "}"; }
  if (event.key === "[") { formatSymbolStart = "["; formatSymbolEnd = "]"; }
  if (event.key === "(") { formatSymbolStart = "("; formatSymbolEnd = ")"; }

  // if no format symbol is found, exit
  if (!formatSymbolStart) return;

  event.preventDefault();

  // a reusable function to check if the formatting should be removed
  // ** are usually outside the selection and __ are inside
  // brackets and quotes are excluded from the check via formatSymbolLength = 0
  const shouldRemoveFormat = (v: string) => formatSymbolLength > 0
    && (v.slice(start - formatSymbolLength, start) === formatSymbolStart && v.slice(end, end + formatSymbolLength) === formatSymbolEnd
      || v.slice(start)?.startsWith(formatSymbolStart) && v.slice(end - formatSymbolLength)?.startsWith(formatSymbolEnd));

  // reusable functions for adding and removing formatting
  const remover = (v: string) => {
    if (v.slice(start)?.startsWith(formatSymbolStart) && v.slice(end - formatSymbolLength)?.startsWith(formatSymbolEnd))
      // the removable part is inside the selection, e.g. __text__
      return v.slice(0, start) + v.slice(start + formatSymbolLength, end - formatSymbolLength) + v.slice(end);
    else
      // the removable part is outside the selection, e.g. **text**
      return v.slice(0, start - formatSymbolLength) + v.slice(start, end) + v.slice(end + formatSymbolLength);
  };

  // a reusable function for adding formatting
  const adder = (v: string) => v.slice(0, start) + formatSymbolStart + v.slice(start, end) + formatSymbolEnd + v.slice(end);

  // find the right v-model to update
  // then check if the formatting already exists and should be added or removed (toggle)
  if (target.id == "questionTextInput") {
    questionText.value = (shouldRemoveFormat(questionText.value)) ? remover(questionText.value) : adder(questionText.value);
    // repeat the same for answers and explanations
  } else if (target.id.startsWith("answerInput")) {
    const index = parseInt(target.id.replace("answerInput", ""));
    answers.value[index].a = (shouldRemoveFormat(answers.value[index].a)) ? remover(answers.value[index].a) : adder(answers.value[index].a);
  } else if (target.id.startsWith("explanationInput")) {
    const index = parseInt(target.id.replace("explanationInput", ""));
    answers.value[index].e = (shouldRemoveFormat(answers.value[index].e)) ? remover(answers.value[index].e) : adder(answers.value[index].e);
  }
  else {
    console.error("Unknown formatting target", target);
  }
}

/** Save question in the cloud */
async function submitQuestion() {

  if (!token.value) {
    // unlikely to get here because there is a page guard in the router
    console.log("No token found. Redirecting to homepage.");
    router.push("/");
    return;
  }
  // the lambda gets all it needs from the serialized JSON object
  const submissionQuestion = JSON.stringify(<Question>{
    qid: props.qid,
    topic: selectedTopic.value,
    question: questionText.value,
    answers: answers.value,
    correct: 0,
    contributor: question.value?.contributor, // this struct is set by a sub-component
  });

  // console.log(submissionQuestion);

  // calculate the hash of the request body for x-amz-content-sha256 header
  // as required by CloudFront
  const hash = new Sha256();
  hash.update(submissionQuestion);
  const bodyHash = toHex(await hash.digest());

  const response = await fetch(`${QUESTION_HANDLER_URL}`, {
    method: "PUT",
    body: submissionQuestion,
    headers: {
      "x-amz-content-sha256": bodyHash,
      [TOKEN_HEADER_NAME]: token.value,
    },
  });

  // a successful response should contain the saved question
  // an error may contain JSON or plain text, depending on where the error occurred
  console.log("Response status: ", response.status);
  if (response.status === 200) {
    try {
      const savedQuestion = <Question>await response.json();
      // console.log("Saved OK: ", savedQuestion);
      // redirect to the saved question
      // TODO: make use of the returned question details to avoid an extra fetch
      router.push(`/question?${URL_PARAM_TOPIC}=${savedQuestion.topic}&${URL_PARAM_QID}=${savedQuestion.qid}`);
    } catch (error) {
      console.error(error);
    }
  } else {
    console.error("Failed to save the question: ", response.status);
  }
}

/// Stores the current question data in local storage and opens a new popup window for live preview
/// LS is a reliable way of passing the initial data to the preview window
/// subsequent updates are sent via postMessage
const showPreviewWindow = () => {
  localStorage.setItem(PREVIEW_QUESTION_LS_KEY, JSON.stringify(<Question>{
    topic: selectedTopic.value,
    question: questionText.value,
    answers: answers.value,
    contributor: question.value?.contributor,
    // this is a very truncated version of a question - bare essentials
  }));

  previewWindow.value = window.open(`${window.location.origin}/preview`, PREVIEW_QUESTION_LS_KEY);
}

/// Slows down markdown conversion to HTML
const debounceMarkdownForHtml = debounce(() => {
  let textToDebounce = "";

  // find the right v-model to debounce
  if (inFocusInputId == "questionTextInput") {
    textToDebounce = questionText.value;
  } else if (inFocusInputId.startsWith("answerInput")) {
    const index = parseInt(inFocusInputId.replace("answerInput", ""));
    textToDebounce = answers.value[index]?.a;
  } else if (inFocusInputId.startsWith("explanationInput")) {
    const index = parseInt(inFocusInputId.replace("explanationInput", ""));
    textToDebounce = answers.value[index]?.e;
  }
  else {
    console.error("Unknown debouncing target: ", inFocusInputId);
  }

  mdTextForPreview.value = textToDebounce;

  postQuestionPreview();
}, 500);

/// Slows down messaging the preview window for changes not covered by markdown conversion
const debouncePostMsg = debounce(() => {
  postQuestionPreview();
}, 500);

// update questionReadiness list and enable the submit button via questionReady
watch([selectedTopic, questionText, answers.value, question], ([, , answersNew], [, , answersOld]) => {
  // assess question readiness
  questionReadiness.value.topic = selectedTopic.value !== "";
  questionReadiness.value.question = questionText.value.length > 10;
  questionReadiness.value.answers = answers.value.length >= 2 && answers.value.every((answer) => answer.a.length > 0);
  questionReadiness.value.correct = answers.value.some((answer) => answer.c);
  questionReadiness.value.explanations = answers.value.every((answer) => answer.e.length > 10);

  // enable / disable the submit button
  questionReady.value = Object.values(questionReadiness.value).every((value) => value);

  if (inFocusInputId) {
    debounceMarkdownForHtml();
    // for text changes postQuestionPreview is called in the debounced function 
  }
  else {
    // other changes are sent to the preview immediately
    debouncePostMsg();
  }
}, { deep: true });

/// Resets local and store values to start accepting data for a brand new question
/// from a blank form
function resetValuesForNewQuestion() {
  selectedTopic.value = "";

  questionText.value = "";
  questionTextDebounced.value = "";

  answers.value.length = 0;
  answers.value.push({ a: "", e: "", c: false, sel: false });

  answersDebounced.value.length = 0;
  answersDebounced.value.push({ a: "", e: "", c: false, sel: false });

  mdTextForPreview.value = "";
  mdCorrectForPreview.value = undefined;
  inFocusInputId = "";

  // clear all fields from the question in store
  // this is a hack and there should be a more elegant solution
  store.resetQuestionToBlank();
}

/// Loads all local variables with the question data, if present.
function loadQuestion(fetchedQuestion: Question) {

  // set debounced values before the main values to avoid triggering out of index errors
  // in the template
  answersDebounced.value = JSON.parse(JSON.stringify(fetchedQuestion.answers));
  questionTextDebounced.value = fetchedQuestion.question;

  // copy DDB values to the form models
  selectedTopic.value = fetchedQuestion.topic;
  questionText.value = fetchedQuestion.question;

  // copy the array while maintaining a reference to the original object
  // https://stackoverflow.com/questions/71353509/why-would-a-vue3-watcher-of-a-prop-not-be-triggered-composition-api
  answers.value.length = 0;
  fetchedQuestion.answers.forEach((answer: Answer) => {
    answers.value.push(answer);
  });

  // copy the loaded question to the store
  // for sub-components to access the data
  question.value = JSON.parse(JSON.stringify(fetchedQuestion));; // store the question in the store
}

watchEffect(async () => {
  // if no topic/qid is set, this is a new question
  if (!(props.topic && props.qid)) {
    console.log("Adding a new question");
    resetValuesForNewQuestion();
    hydrated.value = true; // enable the form
    return;
  }

  // disable the form while fetching the question
  hydrated.value = false;

  // fetching an existing question for editing
  console.log(`Fetching question for ${props.topic}/${props.qid}`);
  if (!token.value) {
    console.log("No token found. Redirecting to homepage.");
    router.push("/");
    return;
  }

  try {
    const response = await fetch(`${QUESTION_HANDLER_URL}${URL_PARAM_TOPIC}=${props.topic}&${URL_PARAM_QID}=${props.qid}`, {
      method: "PUT",
      headers: {
        "x-amz-content-sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", // empty body hash
        [TOKEN_HEADER_NAME]: token.value,
      },
    });


    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      try {
        console.log(`Fetched. Status: ${response.status}`);
        const fetchedQuestion = <Question>await response.json();
        // console.log(question);

        loadQuestion(fetchedQuestion);

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

  hydrated.value = true; // enable the form

});

/// Packages the current question data and sends it to the preview window, if open
function postQuestionPreview() {
  // do nothing if the window is not open, but this will not help if the window was closed
  // because the reference will still be there
  if (!previewWindow.value) return;

  // delete the window ref if the window was closed and exit
  if (previewWindow.value.closed) {
    console.log("Preview window was closed");
    previewWindow.value = null;
    return;
  };

  const msg = <Question>{
    qid: props.qid,
    topic: selectedTopic.value,
    question: questionText.value,
    answers: answers.value,
    correct: 0, // setting this to the correct value will enable checkboxes in the preview
    contributor: question.value?.contributor,
  };

  console.log("Sending preview update msg");
  previewWindow.value?.postMessage(JSON.stringify(msg));
}

</script>
