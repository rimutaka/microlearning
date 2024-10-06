<template>
  <div class="card mt-12">
    <h3>Topics</h3>
    <div class="flex flex-wrap items-center gap-4 justify-center my-4">
      <div class="flex" v-for="topic in topics" :key="topic">
        <Checkbox v-model="selectedTopics" :inputId="topic" name="topics" :value="topic" />
        <label :for="topic" class="ms-2 me-4">{{ topic }}</label>
      </div>
    </div>
    <div class="flex flex-wrap items-center gap-4 justify-center my-8">
      <InputText type="text" v-model="email" placeholder="Your email" />
      <Button label="Subscribe" icon="pi pi-discord" raised rounded class="font-bold px-8 py-4 whitespace-nowrap" />
    </div>
  </div>
</template>


<script setup lang="ts">
import { ref } from "vue";
import Button from 'primevue/button';
import Checkbox from 'primevue/checkbox';
import InputText from 'primevue/inputtext';


const topics = ref(["AWS", "CSS", "General", "JS / TS", "Rust"]);
const selectedTopics = ref([]);

const email = ref("");

</script>
