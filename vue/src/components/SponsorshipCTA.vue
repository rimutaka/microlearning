<template>
  <div>
    <div class="flex">
      <div class="flex-shrink md:flex-grow">
      </div>

      <div class="card flex-grow md:flex-shrink">
        <h3 class="text-justify md:text-center text-slate-700 dark:text-slate-200">Question sponsorship</h3>
        <ul id="signup-pitch">
          <li><i class="pi pi-question-circle"></i>a permanent attribution record</li>
          <li><i class="pi pi-envelope"></i>shown on webpages and in emails</li>
          <li><i class="pi pi-pause-circle"></i>to say thanks</li>
          <li><i class="pi pi-ban"></i>for making us all more competent</li>
        </ul>

      </div>

      <div class="flex-shrink md:flex-grow">
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMainStore } from '@/store';

const store = useMainStore();

</script>
