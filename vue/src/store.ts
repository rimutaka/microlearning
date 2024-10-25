import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Question } from './constants'

/// The main store for the application
export const useMainStore = defineStore('main', () => {
  const question = ref<Question | undefined>()
  const selectedTopics = ref<string[]>([])
  const email = ref<string | undefined>()

  return { question, selectedTopics, email }
})