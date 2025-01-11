<template>
  <div class="mb-4 md:mb-8 text-start">
    <h1 class="mb-2 text-2xl">Question preview about <em class="italic">{{ question?.title || topicName }}</em></h1>
  </div>
  <QuestionCard :next="false" :is-preview="true" />
  <ContributorCard class="mb-12 mt-8 md:mt-16" />
</template>

<script setup lang="ts">
import { computed, watch, ref, watchEffect } from 'vue';
import { useRoute, useRouter } from 'vue-router'
import { TOPICS, PREVIEW_QUESTION_LS_KEY } from "@/constants";
import type { Question, Answer } from "@/interfaces";
import { LoadingStatus } from '@/interfaces';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import initWasmModule, { md_to_html } from "@/wasm-rust/isbn_mod";

import QuestionCard from "../components/QuestionCard.vue";
import ContributorCard from "../components/ContributorCard.vue";

const route = useRoute();
const router = useRouter();

const store = useMainStore();
const { question, questionStatus } = storeToRefs(store);

const topicName = computed(() => {
  return (TOPICS.find((t) => t.id == topic.value))?.t;
});

const qid = computed(() => question.value ? question.value.qid : "");
const topic = computed(() => question.value ? question.value.topic : "...");

/// updates the the vue store with the question data received in the message
function questionUpdateListener(event: MessageEvent) {
  // console.log("event: ", event);
  // discard any messages not coming from the main window
  if (event.origin !== window.origin) return;

  console.log("Msg received");

  if (!event.data) {
    console.log("No question payload in message");
    question.value = undefined;
    return;
  }

  renderQuestion(event.data);
}

window.removeEventListener("message", questionUpdateListener, false);
window.addEventListener("message", questionUpdateListener, false);

/// Renders a markdown question to HTML and updates the store
async function renderQuestion(qMarkdown: string) {
  const parsedQuestion: Question = JSON.parse(qMarkdown);

  // preload the parsed question if it's not in the store yet
  // remove MD/HTML fields
  if (!question.value || question.value.qid == undefined || question.value.qid !== parsedQuestion.qid) {
    const preload: Question = JSON.parse(qMarkdown);
    preload.question = "";
    preload.answers = [];
    question.value = preload;
  }

  if (question.value == undefined) {
    console.log("Question not loaded from the LS or the message");
    return;
  }

  // prepared the markdown to HTML renderer
  try {
    console.log("Loading Wasm module");
    await initWasmModule();
    console.log("Wasm module loaded");
  }
  catch (e) {
    console.error("Error loading wasm module", e);
    // clear all HTML to prevent an out of sync preview 
    question.value.question = "Preview rendering failed";
    question.value.answers = [];
    return;
  }

  // render the question
  md_to_html(parsedQuestion.question).then((html) => {
    console.log("Wasm rendered `question`");
    question.value!.question = html;
  }).catch((e) => {
    console.error("Error rendering `question`", e);
  });

  // render answers one by one
  question.value.answers = await Promise.all(parsedQuestion.answers.map(async (a): Promise<Answer> => {
    return {
      a: a.a ? await md_to_html(a.a) : "",
      c: a.c,
      e: a.e ? await md_to_html(a.e) : "",
      sel: a.sel,
    }
  }));
}

// Load the question from local storage on mount, then listen for updates
(async () => {
  console.log("Loading question from LS");
  const qLS = localStorage.getItem(PREVIEW_QUESTION_LS_KEY);
  // console.log(qLS);

  if (!qLS) {
    console.log("No question in LS");
    question.value = undefined;
    return;
  }

  // convert to a Question object
  renderQuestion(qLS);
  questionStatus.value = LoadingStatus.Loaded;
})();

</script>
