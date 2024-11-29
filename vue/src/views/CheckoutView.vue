<template>
  <SponsorshipCTA />

  <SponsorshipForm />

</template>

<script setup lang="ts">
import { ref, watchEffect, watch } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import TransitionSlot from "@/components/TransitionSlot.vue";

import SponsorshipCTA from '@/components/SponsorshipCTA.vue';
import SubscriptionCompleted from '@/components/SubscriptionCompleted.vue';
import SponsorshipForm from '@/components/SponsorshipForm.vue';
import QuestionCard from '@/components/QuestionCard.vue';

import type { Question } from '@/constants';


const store = useMainStore();
const { user, componentKey, question } = storeToRefs(store);
const isHydrated = ref(false);
const firstTimeSub = ref(false); // true when the user changes from no sub to sub with topics

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

watchEffect(() => {
  // reset the sample question key to make sure it is not showing
  // when the user transitions from another page
  componentKey.value = undefined;

  question.value = <Question>{
    answers: [{e:"lorem"}],
    contributor: {
      name: "Your name",
      url: "https://your-website.com",
      imgUrl: "/your-logo.png",
      about: "I am a contributor",
    },
  }

});

</script>
