<script setup lang="ts">
import { RouterView } from 'vue-router'
import TopHeader from './components/TopHeader.vue';
import FooterStatic from './components/FooterStatic.vue';
</script>

<template>
  <header class="">
    <TopHeader />
  </header>
  <main class="mb-auto">
    <RouterView />
  </main>
  <footer class="">
    <FooterStatic />
  </footer>
</template>
