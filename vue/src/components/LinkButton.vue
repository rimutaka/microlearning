<template>
  <!-- For .p-button- styles to work the page must contain at least one proper Button from 'primevue/button' -->
  <!-- These two types of links are identical except that one goes via the router.
   Keep their markup identical - changes to one should be copied to the other -->
  <router-link v-if="viaRouter" :to="href" class="p-button p-component whitespace-nowrap p-button-sm" :class="{ 'p-button-secondary': !props.primary }" :aria-label="label">
    <span v-if="icon" class="p-button-icon p-button-icon-left" :class="icon"></span>
    <span class="p-button-label">{{ label }}</span>
  </router-link>
  <a v-else :href="href" class="p-button p-component whitespace-nowrap p-button-sm" :class="{ 'p-button-secondary': !props.primary }" :aria-label="label">
    <span v-if="icon" class="p-button-icon p-button-icon-left" :class="icon"></span>
    <span class="p-button-label">{{ label }}</span>
  </a>
</template>

<script setup lang="ts">

const props = defineProps<{
  href: string,
  viaRouter?: boolean,
  label: string,
  icon?: string,
  primary?: boolean,
}>()

// viaRouter is a crutch because the router reloads the entire page for some links and routes for others
// ideally, it needs to be removed or some kind of heuristic is used to determine if it's an external or internal link
// e.g. if it starts with http

</script>
