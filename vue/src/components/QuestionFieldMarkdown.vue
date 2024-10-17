<template>
  <div v-if="text && text.trim()" class="w-full text-start px-4 pb-4 mb-4 bg-slate-100 rounded-md hidden md-rendered">
    <p v-if="props.correct" class="mb-4">Correct.</p>
    <p v-else-if="props.correct === false" class="mb-4">Incorrect.</p>
    <div v-html="renderedHtml"></div>
  </div>
</template>


<script setup lang="ts">
import { ref, watchEffect } from "vue";
// import { Writr } from 'writr';
import { marked } from 'marked';

const props = defineProps<{
  text: string,
  /// true: display `Correct`, false: display `Incorrect`, for explanations only
  /// undefined: do not display correct/incorrect for question and answer fields
  correct: boolean | undefined,
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
