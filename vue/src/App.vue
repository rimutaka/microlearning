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

const { isAuthenticated, isLoading, idTokenClaims } = useAuth0();
const store = useMainStore();
const { email, token } = storeToRefs(store);

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

watchEffect(() => {
  addTokenClaimsToStore();
});

</script>