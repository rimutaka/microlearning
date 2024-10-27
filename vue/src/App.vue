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
import { LAST_AUTH_TIMESTAMP, URL_PARAM_TOPICS, URL_PARAM_LIST_SEPARATOR } from "@/constants";
import { useMainStore } from '@/store';

import TopHeader from './components/TopHeader.vue';
import FooterStatic from './components/FooterStatic.vue';

const { isAuthenticated, isLoading, loginWithRedirect, getAccessTokenSilently, idTokenClaims } = useAuth0();
const store = useMainStore();
const { selectedTopics, email, token } = storeToRefs(store);
const route = useRoute();

console.log(`App load/auth: ${isLoading.value}/${isAuthenticated.value}`);

// save auth details in the localStorage
if (isLoading.value) {
  console.log("Auth is loading");
} else {
  updateAuth();
}

// needed to minimize auth errors when the token is requested before the auth is complete
watch(isLoading, (newVal, OldVal) => {
  if (newVal) {
    console.log(`Auth is still loading: ${newVal} / ${OldVal}`);
  } else {
    updateAuth();
  }
});

function updateAuth() {
  if (isAuthenticated.value) {
    // a flag to tell the app that the user was authenticated before
    // and it is OK to ask to re-authenticate if the token is expired
    localStorage.setItem(LAST_AUTH_TIMESTAMP, Date.now().toString());
    // console.log("Auth status updated in LS");
    console.log(idTokenClaims.value);
    email.value = idTokenClaims.value?.email;
    token.value = idTokenClaims.value?.__raw;
    // console.log(`Email: ${email.value}`);
  }
  else {
    // console.log("Not authenticated");
    (async () => {
      // log in the user if the the user was logged in before
      const lastAuth = localStorage.getItem(LAST_AUTH_TIMESTAMP);
      // console.log(`Last auth/auth'd: ${lastAuth}/${isAuthenticated.value}`);
      if (lastAuth && !isAuthenticated.value) {
        console.log("User was logged in before, logging in again");
        try {
          const accessToken = await getAccessTokenSilently();
          // console.log(`Access token: ${accessToken}`);
        } catch (e) {
          console.log(`Error getting access token: ${e}`);
          loginWithRedirect();
        }
      }
    })();
  }
}

watchEffect(() => {

  // use query string parameters to preset the selected topics
  const qsTopics = route.query[URL_PARAM_TOPICS]?.toString();
  if (qsTopics) {
    console.log("Setting selected topics from query string: ", qsTopics);
    selectedTopics.value = qsTopics.split(URL_PARAM_LIST_SEPARATOR);
  }
});

</script>