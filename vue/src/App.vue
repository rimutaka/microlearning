<template>
  <header class="">
    <TopHeader />
  </header>
  <main class="mb-auto w-full">
    <RouterView />
  </main>
  <footer class="">
    <FooterStatic />
  </footer>
</template>

<script setup lang="ts">
import { watch, watchEffect } from 'vue';
import { RouterView, useRoute } from 'vue-router'
import { useAuth0 } from '@auth0/auth0-vue';
import { storeToRefs } from 'pinia'
import { URL_PARAM_TOPICS, URL_PARAM_LIST_SEPARATOR } from "@/constants";
import { useMainStore } from '@/store';

import TopHeader from './components/TopHeader.vue';
import FooterStatic from './components/FooterStatic.vue';

const { isAuthenticated, isLoading, loginWithRedirect, getAccessTokenSilently, idTokenClaims } = useAuth0();
const store = useMainStore();
const { selectedTopics, email, token } = storeToRefs(store);
const route = useRoute();

console.log(`App load/auth: ${isLoading.value}/${isAuthenticated.value}`);

// get token details as soon as the user is authenticated
watch(isAuthenticated, (newVal) => {
  if (newVal) {
    addTokenClaimsToStore();
  }
});

/// copy token details to the store
function addTokenClaimsToStore() {
  if (!isAuthenticated.value) {
    console.log("Cannot update token - not authenticated");
    return;
  }

  // console.log("Auth status updated in LS");
  console.log("idTokenClaims", idTokenClaims.value);
  // console.log(`Email: ${email.value}`);
  email.value = idTokenClaims.value?.email;
  token.value = idTokenClaims.value?.__raw;
}

/// 1. copy the list of topics from the URL to the store
/// 2. copy the token details to the store
watchEffect(() => {
  // use query string parameters to preset the selected topics
  const qsTopics = route.query[URL_PARAM_TOPICS]?.toString();
  if (qsTopics) {
    console.log("Setting selected topics from query string: ", qsTopics);
    selectedTopics.value = qsTopics.split(URL_PARAM_LIST_SEPARATOR);
  }

  addTokenClaimsToStore();
});

</script>