<template>
  <header class="">
    <TopHeader />
  </header>
  <main class="mb-auto w-full">
    <RouterView />
  </main>
  <footer class="w-full">
    <FooterStatic />
  </footer>
</template>

<script setup lang="ts">
import { watch, watchEffect } from 'vue';
import { RouterView } from 'vue-router'
import { useAuth0 } from '@auth0/auth0-vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import TopHeader from './components/TopHeader.vue';
import FooterStatic from './components/FooterStatic.vue';

const { isAuthenticated, isLoading, idTokenClaims, getAccessTokenSilently } = useAuth0();
const store = useMainStore();
const { email, token } = storeToRefs(store);

console.log(`App load/auth: ${isLoading.value}/${isAuthenticated.value}`);

// get token details as soon as the user is authenticated
watch([isAuthenticated, idTokenClaims], ([newIsAuth, newIdClaims]) => {
  console.log(`isAuthenticated updated: ${newIsAuth}`);
  console.log(`idTokenClaims present: ${newIdClaims ? true : false}`);

  if (newIsAuth && newIdClaims) {
    addTokenClaimsToStore();
  }
});

/// copy token details to the store
function addTokenClaimsToStore() {
  if (!isAuthenticated.value) {
    console.log("Cannot update token - not authenticated");
    return;
  }

  email.value = idTokenClaims.value?.email;
  token.value = idTokenClaims.value?.__raw;
}

// attempt to get a new token silently
(async () => {
  console.log("Attempting to get a new token silently");
  try {
    const newToken = await getAccessTokenSilently({ detailedResponse: true });
  } catch (e) {
    console.log("Failed to get token silently");
  }
})();

</script>