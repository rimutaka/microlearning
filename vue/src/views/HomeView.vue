<template>
  <TransitionSlot>
    <SignupPitch v-if="!currentTopic" />
  </TransitionSlot>
  <SignupForm />
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import TransitionSlot from "@/components/TransitionSlot.vue";

import SignupForm from '@/components/SignupForm.vue';
import SignupPitch from '@/components/SignupPitch.vue';

const store = useMainStore();
const { currentTopic } = storeToRefs(store);

</script>
