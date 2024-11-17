<template>
  <h1 class="mb-4 md:mb-8 text-2xl text-start">Question about: <em class="italic">{{ topicName }}</em></h1>
  <QuestionCard :topic="topic" :qid="qid" :next="false" :use-store="true"/>
</template>

<script setup lang="ts">
import { computed, watch, ref, watchEffect } from 'vue';
import { useRoute, useRouter } from 'vue-router'
import { TOPICS, PREVIEW_QUESTION } from "@/constants";
import type { Question, Answer } from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { marked } from 'marked';


import QuestionCard from "../components/QuestionCard.vue";

const route = useRoute();
const router = useRouter();

const store = useMainStore();
const { question, componentKey, email } = storeToRefs(store);

const topicName = computed(() => {
  return (TOPICS.find((t) => t.id == topic.value))?.t;
});

const qid = computed(() => question.value ? question.value.qid : "");
const topic = computed(() => question.value ? question.value.topic : "...");

/// updates the the vue store with the question data received in the message
function questionUpdateListener(event: MessageEvent) {
  console.log("event: ", event);
  if (event.origin !== window.origin) return;

  // …
}

window.removeEventListener("message", questionUpdateListener, false);
window.addEventListener("message", questionUpdateListener, false);

watchEffect(async () => {
  console.log("Loading question from LS");
  const qLS = localStorage.getItem(PREVIEW_QUESTION);
  // console.log(qLS);

  if (!qLS) {
    console.log("No question in LS");
    question.value = undefined;
    return;
  }

  // convert to a Question object
  const qMarkdown: Question = JSON.parse(qLS);

  // prepared the markdown to HTML renderer
  marked.use({
    async: true,
    breaks: true,
  });

  // render answers one by one
  const answersHtml: Answer[] = await Promise.all(qMarkdown.answers.map(async (a): Promise<Answer> => {
    return {
      a: a.a ? await marked.parse(a.a) : "",
      c: a.c,
      e: a.e ? await marked.parse(a.e) : "",
      sel: a.sel,
    }
  }));

  const qHtml: Question = {
    qid: qMarkdown.qid,
    topic: qMarkdown.topic,
    question: qMarkdown.question ? await marked.parse(qMarkdown.question) : "",
    answers: answersHtml,
    author: qMarkdown.author,
    updated: qMarkdown.updated,
    correct: qMarkdown.correct,
    stats: qMarkdown.stats,
  }

  // save the question in the store
  question.value = qHtml;

  // console.log("question.value: ", question.value);

});

</script>
