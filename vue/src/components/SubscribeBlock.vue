<template>
  <div class="text-start">
    <Button label="Subscribe now for free" icon="pi pi-envelope" raised rounded class="font-bold px-8 py-4 me-4 mb-2 whitespace-nowrap" :disabled="!canSubscribe" @click="subscribe" />
    <p v-if="!selectedTopics.length" class="text-xs text-start text-slate-500">Select at least one topic</p>
    <p v-else class="text-xs text-start md:mb-auto text-slate-500">One new question per day.<br />Auto-paused if you get a backlog.<br />One-click unsubscribe.</p>
  </div>
</template>


<script setup lang="ts">
import { useAuth0 } from '@auth0/auth0-vue';
import { computed } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { USER_HANDLER_URL, TOKEN_HEADER_NAME, URL_PARAM_TOPICS } from "@/constants";
import Button from 'primevue/button';

const { idTokenClaims } = useAuth0();
const store = useMainStore();
const { selectedTopics } = storeToRefs(store);
const canSubscribe = computed(() => selectedTopics.value.length > 0);

async function subscribe() {
  let token = idTokenClaims.value?.__raw;
  if (!token) {
    console.log("No token found.");
    return;
  }

  // create a URL with list of topics
  const url = `${USER_HANDLER_URL}${URL_PARAM_TOPICS}=${selectedTopics.value.map((t) => t).join(".")}`;
  try {
    const response = await fetch(url, {
      headers: {
        [TOKEN_HEADER_NAME]: token,
      },
    });

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 204) { console.log("Subscribed successfully"); }
    else {
      console.error("Failed to subscribe: ", response.status);
    }
  } catch (error) {
    console.error(error);
  }
}

</script>
