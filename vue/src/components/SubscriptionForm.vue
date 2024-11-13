<template>
  <div class="card mt-8">
    <h3 v-if="loading == LoadingStatus.Loading || !loading">Loading topics ...</h3>
    <h3 v-else-if="loading == LoadingStatus.Loaded">Your topics of interest</h3>
    <h3 v-else-if="loading == LoadingStatus.NoData">Select your topics of interest</h3>
    <TopicsList v-if="loading == LoadingStatus.Loaded || loading == LoadingStatus.NoData" :key="user?.updated" />

    <div class="mt-4 md:mt-12 mb-12">
      <div v-if="subscriptionStatus == SubscriptionStatus.LoadedWithSubs" class="text-center">
        <div v-if="inUpdateMode" class="update-sub">
          <div class="flex mb-4">
            <div class="mx-auto">
              <ul>
                <li>Topics to add: </li>
                <li v-for="topic in topicsToAdd" :key="topic">{{ findTopicById(topic) }}</li>
                <li v-if="!topicsToAdd?.length">none</li>
              </ul>
              <ul>
                <li>Topics to remove: </li>
                <li v-for="topic in topicsToRemove" :key="topic">{{ findTopicById(topic) }}</li>
                <li v-if="!topicsToRemove?.length">none</li>
              </ul>
            </div>
          </div>
          <Button label="Cancel" icon="pi pi-times" raised severity="secondary" class="font-bold px-8 py-4 me-2 mb-2 whitespace-nowrap" @click="inUpdateMode = false" />
          <Button label="Confirm" icon="pi pi-check" raised class="font-bold px-8 py-4 ms-2 mb-2 whitespace-nowrap" :disabled="!canUpdateSub" @click="subscribe" />
          <p v-if="saving == LoadingStatus.Loading" class="">Saving ...</p>
          <p v-if="saving == LoadingStatus.Error" class="text-red-600">Failed to update your subscription. Refresh the page and try again.</p>
        </div>

        <Button v-if="!inUpdateMode" label="Update subscription" icon="pi pi-pencil" raised class="font-bold px-8 py-4 mb-2 whitespace-nowrap" @click="inUpdateMode = true" />
        <p v-if="loading == LoadingStatus.Loaded && !inUpdateMode" class="mb-4 text-xs text-slate-500">Last updated: {{ updateDate }} </p>

      </div>

      <div v-if="subscriptionStatus == SubscriptionStatus.LoadedNoSubs" class="update-sub">
        <p class="mb-4">
          <span v-if="topicsReminder && !canSubscribe" class="text-red-600 mb-4">Select at least one topic</span>
          <span v-else-if="saving == LoadingStatus.Loading" class="">Saving ...</span>
          <span v-else-if="saving == LoadingStatus.Error" class="text-red-600">Failed to update your subscription. Refresh the page and try again.</span>
          <span v-else>&nbsp;</span>
        </p>
        <Button label="Subscribe" icon="pi pi-envelope" raised class="font-bold px-8 py-4 mb-2 whitespace-nowrap" :disabled="saving == LoadingStatus.Loading" @click="subscribe" />
      </div>

      <div v-if="!inUpdateMode && (loading == LoadingStatus.Loaded || loading === LoadingStatus.NoData)" class="text-center mb-4">
        <p class="w-full text-center mt-2 mb-4 text-sm">or</p>
        <RandomQuestionButton />
      </div>
      <h3 v-if="loading == LoadingStatus.Error" class="mt-8 mb-8 text-center text-slate-500">Sorry, something went wrong. Try again.</h3>
    </div>
  </div>
</template>


<script setup lang="ts">
import { computed, ref, watchEffect, watch } from "vue";
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { USER_HANDLER_URL, TOKEN_HEADER_NAME, URL_PARAM_TOPICS, URL_PARAM_LIST_SEPARATOR, findTopicById, VUE_EVENT_HYDRATED } from "@/constants";
import { type User, LoadingStatus } from "@/constants";

import Button from 'primevue/button';
import TopicsList from './TopicsList.vue';
import RandomQuestionButton from "./RandomQuestionButton.vue";

const emit = defineEmits([VUE_EVENT_HYDRATED]);

const store = useMainStore();
const { selectedTopics, token, email, user } = storeToRefs(store);
const route = useRoute();

const topicsReminder = ref(false); // true if attempted to subscribe without selecting topics to show a prompt
const loading = ref<LoadingStatus>(); // user details fetch status: loading, loaded, none
const saving = ref<LoadingStatus>(); // set while updating user subs, only loading and error are used, otherwise undefined
const inUpdateMode = ref(false); // true if the update panel is expanded

/// A new sub requires at least one topic, an update may have no topics selected
const canSubscribe = computed(() => selectedTopics.value.length || user.value?.topics.length);

enum SubscriptionStatus {
  LoadedNoSubs,
  LoadedWithSubs,
}
/// A composite of the user subscription status to display Subscribe / Update buttons
const subscriptionStatus = computed(() => {
  if (loading.value === LoadingStatus.Loaded && user.value?.topics.length) return SubscriptionStatus.LoadedWithSubs;
  if (loading.value === LoadingStatus.Loaded && !user.value?.topics.length) return SubscriptionStatus.LoadedNoSubs;
  if (loading.value === LoadingStatus.NoData) return SubscriptionStatus.LoadedNoSubs;
  if (loading.value === LoadingStatus.Error || loading.value == LoadingStatus.Loading) return undefined;
  console.error("Unknown status (it's a bug): ", loading.value);
});


const topicsToRemove = computed(() => {
  return user.value?.topics.filter((t) => !selectedTopics.value.includes(t));
});

const topicsToAdd = computed(() => {
  return selectedTopics.value.filter((t) => !user.value?.topics.includes(t));
});

const canUpdateSub = computed(() => topicsToAdd.value.length || topicsToRemove.value?.length);

/// Format RFC3339 date string to a human-readable date
const updateDate = computed(() => {
  if (!user.value?.updated) return "";
  return new Date(user.value.updated).toLocaleString();
});

async function subscribe() {
  // console.log("Subscribing to topics: ", selectedTopics.value);
  const subTopics = selectedTopics.value.map((t) => t).join(URL_PARAM_LIST_SEPARATOR);

  // if no topics are selected, show a prompt and return
  if (!canSubscribe.value) {
    topicsReminder.value = true;
    return;
  }

  if (!token.value) {
    console.log("No token found.");
    return; // unreachable code
  }

  // create a URL with list of topics
  const url = `${USER_HANDLER_URL}${URL_PARAM_TOPICS}=${subTopics}`;
  try {
    saving.value = LoadingStatus.Loading;
    const response = await fetch(url, {
      headers: {
        [TOKEN_HEADER_NAME]: token.value,
      },
    });

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      console.log("Subscribed successfully");
      try {
        user.value = <User>await response.json();
        console.log(user.value);

        // set selected topics to user's topics
        selectedTopics.value = user.value.topics;
        saving.value = undefined;
        loading.value = LoadingStatus.Loaded;
        inUpdateMode.value = false;
      } catch (error) {
        saving.value = LoadingStatus.Error;
        console.error(error);
      }

    }
    else {
      saving.value = LoadingStatus.Error;
      console.error("Failed to subscribe: ", response.status);
    }
  } catch (error) {
    saving.value = LoadingStatus.Error;
    console.error(error);
  }
}

// emit an event to notify the parent that the component is hydrated
watch(loading, (newVal) => {
  if (newVal === LoadingStatus.Loaded || newVal === LoadingStatus.NoData) {
    emit(VUE_EVENT_HYDRATED);
  }
});

watchEffect(async () => {
  console.log(`Fetching user details for: ${email.value}`);

  // reset the current topic so that no questions are showing until the user presses the button to show one
  store.currentTopic = undefined;

  // only fetch if the user is known
  if (!email.value) return;

  // redirect the user to login with the list of topics as a parameter
  if (!token.value) {
    console.log("No token found.");
    return; // unreachable code
  }

  try {
    loading.value = LoadingStatus.Loading;
    const response = await fetch(USER_HANDLER_URL, {
      headers: {
        [TOKEN_HEADER_NAME]: token.value,
      },
    });

    console.log(`Fetched. Status: ${response.status}`);

    // a successful response should contain the saved question
    // an error may contain JSON or plain text, depending on where the errror occurred
    if (response.status === 200) {
      try {
        user.value = <User>await response.json();
        // console.log(user.value);

        // set selected topics to user's topics if there is an active subscription
        if (user.value?.topics?.length) {
          selectedTopics.value = user.value.topics;
          loading.value = LoadingStatus.Loaded;
        }
        else {
          // a user may have no topics or the user may have been created just now
          loading.value = LoadingStatus.NoData;
          // use query string parameters to preset the selected topics
          const qsTopics = route.query[URL_PARAM_TOPICS]?.toString();
          if (qsTopics) {
            console.log("Setting selected topics from query string: ", qsTopics);
            selectedTopics.value = qsTopics.split(URL_PARAM_LIST_SEPARATOR);
          }
        }
      } catch (error) {
        loading.value = LoadingStatus.Error;
        console.error(error);
      }
    }
    else {
      // only 200 and 404 come from lambda, any other code is an error
      loading.value = LoadingStatus.Error;
      console.error("Failed to get user. Status: ", response.status);
    }
  } catch (error) {
    loading.value = LoadingStatus.Error;
    console.error("Failed to get user.", error);
  }
  finally {
    if (!loading.value) {
      console.log("Loading status was not updated. Setting to error");
      loading.value = LoadingStatus.Error
    };
  }
});

</script>
