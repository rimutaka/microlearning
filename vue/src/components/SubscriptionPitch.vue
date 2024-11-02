<template>
  <div>
    <div class="flex">
      <div class="flex-shrink md:flex-grow">
      </div>

      <div class="mt-4 card flex-grow md:flex-shrink">
        <h3 class="text-justify md:text-center text-slate-700">Your free subscription to being 1% better every day:</h3>
        <ul id="signup-pitch">
          <li><i class="pi pi-question-circle"></i>one question with detailed answers</li>
          <li><i class="pi pi-envelope"></i>emailed daily to <strong>{{ store.email }}</strong> </li>
          <li><i class="pi pi-pause-circle"></i>auto-paused if you get a backlog of questions</li>
          <li><i class="pi pi-ban"></i>one-click unsubscribe</li>
        </ul>

        <div class="my-12 mx-auto text-center">
          <figure class="max-w-screen-md mx-auto">
            <svg class="h-8 mx-auto mb-3 text-slate-400" viewBox="0 0 24 27" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M14.017 18L14.017 10.609C14.017 4.905 17.748 1.039 23 0L23.995 2.151C21.563 3.068 20 5.789 20 8H24V18H14.017ZM0 18V10.609C0 4.905 3.748 1.038 9 0L9.996 2.151C7.563 3.068 6 5.789 6 8H9.983L9.983 18L0 18Z" fill="currentColor" />
            </svg>
            <blockquote>
              <p class="text-l font-medium text-slate-900 max-w-md mx-auto">"I became much more productive after a few weeks of investing 5 minutes per day into learning something new."</p>
            </blockquote>
            <figcaption class="flex items-center justify-center mt-6 space-x-3">
              <img class="hidden sm:block w-6 h-6 rounded-full" src="https://avatars.githubusercontent.com/u/5926028?v=4" alt="profile picture">
              <div class="flex items-center divide-x-2 divide-slate-500 text-slate-500">
                <div class="pe-3 font-medium">Max @rimutaka</div>
                <div class="px-3 text-sm font-light">creator of this app</div>
                <div class="ps-3">
                  <a href="https://github.com/rimutaka" class="text-slate-500 me-2"><i class="pi pi-github ms-1"></i></a>
                  <a href="https://www.linkedin.com/in/maxvoskob" class="text-slate-500 me-2"><i class="pi pi-linkedin"></i></a>
                  <a href="mailto:max@onebro.me?subject=Feedback about Bite-sized" class="text-slate-500 me-2"><i class="pi pi-envelope"></i></a>
                </div>
              </div>
            </figcaption>
          </figure>
        </div>
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
