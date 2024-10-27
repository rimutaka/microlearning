<template>
  <TransitionSlot>
    <HomePitch v-if="!currentTopic" />
  </TransitionSlot>
  <HomeForm />
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useMainStore } from '@/store';

import TransitionSlot from "@/components/TransitionSlot.vue";

import HomeForm from '@/components/HomeForm.vue';
import HomePitch from '@/components/HomePitch.vue';

const store = useMainStore();
const { currentTopic } = storeToRefs(store);

</script>
