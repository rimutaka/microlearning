<template>
  <div>
    <div>
      <div class="flex flex-wrap gap-4 mb-4">
        <InputText type="text" v-model="qty" placeholder="How many questions" size="small" class="flex-shrink" />
      </div>
      <InputText type="text" v-model="topics" placeholder="Topics" size="small" class="w-full mb-2" />
    </div>
    <QuestionContributorForm />
    <QuestionContributor />
    <div class="flex-shrink text-end">
      <Button label="Make the payment" icon="pi pi-check" raised class="my-auto whitespace-nowrap" @click="get_checkout_url()" />
    </div>
    <LoadingMessage v-if="loadingStatus == LoadingStatus.Loading" />
    <h3 v-if="loadingStatus == LoadingStatus.Error" class="mt-8 mb-8 text-center text-slate-500 dark:text-slate-200">Sorry, something went wrong. Try again.</h3>
  </div>
</template>


<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import type { LoadingStatus as LoadingStatusType, QuestionDonation } from '@/interfaces'
import { PAYMENTS_HANDLER_URL, SPONSOR_DETAILS_LS_KEY } from "@/constants";
import { LoadingStatus } from "@/interfaces";
import { Sha256 } from '@aws-crypto/sha256-js';
import { toHex } from "uint8array-tools";

import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import QuestionContributor from '@/components/QuestionContributor.vue';
import QuestionContributorForm from '@/components/QuestionContributorForm.vue';
import LoadingMessage from "./LoadingMessage.vue";
import { PageIDs } from "@/router";


const store = useMainStore();
const { question } = storeToRefs(store);

const qty = ref(1);
const topics = ref("any topic");
const loadingStatus = ref<LoadingStatusType>(LoadingStatus.Loaded);

const sponsorDetailsInLS = localStorage.getItem(SPONSOR_DETAILS_LS_KEY);

/// Saves the default contributor details to local storage
const saveDefaultSponsorDetails = () => {
  const sponsorDetails = <QuestionDonation>{
    qty: qty.value,
    topics: topics.value,
    contributor: question.value?.contributor,
    cancelUrl: "", // cancel and success urls are set during the checkout process
    successUrl: "",
  };

  localStorage.setItem(SPONSOR_DETAILS_LS_KEY, JSON.stringify(sponsorDetails));
}

/** Calls a lambda to create a checkout page for this transaction and redirects the user to it */
async function get_checkout_url() {

  loadingStatus.value = LoadingStatus.Loading;

  saveDefaultSponsorDetails();

  // the lambda gets all it needs from the serialized JSON object
  const questionDonation = JSON.stringify(<QuestionDonation>{
    qty: qty.value,
    topics: topics.value,
    contributor: question.value?.contributor, // this struct is set by a sub-component
    cancelUrl: window.location.href,
    successUrl: `${window.location.origin}/${PageIDs.THANKYOU}`,
  });

  console.log(questionDonation);

  // calculate the hash of the request body for x-amz-content-sha256 header
  // as required by CloudFront
  const hash = new Sha256();
  hash.update(questionDonation);
  const bodyHash = toHex(await hash.digest());

  const response = await fetch(`${PAYMENTS_HANDLER_URL}`, {
    method: "POST",
    body: questionDonation,
    headers: {
      "x-amz-content-sha256": bodyHash,
    },
  });

  // a successful response should contain the saved question
  // an error may contain JSON or plain text, depending on where the error occurred
  console.log("Response status: ", response.status);
  if (response.status === 200) {
    try {
      const checkoutUrl = await response.text();
      if (checkoutUrl?.startsWith("https://")) {
        console.log("Redirecting to checkout page: ", checkoutUrl);
        window.location.href = checkoutUrl;
      }

      // it is probably a lambda bug if the status = 200 and the URL is not valid
      console.log("Invalid checkout URL: ", checkoutUrl);
      loadingStatus.value = LoadingStatus.Error;

    } catch (error) {
      console.error(error);
      loadingStatus.value = LoadingStatus.Error;
    }
  } else {
    console.error("Failed to get checkout URL: ", response.status);
    loadingStatus.value = LoadingStatus.Error;
  }
}

</script>