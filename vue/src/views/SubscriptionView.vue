<template>
  <TransitionSlot>
    <SubscriptionPitch v-if="user && !user.topics.length" />
  </TransitionSlot>
  <SubscriptionForm />
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import TransitionSlot from "@/components/TransitionSlot.vue";

import SubscriptionPitch from '@/components/SubscriptionPitch.vue';
import SubscriptionForm from '@/components/SubscriptionForm.vue';

const store = useMainStore();
const { user } = storeToRefs(store);

</script>
