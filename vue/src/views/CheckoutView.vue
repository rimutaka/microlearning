<template>
  <div class="mb-12 md:mb-20">
    <div class="flex mb-12 md:mb-20">
      <div class="flex-shrink md:flex-grow">
      </div>

      <div class="card flex-grow md:flex-shrink cta-box">
        <SponsorshipCTA />
        <QuestionContributor />
        <div>
          <input type="radio" class="m-2" id="addContributorTrue" name="addContributor" :value="true" v-model="addContributor" />
          <label class="me-4 text-xs" for="addContributorTrue">Add your details</label>
          <input type="radio" class="m-2" id="addContributorFalse" name="addContributor" :value="false" v-model="addContributor" />
          <label for="addContributorFalse" class="text-xs">Remain anonymous</label>
          <QuestionContributorForm v-if="addContributor" class="mt-4" :autosave="true" />
        </div>
      </div>
      <div class="flex-shrink md:flex-grow">
      </div>
    </div>

    <SponsorshipForm />
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect, watch } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import type { Question } from '@/interfaces';

import TransitionSlot from "@/components/TransitionSlot.vue";
import SponsorshipCTA from '@/components/SponsorshipCTA.vue';
import SubscriptionCompleted from '@/components/SubscriptionCompleted.vue';
import SponsorshipForm from '@/components/SponsorshipForm.vue';
import QuestionCard from '@/components/QuestionCard.vue';
import QuestionContributor from '@/components/QuestionContributor.vue';
import QuestionContributorForm from '@/components/QuestionContributorForm.vue';



const store = useMainStore();
const { user, componentKey, question } = storeToRefs(store);
const isHydrated = ref(false);
const firstTimeSub = ref(false); // true when the user changes from no sub to sub with topics
const addContributor = ref<boolean>(false);

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
    answers: [{ e: "lorem" }], // needed to show the contributor section in full color
    contributor: {
      name: "Your name",
      url: "https://your-website.com",
      imgUrl: "/your-logo.png",
      about: "I am a contributor",
    },
  }

});

</script>
