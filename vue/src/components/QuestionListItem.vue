<template>
  <div :class="status.class">
    <p>
      <i></i>
      <router-link :to="questionTopicAndPageUrl" :aria-label="status.class + title">{{ title }}</router-link>
    </p>
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect, computed, watch } from "vue";
import router, { PageIDs } from "@/router";

import type { QuestionWithHistory } from "@/interfaces";
import * as constants from "@/constants";

const props = defineProps<{
  question: QuestionWithHistory,
}>()

const title = computed(() => props.question.question?.title ? props.question.question?.title : "Untitled question");

// contains CSS class and aria-label for the question
const status = computed((): { class: string, aria: string } => {
  if (props.question?.history?.[0].correct) return { class: "correct", aria: "Correctly answered before, " };
  if (props.question?.history?.[0].incorrect) return { class: "incorrect", aria: "Incorrect answer last time, " };
  if (props.question?.history?.[0]) return { class: "viewed", aria: "Viewed before, " };
  return {class: "", aria: ""}; // not viewed
});

/// A URL to the page with the question on its own
/// for sharing or opening separately
const questionTopicAndPageUrl = computed(() => `/${PageIDs.QUESTION}?${constants.URL_PARAM_TOPIC}=${props.question.question.topic}&${constants.URL_PARAM_QID}=${props.question.question.qid}`);

</script>
