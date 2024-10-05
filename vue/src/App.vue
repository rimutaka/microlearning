<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import TopMenu from './components/TopMenu.vue'
import TopHeader from './components/TopHeader.vue';
</script>

<template>
  <header>
    <TopMenu />
    <TopHeader />
  </header>

  <RouterView />
</template>
