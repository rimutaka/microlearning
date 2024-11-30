<template>
  <div>
    <h1 class="text-balance mb-4 text-slate-700 dark:text-slate-200">Thank you for your support!</h1>
    <p class="text-justify md:text-center text-slate-700 dark:text-slate-200">As a small token of appreciation for helping us all become a little bit more competent we add</p>
    <ul class="" id="signup-pitch">
      <li>
        <i class="pi pi-user"></i>a permanent attribution note
      </li>
      <li>
        <i class="pi pi-desktop"></i>shown on webpages and in emails
      </li>
      <li>
        <i class="pi pi-heart"></i>to say thank you
      </li>
      <li>
        <i class="pi pi-sparkles"></i>for sponsoring the question
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { useMainStore } from '@/store';

const store = useMainStore();

</script>
