<template>
  <div>
    <LoadingMessage v-if="questionListStatus == undefined || questionListStatus == LoadingStatus.Loading" :msg="`Loading questions about ${topicName}`" />
    <div v-if="questionListStatus == LoadingStatus.Loaded" class="q-list">
      <QuestionListItem v-for="(question, index) in questions" :question="question" :key="index" />
    </div>
    <div v-if="questionListStatus == LoadingStatus.Error || questionListStatus == LoadingStatus.NoData">Cannot load the list of questions - something went wrong.</div>
  </div>
</template>

<script setup lang="ts">
import { computed, watchEffect, ref } from 'vue';
import * as constants from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { LoadingStatus } from '@/interfaces';
import type { QuestionWithHistory } from '@/interfaces';

import QuestionListItem from '@/components/QuestionListItem.vue';
import LoadingMessage from '@/components/LoadingMessage.vue';

const store = useMainStore();
const { token, questionListStatus } = storeToRefs(store);

const props = defineProps<{ topic: string }>();

const questions = ref<QuestionWithHistory[]>([]);

const topicName = computed(() => {
  return (constants.TOPICS.find((t) => t.id == props.topic))?.t;
});

/// The topic always comes from props.topic
/// The qid comes from props.qid if random is false.
const loadQuestions = async (paramTopic: string) => {

  console.log(`Fetching questions for: ${paramTopic}`);

  // make sure nothing is showing if the component is reused
  questionListStatus.value = LoadingStatus.Loading;

  if (!paramTopic) {
    console.error("No topic provided - using any.");
    questionListStatus.value = LoadingStatus.Error;
    return;
  }

  // add a token with the email, if there is one (logged in users)
  const headers = new Headers();
  if (token.value) headers.append(constants.TOKEN_HEADER_NAME, token.value);

  try {
    const response = await fetch(`${constants.QUESTION_LIST_HANDLER_URL}${constants.URL_PARAM_TOPIC}=${paramTopic}`,
      {
        headers: headers,
        signal: AbortSignal.timeout(5000),
      });
    console.log(`Fetched. Status: ${response.status}`);

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      try {
        questions.value = <QuestionWithHistory[]>await response.json();
        // console.log(questions.value);
        console.log(`Questions loaded: ${questions.value.length}`);

        // update the local loading status
        questionListStatus.value = LoadingStatus.Loaded;
      } catch (error) {
        console.error(error);
        console.error("Failed to parse list of questions.");
      }
    }
    else {
      questionListStatus.value = LoadingStatus.Error;
      console.error("Failed to get questions. Status: ", response.status);
    }
  } catch (error) {
    questionListStatus.value = LoadingStatus.Error;
    console.error("Failed to get questions.");
    console.error(error);
  }
};

watchEffect(async () => {
  await loadQuestions(props.topic);
});

</script>
