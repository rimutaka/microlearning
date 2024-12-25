<template>
  <TransitionSlot>
    <SubscriptionCTA v-if="formHydrated && displayCTA && (!user || !user?.topics.length)" />
    <SubscriptionCompleted v-if="formHydrated && firstTimeSub" />
  </TransitionSlot>
  <SubscriptionForm @hydrated="formHydrated = true" />
  <TransitionSlot>
    <SampleQuestion v-if="formHydrated && showingRandomQuestion" />
  </TransitionSlot>
</template>

<script setup lang="ts">
import { ref, watchEffect, watch } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import TransitionSlot from "@/components/TransitionSlot.vue";
import SubscriptionCTA from '@/components/SubscriptionCTA.vue';
import SubscriptionCompleted from '@/components/SubscriptionCompleted.vue';
import SubscriptionForm from '@/components/SubscriptionForm.vue';
import SampleQuestion from "@/components/RandomQuestion.vue";

const store = useMainStore();
const { user, showingRandomQuestion, question } = storeToRefs(store);
const formHydrated = ref(false);
const firstTimeSub = ref(false); // true when the user changes from no sub to sub with topics

const displayCTA = ref(true);
watch(question, (newVal) => {
  if (newVal) {
    displayCTA.value = false;
  }
});

// display first time subscription message when the user changes from having no topics
// to having some, so it is only displayed once
watch(user, (userNew, userOld) => {
  if (userNew && userNew.topics.length && userOld && !userOld.topics.length) {
    firstTimeSub.value = true;
  }
  else {
    firstTimeSub.value = false;
  }
});

// do not show the sample question when the page is first displayed
// it will be shown when the user clicks on the button requesting it
showingRandomQuestion.value= false;

</script>
