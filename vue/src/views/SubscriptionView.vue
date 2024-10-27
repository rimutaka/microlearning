<template>
  <SubscriptionPitch/>
  <SubscriptionForm />
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

// import TransitionSlot from "@/components/TransitionSlot.vue";

import SubscriptionPitch from '@/components/SubscriptionPitch.vue';
import SubscriptionForm from '@/components/SubscriptionForm.vue';

const store = useMainStore();
// const { currentTopic } = storeToRefs(store);

</script>
