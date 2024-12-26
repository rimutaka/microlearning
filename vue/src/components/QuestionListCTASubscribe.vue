<template>
  <div>
    <h3 class="mb-4 text-slate-700 dark:text-slate-300 w-full">Subscribe to regular learning in bite-sized portions</h3>
    <ul class="signup-pitch mb-8 ">
      <li><i class="pi pi-list-check"></i>new questions about your topics of interest</li>
      <li><i class="pi pi-envelope"></i>delivered to your inbox</li>
      <li><i class="pi pi-face-smile"></i>for free</li>
      <li><i class="pi pi-sparkles"></i>to help you not fall behind</li>
    </ul>
    <div class="text-center mx-auto">
      <Button label="Choose your topics" icon="pi pi-list-check" raised rounded class="font-bold px-8 py-4 md:me-4 mb-2 whitespace-nowrap" @click="router.push({ name: 'subscription' })" />
    </div>
  </div>
</template>

<script setup lang="ts">
import router from '@/router';
import { PageIDs } from '@/router';
import Button from 'primevue/button';

</script>