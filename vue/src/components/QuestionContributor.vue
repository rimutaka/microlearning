<template>
  <div class="my-12 mx-auto text-center">
    <figure class="max-w-screen-md mx-auto">
      <div class="w-full h-12 mb-4 contributor"></div>
      <blockquote>
        <p class="text-l font-medium  dark:text-slate-200 dark:opacity-70 max-w-md mx-auto">This question was contributed by a generous community member</p>
      </blockquote>
      <figcaption class="flex items-center justify-center mt-6 space-x-3">
        <div v-if="contributorImgUrl" class="w-8 min-w-8 h-8 bg-contain bg-no-repeat bg-center rounded-sm" :style="`background-image: url(${contributorImgUrl})`"></div>

        <div class="flex items-center divide-x-2 divide-slate-500 text-slate-500 dark:divide-slate-300 dark:text-slate-300 dark:opacity-70">
          <div class="pe-3 font-medium">{{ contributorName }}</div>
          <div v-if="contributorAbout" class="px-3 text-sm font-light hidden md:block">{{ contributorAbout }}</div>
          <div v-if="linkIcon" class="ps-3" style="height: 100%;">
            <a :href="contributorUrl" class="text-slate-500 dark:text-slate-300 me-2 my-auto"><i class="pi ms-1" :class="linkIcon"></i></a>
          </div>
        </div>

      </figcaption>
      <div v-if="contributorAbout" class="px-3 mt-2 text-xs font-light md:hidden w-full">{{ contributorAbout }}</div>
    </figure>
  </div>
</template>


<script setup lang="ts">
import { ref, watch, watchEffect, computed } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import type { ContributorProfile } from '@/constants'
import { CONTRIBUTOR_DETAILS_LS_KEY } from "@/constants";

const store = useMainStore();
const { question } = storeToRefs(store);

// this relies on the component being loaded after the store was updated with the data
// fetched from the server
const contributorName = ref(question.value?.contributor?.name);
const contributorUrl = ref(question.value?.contributor?.url);
const contributorImgUrl = ref(question.value?.contributor?.imgUrl);
const contributorAbout = ref(question.value?.contributor?.about);

/** Returns the appropriate icon for the link */
const linkIcon = computed(() => {
  let url = question.value?.contributor?.url;

  if (!url) return undefined;

  if (url.startsWith("https://github.com")) return "pi-github";
  if (url.startsWith("https://www.linkedin.com")) return "pi-linkedin";
  if (url.startsWith("mailto:")) return "pi-envelope";

  return "pi-external-link"

});

// the changes come via a message to the parent component, then to the store, then here
// taking the data directly from the store did not work
watch(question, () => {
  // watch([question.value?.contributor?.name,question.value?.contributor?.imgUrl, question.value?.contributor?.url ],() => {
  contributorName.value = question.value?.contributor?.name;
  contributorUrl.value = question.value?.contributor?.url;
  contributorImgUrl.value = question.value?.contributor?.imgUrl;
  contributorAbout.value = question.value?.contributor?.about;
}, { deep: true });

</script>