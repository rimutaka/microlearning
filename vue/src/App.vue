<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'
</script>

<template>
  <header>
    <img alt="Bite-sized Learning Logo" class="p-2 w-24 h-24" src="@/assets/logo.svg" />
<h1>Bite-sized Learning for busy devs</h1>
<h3>Learn new skills in 2-min long email lessons, get certified, get a better job.</h3>
    <div class="wrapper">
      <nav>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/about">About</RouterLink>
      </nav>
    </div>
  </header>

  <RouterView />
</template>
