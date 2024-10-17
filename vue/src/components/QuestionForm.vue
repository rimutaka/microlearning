<template>
  <div class="card mt-12">
    <h3 class="mb-8">New question form</h3>
    <div class="flex flex-wrap gap-4 mb-4">
      <h4>Topics: </h4>
      <div class="flex" v-for="topic in topics" :key="topic.id">
        <RadioButton v-model="selectedTopic" name="topics" :value="topic.id" />
        <label :for="topic.id" class="ms-2 me-4">{{ topic.t }}</label>
      </div>
    </div>
    <div class="flex flex-wrap gap-4 mb-4">
      <h4>Question: </h4>
      <div class="w-full md-group" @focusin="inFocusInputId = 'questionTextInput'">
        <Textarea v-model="questionText" id="questionTextInput" class="w-full" rows="3" @keydown="formattingKeypress" />
        <QuestionFieldMarkdown :text="questionTextDebounced" :correct="undefined" :active="inFocusInputId == 'questionTextInput'"/>
      </div>
    </div>
    <div class="flex flex-wrap gap-4 mb-8">
      <h4>Answers: </h4>
      <div class="w-full mb-6" v-for="(answer, idx) in answers" :key="idx">
        <div class="md-group mb-2" @focusin="inFocusInputId = `answerInput${idx}`">
          <Textarea v-model="answer.a" :value="answer.a" rows="3" :id="`answerInput${idx}`" class="w-full" placeholder="An answer options (always visible)" @keydown="formattingKeypress" />
          <QuestionFieldMarkdown :text="answersDebounced[idx].a" :correct="undefined" :active="inFocusInputId == `answerInput${idx}`"/>
        </div>

        <div class="md-group mb-2" @focusin="inFocusInputId = `explanationInput${idx}`">
          <Textarea v-model="answer.e" :value="answer.e" rows="5" :id="`explanationInput${idx}`" class="w-full" placeholder="A detailed explanation (visible after answering)" @keydown="formattingKeypress" />
          <QuestionFieldMarkdown :text="answersDebounced[idx].e" :correct="answer.c === true" :active="inFocusInputId == `explanationInput${idx}`"/>
        </div>

        <div class="flex">
          <div class="flex-grow justify-start text-start ps-4">
            <input type="radio" v-model="answer.c" :name="`c${idx}`" :value="true" class="h-8 w-8 checked:bg-green-600 text-green-500 p-3" />
            <label class="ms-2" :for="`c${idx}`">Correct</label>
            <input type="radio" v-model="answer.c" :name="`c${idx}`" :value="false" :checked="!answer.c" class="h-8 w-8 checked:bg-red-600 text-red-500 p-3 ms-6" />
            <label class="ms-2" :for="`c${idx}`">Incorrect</label>
          </div>
          <div class="flex-shrink">
            <Button label="Add another answer" icon="pi pi-plus" severity="secondary" rounded class="ms-4 whitespace-nowrap" @click="addAnswer(idx)" />
            <Button v-if="answers.length > 1" label="Delete this answer" icon="pi pi-trash" severity="secondary" rounded class="ms-4 whitespace-nowrap" @click="deleteAnswer(idx)" />
          </div>
        </div>
      </div>
    </div>
    <div class="flex gap-12">
      <div class="flex-grow text-end">
        <Button label="Submit" icon="pi pi-check" raised rounded class="font-bold px-24 py-4 my-auto whitespace-nowrap" :disabled="!questionReady" @click="submitQuestion()" />
      </div>
      <div class="text-left flex-shrink">
        <h4 class="mb-4">Question readiness:</h4>
        <ul class="question-readiness">
          <li :class="{ 'question-ready': questionReadiness.topic, 'question-not-ready': !questionReadiness.topic }"><i></i>Topic selected</li>
          <li :class="{ 'question-ready': questionReadiness.question, 'question-not-ready': !questionReadiness.question }"><i></i>Question text entered</li>
          <li :class="{ 'question-ready': questionReadiness.answers, 'question-not-ready': !questionReadiness.answers }"><i></i>At least 2 answers</li>
          <li :class="{ 'question-ready': questionReadiness.correct, 'question-not-ready': !questionReadiness.correct }"><i></i>At least 1 correct answer</li>
          <li :class="{ 'question-ready': questionReadiness.explanations, 'question-not-ready': !questionReadiness.explanations }"><i></i>Detailed explanations for all answers</li>
        </ul>
      </div>

    </div>
  </div>
</template>


<script setup lang="ts">
import { ref, watch, computed, watchEffect } from "vue";

import Button from 'primevue/button';
import RadioButton from 'primevue/radiobutton';
import Textarea from 'primevue/textarea';
import QuestionFieldMarkdown from "./QuestionFieldMarkdown.vue";

import { TOPICS, QUESTION_HANDLER_URL, URL_PARAM_TOPIC, URL_PARAM_QID, TOKEN_HEADER_NAME } from "@/constants";

import type { Answer, Question } from "@/constants";
import { useRouter } from "vue-router";
import { Sha256 } from '@aws-crypto/sha256-js';
import { toHex } from "uint8array-tools";
import debounce from "lodash.debounce"

const props = defineProps<{
  topic: string | undefined,
  qid?: string | undefined,
}>()

const router = useRouter();

const topics = ref(TOPICS);
const selectedTopic = ref(""); // the topic of the question from TOPICS

const questionText = ref(""); // the text of the question in markdown
const questionTextDebounced = ref(""); // for HTML conversion

const answers = ref<Array<Answer>>([{ a: "", e: "", c: false }]); // the list of answers
const answersDebounced = ref<Array<Answer>>([{ a: "", e: "", c: false }]); // for HTML conversion

const questionReady = ref(false); // enables Submit button
const inFocusInputId = ref("") // the ID of the input field that is currently in focus to enable MD rendering

/// used to inform the user what steps are required
/// affects questionReady
const questionReadiness = ref({
  topic: false,
  question: false,
  answers: false,
  correct: false,
  explanations: false,
});

/// Adds an answer block to the form
function addAnswer(index: number) {
  answers.value.splice(index + 1, 0, { a: "", e: "", c: false });
  answersDebounced.value.splice(index + 1, 0, { a: "", e: "", c: false });
}

/// Removes an answer block from the form
function deleteAnswer(index: number) {
  if (answers.value.length === 1) {
    // cannot delete the last remaining answer
    return;
  }
  answers.value.splice(index, 1);
  answersDebounced.value.splice(index, 1);
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

async function submitQuestion() {

  // this is a temporary hack to limit who can update DDB
  let token = localStorage.getItem(TOKEN_HEADER_NAME);
  if (!token) {
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
    correct: 0
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
      [TOKEN_HEADER_NAME]: token,
    },
  });

  // a successful response should contain the saved question
  // an error may contain JSON or plain text, depending on where the errror occurred
  if (response.status === 200) {
    try {
      const savedQuestion = <Question>await response.json();
      console.log(savedQuestion);
      console.log(savedQuestion.topic);
      console.log(savedQuestion.qid);

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

/// Slows down markdown conversion to HTML
const debounceMarkdownForHtml = debounce(() => {
  questionTextDebounced.value = questionText.value;
  answersDebounced.value = JSON.parse(JSON.stringify(answers.value))
}, 500)

// update questionReadiness list and enable the submit button via questionReady
watch([selectedTopic, questionText, answers.value], () => {
  // assess question readiness
  questionReadiness.value.topic = selectedTopic.value !== "";
  questionReadiness.value.question = questionText.value.length > 10;
  questionReadiness.value.answers = answers.value.length >= 2 && answers.value.every((answer) => answer.a.length > 0);
  questionReadiness.value.correct = answers.value.some((answer) => answer.c);
  questionReadiness.value.explanations = answers.value.every((answer) => answer.e.length > 10);

  // enable / disable the submit button
  questionReady.value = Object.values(questionReadiness.value).every((value) => value);

  debounceMarkdownForHtml();
});

watchEffect(async () => {

  // this is a temporary hack to limit who can update DDB
  let token = localStorage.getItem(TOKEN_HEADER_NAME);
  if (!token) {
    console.log("No token found. Redirecting to homepage.");
    router.push("/");
    return;
  }

  // if no topic is set, this is a new question
  if (!(props.topic && props.qid)) {
    console.log("Adding a new question");
    return;
  }

  // fetching an existing question for editing
  console.log(`Fetching question for ${props.topic}/${props.qid}`);

  try {
    const response = await fetch(`${QUESTION_HANDLER_URL}${URL_PARAM_TOPIC}=${props.topic}&${URL_PARAM_QID}=${props.qid}`, {
      method: "GET",
      headers: {
        [TOKEN_HEADER_NAME]: token,
      }
    });

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      try {
        const question = <Question>await response.json();
        console.log(`Fetched. Status: ${response.status}`);
        // console.log(question);

        // set debounced values before the main values to avoid triggering out of index errors
        // in the template
        answersDebounced.value = JSON.parse(JSON.stringify(question.answers));
        questionTextDebounced.value = question.question;

        // copy DDB values to the form models
        selectedTopic.value = question.topic;
        questionText.value = question.question;

        // copy the array while maintaining a reference to the original object
        // https://stackoverflow.com/questions/71353509/why-would-a-vue3-watcher-of-a-prop-not-be-triggered-composition-api
        answers.value.length = 0;
        question.answers.forEach((answer: Answer) => {
          answers.value.push(answer);
        });

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
