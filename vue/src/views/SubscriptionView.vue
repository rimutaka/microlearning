<template>
  <TransitionSlot>
    <SubscriptionPitch v-if="formHydrated && (!user || !user?.topics.length)" />
  </TransitionSlot>
  <SubscriptionForm @hydrated="formHydrated = true" />
  <TransitionSlot>
    <SampleQuestion v-if="formHydrated && currentTopic" :topic="currentTopic" :nonce="currentTopicKey" />
  </TransitionSlot>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import TransitionSlot from "@/components/TransitionSlot.vue";

import SubscriptionPitch from '@/components/SubscriptionPitch.vue';
import SubscriptionForm from '@/components/SubscriptionForm.vue';
import SampleQuestion from "@/components/SampleQuestion.vue";

const store = useMainStore();
const { user, currentTopic, currentTopicKey } = storeToRefs(store);
const formHydrated = ref(false);

</script>
