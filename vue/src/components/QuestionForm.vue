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
      <div class="w-full md-group">
        <Textarea v-model="questionText" class="w-full" rows="3" />
        <QuestionFieldMarkdown :text="questionTextDebounced" />
      </div>
    </div>
    <div class="flex flex-wrap gap-4 mb-8">
      <h4>Answers: </h4>
      <div class="w-full mb-6" v-for="(answer, idx) in answers" :key="idx">
        <div class="md-group mb-2">
          <Textarea v-model="answer.a" :value="answer.a" rows="3" class="w-full" placeholder="An answer options (always visible)" />
          <QuestionFieldMarkdown :text="answersDebounced[idx].a" />
        </div>
        <div class="md-group mb-2">
          <Textarea v-model="answer.e" :value="answer.e" rows="5" class="w-full" placeholder="A detailed explanation (visible after answering)" />
          <QuestionFieldMarkdown :text="answersDebounced[idx].e" />
        </div>

        <div class="flex">
          <div class="flex-grow justify-start text-start ps-4">
            <input type="radio" v-model="answer.c" :name="`c${idx}`" :value="true" class="h-8 w-8 checked:bg-green-600 text-green-500 p-3" />
            <label class="ms-2" :for="`c${idx}`">Correct</label>
            <input type="radio" v-model="answer.c" :name="`c${idx}`" :value="false" class="h-8 w-8 checked:bg-red-600 text-red-500 p-3 ms-6" />
            <label class="ms-2" :for="`c${idx}`">Incorrect</label>
          </div>
          <div class="flex-shrink">
            <Button label="Add another answer" icon="pi pi-plus" severity="secondary" rounded class="ms-4 whitespace-nowrap" @click="addAnswer(idx)" />
            <Button label="Delete this answer" icon="pi pi-trash" severity="secondary" rounded class="ms-4 whitespace-nowrap" @click="deleteAnswer(idx)" />
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
import { ref, watch, computed } from "vue";

import Button from 'primevue/button';
import RadioButton from 'primevue/radiobutton';
import Textarea from 'primevue/textarea';
import QuestionFieldMarkdown from "./QuestionFieldMarkdown.vue";

import { TOPICS, QUESTION_HANDLER_URL, URL_PARAM_TOPIC, URL_PARAM_QID } from "@/constants";

import type { Answer, Question } from "@/constants";
import { useRouter } from "vue-router";
import { Sha256 } from '@aws-crypto/sha256-js';
import { toHex } from "uint8array-tools";
import debounce from "lodash.debounce"

const router = useRouter();

const topics = ref(TOPICS);
const selectedTopic = ref(""); // the topic of the question from TOPICS
const questionText = ref(""); // the text of the question in markdown
const questionTextDebounced = ref(""); // for HTML conversion
const answers = ref<Array<Answer>>([{ a: "", e: "", c: false }]); // the list of answers
const answersDebounced = ref<Array<Answer>>([{ a: "", e: "", c: false }]); // for HTML conversion
const questionReady = ref(false); // enables Submit button

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
  answers.value.splice(index, 1);
  answersDebounced.value.splice(index, 1);

}

async function submitQuestion() {
  // the lambda gets all it needs from the serialized JSON object
  const submissionQuestion = JSON.stringify(<Question>{
    qid: "",
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


</script>
