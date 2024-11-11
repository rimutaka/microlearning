<template>
  <a :href="href" class="p-button p-component p-button-secondary whitespace-nowrap p-button-sm" :aria-label="label">
    <span v-if="icon" class="p-button-icon p-button-icon-left" :class="icon"></span>
    <span class="p-button-label">{{ label }}</span>
  </a>
</template>

<script setup lang="ts">

const props = defineProps<{
  href: string,
  label: string,
  icon?: string,
}>()

</script>
