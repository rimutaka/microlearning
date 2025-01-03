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
import { marked } from 'marked';


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

  renderQuestion(event.data).then((q) => {
    // console.log("Setting question in store");
    question.value = q;
  });
}

window.removeEventListener("message", questionUpdateListener, false);
window.addEventListener("message", questionUpdateListener, false);

/// Renders a markdown question to HTML and returns it as a string
async function renderQuestion(qMarkdown: string) {
  const parsedQuestion: Question = JSON.parse(qMarkdown);
  // prepared the markdown to HTML renderer
  marked.use({
    async: true,
    breaks: true,
  });

  // render answers one by one
  const answersHtml: Answer[] = await Promise.all(parsedQuestion.answers.map(async (a): Promise<Answer> => {
    return {
      a: a.a ? await marked.parse(a.a) : "",
      c: a.c,
      e: a.e ? await marked.parse(a.e) : "",
      sel: a.sel,
    }
  }));

  // collect the data into a Question object and return
  return <Question>{
    qid: parsedQuestion.qid,
    topic: parsedQuestion.topic,
    question: parsedQuestion.question ? await marked.parse(parsedQuestion.question) : "",
    answers: answersHtml,
    author: parsedQuestion.author,
    updated: parsedQuestion.updated,
    correct: parsedQuestion.correct,
    stats: parsedQuestion.stats,
    contributor: parsedQuestion.contributor,
    title: parsedQuestion.title,
  };
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
  question.value = await renderQuestion(qLS);
  questionStatus.value = LoadingStatus.Loaded;
})();

</script>
