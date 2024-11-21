<template>
  <div>
    <div class="flex flex-wrap gap-4 mb-4">
      <InputText type="text" v-model="contributorName" placeholder="Your name" size="small" class="flex-grow" />
      <InputText type="text" v-model="contributorProfileUrl" placeholder="Link to your profile or a project" size="small" class="flex-grow" />
      <InputText type="text" v-model="contributorImageUrl" placeholder="Link to your logo or avatar" size="small" class="flex-grow" />
    </div>
    <div class="text-end">
      <TransitionSlot>
        <p v-if="isDifferentFromLS" class="text-sm"><span class="link" @click.prevent="saveDefaultContributorDetails">Save</span> these contributor details as my default for other questions</p>
        <p v-else-if="canApplyDefaultContributor" class="text-sm">Set <span class="link" @click.prevent="applyDefaultContributorDetails">{{ contributorInLS?.name }}</span> as the contributor for this question</p>
      </TransitionSlot>
    </div>
  </div>
</template>


<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import type { ContributorProfile } from '@/constants'
import { CONTRIBUTOR_DETAILS_LS_KEY } from "@/constants";
import _ from 'lodash';

import InputText from 'primevue/inputtext';
import TransitionSlot from "./TransitionSlot.vue";

const store = useMainStore();
const { question } = storeToRefs(store);

// this relies on the component being loaded after the store was updated with the data
// fetched from the server
const contributorName = ref(question.value?.contributor?.name || "");
const contributorProfileUrl = ref(question.value?.contributor?.url);
const contributorImageUrl = ref(question.value?.contributor?.imgUrl);
const isDifferentFromLS = ref(false); // true if the current values are different from the what is in the local storage

const contributorInLsOnMount = localStorage.getItem(CONTRIBUTOR_DETAILS_LS_KEY);
const contributorInLS = ref(contributorInLsOnMount ? <ContributorProfile>JSON.parse(contributorInLsOnMount) : undefined);

const canApplyDefaultContributor = computed(() => contributorInLS.value?.name && !question.value?.contributor);

/// Saves the default contributor details to local storage
const saveDefaultContributorDetails = () => {
  const contributorDetails = <ContributorProfile>{
    name: contributorName.value,
    url: contributorProfileUrl.value,
    imgUrl: contributorImageUrl.value
  };

  localStorage.setItem(CONTRIBUTOR_DETAILS_LS_KEY, JSON.stringify(contributorDetails));
  contributorInLS.value = contributorDetails;
  isDifferentFromLS.value = false;
}

/// Copies the default contributor from LS into the question
const applyDefaultContributorDetails = () => {
  if (question.value && contributorInLS.value?.name) {
    // setting the local refs here because setting the question in store does not update the local refs
    // the store is updated via watch
    contributorName.value = contributorInLS.value?.name;
    contributorProfileUrl.value = contributorInLS.value?.url;
    contributorImageUrl.value = contributorInLS.value?.imgUrl;

    isDifferentFromLS.value = false;
  }
  else {
    console.log('No question in storage');
  }
}

watch([contributorName, contributorProfileUrl, contributorImageUrl], ([name, profileUrl, imgUrl]) => {
  console.log('Contributor details changed');
  if (!question.value) {
    console.log('No question in storage');
    return;
  }
  if (!question.value?.contributor) {
    // this should not happen, but in case the structure is not there we create it from scratch
    question.value.contributor = <ContributorProfile>{ name, url: profileUrl, imgUrl: imgUrl };
  }
  else {
    // update the values since the structure may have other fields we should not overwrite
    question.value.contributor.name = name;
    question.value.contributor.url = profileUrl;
    question.value.contributor.imgUrl = imgUrl;
  }

  // enable / disable Save as default button based on whether the current values are different from the saved values
  isDifferentFromLS.value = !_.isEqual(question.value.contributor, contributorInLS.value);
});

</script>