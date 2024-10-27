<template>
  <div class="">
    <Button label="Subscribe now for free" icon="pi pi-envelope" raised rounded class="font-bold px-8 py-4 md:me-4 mb-2 whitespace-nowrap" @click="navigateToSubscription" />
    <p class="text-xs text-start md:mb-auto text-slate-500">One new question per day.<br />Auto-paused if you get a backlog.<br />One-click unsubscribe.</p>
  </div>
</template>


<script setup lang="ts">
import { useRoute } from 'vue-router'
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';
import { URL_PARAM_LIST_SEPARATOR } from "@/constants";
import Button from 'primevue/button';
import router from '@/router';

const route = useRoute();
const store = useMainStore();
const { selectedTopics } = storeToRefs(store);

async function navigateToSubscription() {
  console.log("Subscribing to topics: ", selectedTopics.value);
  const subTopics = selectedTopics.value.map((t) => t).join(URL_PARAM_LIST_SEPARATOR);
  if (subTopics) {
    router.push({ name: 'subscription', query: { topics: subTopics } });
  } else {
    router.push({ name: 'subscription' });
  }
}

</script>
