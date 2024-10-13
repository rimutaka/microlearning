<template>
  <div class="card mt-12">
    <h3 class="mb-8">New question form</h3>
    <div class="flex flex-wrap gap-4 mb-4">
      <h4>Topics: </h4>
      <div class="flex" v-for="topic in topics" :key="topic.id">
        <RadioButton v-model="selectedTopic" :inputId="topic" name="topics" :value="topic.id" />
        <label :for="topic.id" class="ms-2 me-4">{{ topic.t }}</label>
      </div>
    </div>
    <div class="flex flex-wrap gap-4 mb-4">
      <h4>Question: </h4>
      <div class="w-full">
        <Textarea v-model="questionText" class="w-full" rows="3" />
      </div>
    </div>
    <div class="flex flex-wrap gap-4 mb-8">
      <h4>Answers: </h4>
      <div class="w-full mb-6" v-for="(answer, idx) in answers" :key="idx">
        <Textarea v-model="answer.a" :value="answer.a" rows="3" class="w-full mb-2" placeholder="An answer options (always visible)" />
        <Textarea v-model="answer.e" :value="answer.e" rows="5" class="w-full mb-2" placeholder="A detailed explanation (visible after answering)" />
        <div class="flex">
          <div class="flex-grow justify-start text-start ps-4">
            <input type="radio" v-model="answer.c" :inputId="`c${idx}`" :name="`c${idx}`" :value="true" class="h-8 w-8 checked:bg-green-600 text-green-500 p-3" />
            <label class="ms-2" :for="`c${idx}`">Correct</label>
            <input type="radio" v-model="answer.c" :inputId="`c${idx}`" :name="`c${idx}`" :value="false" class="h-8 w-8 checked:bg-red-600 text-red-500 p-3 ms-6" />
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
import Checkbox from 'primevue/checkbox';
import Textarea from 'primevue/textarea';

import { TOPICS, QUESTION_HANDLER_URL, URL_PARAM_TOPIC } from "@/constants";

import type { Answer, Question } from "@/constants";
import { useRouter } from "vue-router";
import { Sha256 } from '@aws-crypto/sha256-js';
import { toHex } from "uint8array-tools";

const topics = ref(TOPICS);
const selectedTopic = ref("");
const questionText = ref("");
const answers = ref<Array<Answer>>([{ a: "", e: "", c: false }]);
const questionReady = ref(false);

const questionReadiness = ref({
  topic: false,
  question: false,
  answers: false,
  correct: false,
  explanations: false,
});

function addAnswer(index: number) {
  answers.value.splice(index + 1, 0, { a: "", e: "", c: false });
}

function deleteAnswer(index: number) {
  answers.value.splice(index, 1);
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

  const response = await fetch(`${QUESTION_HANDLER_URL}topic=aws`, {
    method: "POST",
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

      // useRouter().push(`/question/${savedQuestion.topic}/${savedQuestion.qid}`);
    } catch (error) {
      console.error(error);
    }
  } else {
    console.error("Failed to save the question: ", response.status);
  }
}

// update questionReadiness list and enable the submit button via questionReady
watch([selectedTopic, questionText, answers.value], () => {
  questionReadiness.value.topic = selectedTopic.value !== "";
  questionReadiness.value.question = questionText.value.length > 10;
  questionReadiness.value.answers = answers.value.length >= 2 && answers.value.every((answer) => answer.a.length > 0);
  questionReadiness.value.correct = answers.value.some((answer) => answer.c);
  questionReadiness.value.explanations = answers.value.every((answer) => answer.e.length > 10);
  questionReady.value = Object.values(questionReadiness.value).every((value) => value);
  // console.log(questionReadiness.value);
});

</script>
