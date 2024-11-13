<template>
  <TransitionSlot>
    <SubscriptionCTA v-if="formHydrated && (!user || !user?.topics.length)" />
  </TransitionSlot>
  <SubscriptionForm @hydrated="formHydrated = true" />
  <TransitionSlot>
    <SampleQuestion v-if="formHydrated && componentKey" :nonce="componentKey" />
  </TransitionSlot>
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import TransitionSlot from "@/components/TransitionSlot.vue";

import SubscriptionCTA from '@/components/SubscriptionCTA.vue';
import SubscriptionForm from '@/components/SubscriptionForm.vue';
import SampleQuestion from "@/components/SampleQuestion.vue";

const store = useMainStore();
const { user, componentKey } = storeToRefs(store);
const formHydrated = ref(false);

watchEffect(() => {
  // reset the sample question key to make sure it is not showing
  // when the user transitions from another page
  componentKey.value = undefined;
});

</script>
