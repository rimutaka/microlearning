<template>
  <div class="w-full mb-4">
    <Menubar :model="items">
      <template #start>
        <img alt="Bite-sized Learning Logo" class="h-12 w-12" src="@/assets/logo.svg" />
      </template>
      <template #item="{ item, props, hasSubmenu }">
        <router-link v-if="item.route" v-slot="{ href, navigate }" :to="item.route" custom>
          <a v-ripple :href="href" v-bind="props.action" @click="navigate">
            <span :class="item.icon" />
            <span class="ml-2">{{ item.label }}</span>
          </a>
        </router-link>
        <a v-else v-ripple :href="item.url" :target="item.target" v-bind="props.action">
          <span :class="item.icon" />
          <span class="ml-2">{{ item.label }}</span>
          <span v-if="hasSubmenu" class="pi pi-fw pi-angle-down ml-2" />
        </a>
      </template>
      <template #end>
        <div class="flex items-center gap-2">
          <InputText placeholder="Search" type="text" class="w-32 sm:w-auto" />
          <Avatar image="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png" shape="circle" />
        </div>
      </template>
    </Menubar>
  </div>
</template>

<script setup>
import { ref } from "vue";

import Menubar from 'primevue/menubar';

const items = ref([
  {
    label: 'Home',
    icon: 'pi pi-home',
    route: '/'
  },
  {
    label: 'About',
    icon: 'pi pi-star',
    route: '/about'
  },
  {
    label: 'Contact',
    icon: 'pi pi-envelope',
    badge: 3,
    url: 'mailto:max@onebro.me'
  }
]);
</script>
