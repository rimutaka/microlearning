<template>
  <Transition name="fade" enter-active-class="duration-1000 ease-out" enter-from-class="transform opacity-0" enter-to-class="opacity-100" leave-active-class="duration-1000 ease-in" leave-from-class="opacity-100" leave-to-class="transform opacity-0">
    <slot></slot>
  </Transition>
</template>

<script setup lang="ts">
import { Transition } from "vue";
</script>