<template>
  <div v-if="question?.qid" class="">
    <div class="mb-4">
      <div class="w-full">
        <p class="mb-8">All questions should be <i>correct</i>, <i>concise</i> and <i>unambiguous</i>.</p>
        <p class="mb-8">Please report any issues and corrections to help us improve.</p>
        <Textarea v-model="feedbackText" class="w-full" rows="3" />

      </div>
    </div>
    <div class="flex-shrink text-end flex-row space-x-4 space-y-4">
      <Button label="Send feedback" icon="pi pi-send" severity="secondary" class="my-auto whitespace-nowrap" @click="submitFeedback()" />
    </div>
  </div>
</template>


<script setup lang="ts">
import { ref } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { Sha256 } from '@aws-crypto/sha256-js';
import { toHex } from "uint8array-tools";

import { QUESTION_FEEDBACK_HANDLER_URL, URL_PARAM_TOPIC, URL_PARAM_QID, TOKEN_HEADER_NAME } from "@/constants";


import Button from 'primevue/button';
import Textarea from 'primevue/textarea';

const store = useMainStore();
const { token, question } = storeToRefs(store);

const feedbackText = ref("");

/** Save question in the cloud */
async function submitFeedback() {
  // validate input
  if (feedbackText.value.length < 10 || feedbackText.value.length > 3000) {
    console.error("Feedback is too short or too long");
    return;
  }

  if (!question.value?.qid || !question.value?.topic) {
    console.error("No question ID or topic found");
    return;
  }
  // calculate the hash of the request body for x-amz-content-sha256 header
  // as required by CloudFront
  const hash = new Sha256();
  hash.update(feedbackText.value);
  const bodyHash = toHex(await hash.digest());

  // user token is optional, but it's nice to know who submitted the feedback
  const headers = new Headers();
  headers.append("x-amz-content-sha256", bodyHash);
  if (token.value) headers.append(TOKEN_HEADER_NAME, token.value);

  try {
    const response = await fetch(`${QUESTION_FEEDBACK_HANDLER_URL}${URL_PARAM_TOPIC}=${question.value.topic}&${URL_PARAM_QID}=${question.value.qid}`, {
      method: "POST",
      body: feedbackText.value,
      headers,
    });

    // 204 on success, anything else is an error
    console.log("Response status: ", response.status);
    if (response.status === 204) {
      console.log("Feedback submitted OK");
    }
    else {
      console.error("Failed to submit feedback: ", response.status);
    }
  } catch (error) {
    console.error(error);
  }
}

</script>
