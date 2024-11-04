<template>
  <TransitionSlot>
    <HomePitch />
  </TransitionSlot>
  <HomeForm />
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import router from '@/router';
import { useAuth0 } from '@auth0/auth0-vue';
import { PageIDs } from '@/router';


import TransitionSlot from "@/components/TransitionSlot.vue";
import HomeForm from '@/components/HomeForm.vue';
import HomePitch from '@/components/HomePitch.vue';

const store = useMainStore();
const { currentTopic } = storeToRefs(store);
const { isAuthenticated, isLoading } = useAuth0();

/// redirect to subscription page if the user is authenticated
watchEffect(() => {
  console.log(`HomeView: Auth status: ${isLoading.value}/${isAuthenticated.value}`);
  
  if (isAuthenticated.value) {
    console.log("User is authenticated - redirecting to subscription page");
    router.push({ name: PageIDs.SUBSCRIPTION });
  }
});

</script>
