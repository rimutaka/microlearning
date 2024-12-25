<template>
  <h1 class="mb-4 md:mb-8 text-2xl text-start">Questions about <em class="italic">{{ topicName }}</em></h1>
  <LoadingMessage v-if="isLoading" />

  <QuestionListItem v-for="(question, index) in questions" :question="question.question" :key="index" />
  <div v-if="!isLoading && ctaBlockVisible" class="mb-12 md:mt-12 cta-box">
    <PostAnswerCTA />
  </div>
</template>

<script setup lang="ts">
import { computed, watch, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router'
import * as constants from "@/constants";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { useAuth0 } from '@auth0/auth0-vue';
import { LoadingStatus } from '@/interfaces';
import type { QuestionWithHistory } from '@/interfaces';

import QuestionListItem from '@/components/QuestionListItem.vue';
import PostAnswerCTA from '@/components/PostAnswerCTA.vue';
import LoadingMessage from '@/components/LoadingMessage.vue';

const route = useRoute();
const router = useRouter();

const store = useMainStore();
const { token, email, user } = storeToRefs(store);
const { isLoading } = useAuth0();

const topicName = computed(() => {
  return (constants.TOPICS.find((t) => t.id == route.query.topic))?.t;
});

// save the initial values to maintain state later because the topic and qid will change
const initialTopic = route.query.topic ? <string>route.query.topic : "";

// route.query.topic and .qid can potentially be an array, but it should not happen in this app,
// so it is safe to cast them into a string
const topic = ref<string>(initialTopic);
const isLoadingQuestions = ref(LoadingStatus.Loading);
const questions = ref<QuestionWithHistory[]>([]);


const ctaBlockVisible = computed(() => {
  // checking user subs does not work because this page does not load user details
  // if  (!user.value?.topics.length && question.value?.answers?.[0].e) { return true } else { return false };
  if (email.value && user.value?.topics?.length) { return false } else { return true };
});

/// The topic always comes from props.topic
/// The qid comes from props.qid if random is false.
const loadQuestions = async (paramTopic: string) => {

  console.log(`Fetching questions for: ${paramTopic}`);

  // make sure nothing is showing if the component is reused
  isLoadingQuestions.value = LoadingStatus.Loading;

  if (!paramTopic) {
    console.error("No topic provided - using any.");
    isLoadingQuestions.value = LoadingStatus.Error;
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
        console.log(questions.value);
        // console.log(question.topic);
        console.log(`Questions loaded: ${questions.value.length}`);

        isLoadingQuestions.value = LoadingStatus.Loaded;
      } catch (error) {
        console.error(error);
        console.error("Failed to parse list of questions.");
      }
    }
    else {
      isLoadingQuestions.value = LoadingStatus.Error;
      console.error("Failed to get questions. Status: ", response.status);
    }
  } catch (error) {
    isLoadingQuestions.value = LoadingStatus.Error;
    console.error("Failed to get questions.");
    console.error(error);
  }
};


// load the question when the component is mounted
(async () => await loadQuestions(initialTopic))();

</script>
