<template>
  <TransitionSlot>
    <HomeCTA />
  </TransitionSlot>
  <HomeForm />
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import router from '@/router';
import { PageIDs } from '@/router';


import TransitionSlot from "@/components/TransitionSlot.vue";
import HomeForm from '@/components/HomeForm.vue';
import HomeCTA from '@/components/HomeCTA.vue';

const store = useMainStore();
const { token } = storeToRefs(store);

/// redirect to subscription page if the user is authenticated
watchEffect(() => {
  // this redirect has to be here to redirect from homepage only
  // any other page should not redirect to sub automatically
  if (token.value) {
    console.log("Token obtained - redirecting to subscription page");
    router.replace({ name: PageIDs.SUBSCRIPTION }); // cleaner history with replace
  }
});

</script>
