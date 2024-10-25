<template>
  <div class="w-full mb-4 flex gap-8 justify-center border-t pt-2 mt-12">
    <template v-for="item in menuItems" :key="item.label">
      <router-link v-if="item.route" v-slot="{ href, navigate }" :to="item.route" custom>
        <a :href="href" @click="navigate">
          <span :class="item.icon" />
          <span class="ml-2">{{ item.label }}</span>
        </a>
      </router-link>
      <a v-else :href="item.url">
        <span :class="item.icon" />
        <span class="ml-2">{{ item.label }}</span>
      </a>
    </template>
    <a v-if="isAuthenticated" @click="signout">
      <span class="pi pi-sign-out" />
      <span class="ml-2">Logout</span>
    </a>
    <a v-else @click="signin">
      <span class="pi pi-sign-in" />
      <span class="ml-2">Login</span>
    </a>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import router from "@/router";
import { useAuth0 } from '@auth0/auth0-vue';
import { LAST_AUTH_TIMESTAMP } from "@/constants";

const { isAuthenticated, loginWithRedirect, logout } = useAuth0();

const menuItems = ref([
  {
    label: 'Home',
    icon: 'pi pi-home',
    route: '/'
  },
  {
    label: 'Add',
    icon: 'pi pi-question-circle',
    route: '/add'
  },
  {
    label: 'About',
    icon: 'pi pi-info-circle',
    route: '/about'
  },
  {
    label: 'Contact',
    icon: 'pi pi-envelope',
    badge: 3,
    url: 'mailto:max@onebro.me'
  }
]);

/// Auth0 login
function signin() {
  if (!isAuthenticated.value) {
    loginWithRedirect();
  } else {
    console.log("Already authenticated");
  }
}

/// Auth0 logout
function signout() {
  if (isAuthenticated.value) {
    localStorage.removeItem(LAST_AUTH_TIMESTAMP);
    logout({
      openUrl(url) {
        console.log("Redirecting to logout: ", url);
        router.push("/");
      }
    });
    console.log("Signed out. LAST_AUTH_TIMESTAMP deleted.");

  } else {
    console.log("Already signed out");
  }
}

</script>