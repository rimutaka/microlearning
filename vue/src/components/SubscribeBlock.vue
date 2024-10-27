<template>
  <div class="">
    <Button label="Subscribe now for free" icon="pi pi-envelope" raised rounded class="font-bold px-8 py-4 md:me-4 mb-2 whitespace-nowrap" @click="subscribe" />
    <p class="text-xs text-start md:mb-auto text-slate-500">One new question per day.<br />Auto-paused if you get a backlog.<br />One-click unsubscribe.</p>
    <p v-if="!selectedTopics.length && topicsRequired" class="text-xs text-start text-red-500">Select at least one topic to subscribe</p>
  </div>
</template>


<script setup lang="ts">
import { useAuth0 } from '@auth0/auth0-vue';
import { computed, ref, watchEffect } from "vue";
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { USER_HANDLER_URL, TOKEN_HEADER_NAME, URL_PARAM_TOPICS, URL_PARAM_LIST_SEPARATOR } from "@/constants";
import Button from 'primevue/button';

const route = useRoute();
const { idTokenClaims, loginWithRedirect } = useAuth0();
const store = useMainStore();
const { selectedTopics } = storeToRefs(store);
const canSubscribe = computed(() => selectedTopics.value.length > 0);
const topicsRequired = ref(false); // true if attempted to subscribe without selecting topics to show a prompt

async function subscribe() {
  console.log("Subscribing to topics: ", selectedTopics.value);

  // if no topics are selected, show a prompt and return
  if (!canSubscribe.value) {
    topicsRequired.value = true;
    return;
  }

  // prepare the list of topics to subscribe to
  const subTopics = selectedTopics.value.map((t) => t).join(URL_PARAM_LIST_SEPARATOR);

  // redirect the user to login with the list of topics as a parameter
  let token = idTokenClaims.value?.__raw;
  if (!token) {
    console.log("No token found.");
    return; // unreachable code
  }

  // create a URL with list of topics
  const url = `${USER_HANDLER_URL}${URL_PARAM_TOPICS}=${subTopics}`;
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
