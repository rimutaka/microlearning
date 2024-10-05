<script setup lang="ts">
defineProps<{
  msg: string
}>()
</script>

<template>
  <div class="greetings">
    <h1 class="green">{{ msg }}</h1>
    <h3>
      Learn new skills in 2-min long email lessons, get certified, get a better job.
    </h3>
  </div>
</template>
