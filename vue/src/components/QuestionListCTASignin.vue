<template>
  <div>
    <h3 class="mb-4 text-slate-700 dark:text-slate-300 w-full">For registered users only</h3>

    <div class="flex w-full">
      <div class="flex-shrink md:flex-grow">
      </div>

      <div class="card flex-grow md:flex-shrink mb-8 text-slate-700 dark:text-slate-200">
        <p class="mb-8">Signed-in users can track their progress and avoid getting the same question more than once.</p>

        <ul class="signup-pitch mx-auto mb-8">
          <li><i class="pi pi-lock"></i>your data is not shared with anyone</li>
          <li><i class="pi pi-chart-scatter"></i>only you can see your progress</li>
          <li><i class="pi pi-face-smile"></i>it's free</li>
        </ul>

        <div class="text-center mx-auto">
          <Button label="Sign in" aria-label="Sign in with Google or Github" icon="pi pi-sign-in" raised rounded class="font-bold px-8 py-4 md:me-4 mb-2 whitespace-nowrap" @click="signin()" />
          <p class="text-xs text-center md:mb-auto text-slate-500 dark:text-slate-200" aria-hidden="true">No registration is required - sign in with <i class="pi pi-github ms-1 me-2  dark:opacity-85"></i><i class="pi pi-google me-2  dark:opacity-85"></i>
          </p>
        </div>

      </div>
      <div class="flex-shrink md:flex-grow">
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import Button from 'primevue/button';
import { useAuth0 } from '@auth0/auth0-vue';
import router from "@/router";

// Auth0 login - this is a duplication
// TODO: move this to a shared file, but that requires making Auth0 working outside the component 
const { isAuthenticated, loginWithRedirect } = useAuth0();
const signin = () => {
  if (!isAuthenticated.value) {
    console.log("Not authenticated. Will redirect to ", router.currentRoute.value.fullPath);
    loginWithRedirect({
      appState: { target: router.currentRoute.value.fullPath }
    });
  } else {
    console.log("Already authenticated");
  }
}

</script>