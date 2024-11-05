<template>

  <div class="contribute-card">
    <div class="abstract">
      <i class="pi pi-question-circle text-green-300 opacity-30"></i>
    </div>
    <h3>Write a question</h3>
    <p>You will be surprised how much you can learn by writing questions,
      exploring possible answers and making sure the explanations will withstand the scrutiny of your peers.</p>
    <p>Give it a try. Our moderators will help you along the way.</p>
    <p class="cta text-sm">
      <i class="pi pi-pen-to-square me-1"></i><router-link to="/add" class="">Write a draft question</router-link>
    </p>
  </div>

  <div class="contribute-card">
    <div class="abstract abstract-2">
      <i class="pi pi-comments text-yellow-500 opacity-20"></i>
    </div>
    <h3>Give feedback</h3>
    <p>Did you come across a bug or have a suggestion?</p>
    <p>Consider making a <a href="https://github.com/rimutaka/bitesized">pull request</a> or <a href="mailto:max@onebro.me">email the developer</a> with your ideas,
      corrections or whatever other feedback you may have.</p>
  </div>

  <div class="contribute-card">
    <div class="abstract abstract-3">
      <i class="pi pi-megaphone text-red-600 opacity-20" style="transform: rotate(-12deg);"></i>
    </div>
    <h3>Spread the word</h3>
    <p>Share your favourite questions on social media or forward our emails to colleagues
      to challenge them with a bite-sized piece of fun and knowledge.</p>
    <p>
      Questions are the new funny cat videos.
    </p>
    <p class="cta">
      <a href="https://www.linkedin.com/feed/" target="_blank"><i class="pi pi-linkedin text-sky-300"></i></a>
      <a href="https://www.reddit.com/submit?type=LINK" target="_blank"><i class="pi pi-reddit text-red-300"></i></a>
      <a href="https://twitter.com/" target="_blank"><i class="pi pi-twitter text-gray-500"></i></a>
      <a href="https://www.facebook.com/" target="_blank"><i class="pi pi-facebook text-blue-300"></i></a>
      <a href="https://www.instagram.com/" target="_blank"><i class="pi pi-instagram" style="color: #ff006999"></i></a>
      <a href="https://www.tiktok.com" target="_blank"><i class="pi pi-tiktok text-teal-300"></i></a>
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