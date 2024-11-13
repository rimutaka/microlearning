<template>
  <div class="card flex-grow md:flex-shrink cta-box">
    <h3 class="text-justify md:text-center text-slate-700">About your subscription</h3>
    <ul id="signup-pitch">
      <li><i class="pi pi-envelope"></i>You will get one email per day on <strong>{{ store.email }}</strong> with a random question about one of your selected topics.</li>
      <li><i class="pi pi-clock"></i>The next email is scheduled for the following day when you answer the previous question.</li>
      <li><i class="pi pi-pause-circle"></i>Emails are automatically paused if you stop answering questions to minimize your backlog.</li>
      <li><i class="pi pi-ban"></i>You can change or remove your topics on this page and every email has a one-click unsubscribe link.</li>
    </ul>
    <p class="mt-8">Thank you for subscribing and happy learning!</p>
    <div class="flex items-start justify-center mt-2 mb-4 space-x-3">
      <img class="hidden sm:block w-6 h-6 rounded-full" src="https://avatars.githubusercontent.com/u/5926028?v=4" alt="profile picture">
      <div class="flex items-center divide-x-2 divide-slate-500 text-slate-500">
        <div class="pe-3 font-medium">Max @rimutaka</div>
        <div class="px-3 text-sm font-light">creator of this app</div>
        <div class="ps-3">
          <a href="https://github.com/rimutaka" class="text-slate-500 me-2"><i class="pi pi-github ms-1"></i></a>
          <a href="https://www.linkedin.com/in/maxvoskob" class="text-slate-500 me-2"><i class="pi pi-linkedin"></i></a>
          <a href="mailto:max@bitesized.info?subject=Feedback about Bite-sized" class="text-slate-500 me-2"><i class="pi pi-envelope"></i></a>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMainStore } from '@/store';

const store = useMainStore();

</script>
