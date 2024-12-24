<template>
    <div class="text-start q-list">
      <p><router-link :to="questionTopicAndPageUrl">{{ props.question?.title ? props.question?.title : "Untitled question" }}</router-link></p>
   
    </div>

</template>

<script setup lang="ts">
import { ref, watchEffect, computed, watch } from "vue";
import router, { PageIDs } from "@/router";

import type { Question } from "@/interfaces";
import * as constants from "@/constants";

const props = defineProps<{
  question: Question,
}>()



/// A URL to the page with the question on its own
/// for sharing or opening separately
const questionTopicAndPageUrl = computed(() => `/${PageIDs.QUESTION}?${constants.URL_PARAM_TOPIC}=${props.question.topic}&${constants.URL_PARAM_QID}=${props.question.qid}`);

/// A URL to the question edit page
const editPageUrl = computed(() => `/${PageIDs.ADD}?${constants.URL_PARAM_TOPIC}=${props.question.topic}&${constants.URL_PARAM_QID}=${props.question.qid}`);



</script>
