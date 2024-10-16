<template>
  <div v-if="text && text.trim()" class="w-full text-start px-4 pb-4 mb-4 bg-slate-100 rounded-md hidden md-rendered" v-html="renderedHtml"></div>
</template>


<script setup lang="ts">
import { ref, watchEffect } from "vue";
// import { Writr } from 'writr';
import { marked } from 'marked';

const props = defineProps<{
  text: string
}>()

const renderedHtml = ref("");

// https://www.npmjs.com/package/writr?activeTab=readme
// watchEffect(async () => {
//   const writr = new Writr(props.text, {
//     renderOptions: {
//       emoji: false,
//       toc: false,
//       slug: false,
//       highlight: false,
//       gfm: true,
//       math: false,
//       mdx: false,
//       caching: false,
//     }
//   });
//   renderedHtml.value = await writr.render();
// });

// https://www.npmjs.com/package/marked
watchEffect(async () => {
  marked.use({
    async: true,
    breaks: true,
  });
  renderedHtml.value = await marked.parse(props.text);
});

</script>
