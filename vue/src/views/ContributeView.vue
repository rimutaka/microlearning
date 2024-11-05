<template>

  <div class="contribute-card">
    <div class="abstract abstract-1">
    </div>
    <h3 class="text-center text-2xl font-bold mt-4 mb-2 text-slate-700">Write a question</h3>
    <p>You will be surprised how much you can learn by writing questions,
      exploring possible answers and making sure the explanations will withstand the scrutiny of your peers.</p>
    <p>Give it a try. Our moderators will help you along the way.</p>
    <p class="cta text-sm">
      <i class="pi pi-pen-to-square me-1"></i><router-link to="/add" class="">Write a draft question</router-link>  
    </p>
  </div>

  <div class="contribute-card">
    <div class="abstract abstract-2">
    </div>
    <h3 class="text-center text-2xl font-bold mt-4 mb-2 text-slate-700">Give feedback</h3>
    <p>Did you come across a bug or have a suggestion?</p>
     <p><a href="mailto:max@onebro.me">Email the developer</a> with your ideas, 
      correction or whatever other feedback you may have.</p>
    <p>Consider making a <a href="https://github.com/rimutaka/bitesized">pull request</a>.</p>
  </div>

  <div class="contribute-card">
    <div class="abstract abstract-3">
    </div>
    <h3 class="text-center text-2xl font-bold mt-4 mb-2 text-slate-700">Spread the word</h3>
    <p>Share your favorite questions on social media or forward our emails to colleagues
      to challenge them with a bite-sized piece of fun and knowledge.</p>
    <p>
      Questions are the new funny cat videos.
    </p>
    <p class="cta">
      <a href="https://www.linkedin.com/feed/" target="_blank"><i class="pi pi-linkedin"></i></a>
      <a href="https://www.reddit.com/submit?type=LINK" target="_blank"><i class="pi pi-reddit"></i></a>
      <a href="https://twitter.com/" target="_blank"><i class="pi pi-twitter"></i></a>
      <a href="https://www.facebook.com/" target="_blank"><i class="pi pi-facebook"></i></a>
      <a href="https://www.instagram.com/" target="_blank"><i class="pi pi-instagram"></i></a>
      <a href="https://www.tiktok.com" target="_blank"><i class="pi pi-tiktok"></i></a>
    </p>
  </div>

</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';

import Card from 'primevue/card';
import Button from 'primevue/button';

const route = useRoute();


</script>