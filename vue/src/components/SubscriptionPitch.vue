<template>
  <div>
    <div class="flex">
      <div class="flex-shrink md:flex-grow">
      </div>

      <div class="mt-4 card flex-grow md:flex-shrink">
        <h3 class="text-justify md:text-center text-slate-700">Confirm your subscription</h3>
        <ul id="signup-pitch">
          <li><i class="pi pi-question-circle"></i>one question per email</li>
          <li><i class="pi pi-envelope"></i>sent daily to <strong>{{ store.email }}</strong> </li>
          <li><i class="pi pi-pause-circle"></i>paused automatically if you get a backlog</li>
          <li><i class="pi pi-ban"></i>every email includes an unsubscribe link</li>
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
