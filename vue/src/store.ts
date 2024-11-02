import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { TOPICS } from './constants'
import type { Question, User } from './constants'

/// The main store for the application
export const useMainStore = defineStore('main', () => {
  /// Currently loaded question
  const question = ref<Question | undefined>()

  /// All topics selected by the user
  const selectedTopics = ref<string[]>([])

  /// Either a random topic or the last selected topic
  /// The component decides which one to use
  /// Used to show a new question
  const currentTopic = ref<string | undefined>()

  /// A random string to force the question update if the topic does not change
  const currentTopicKey = ref<string | undefined>()

  /// Email from the token from the ID provider, e.g. Google or Github
  const email = ref<string | undefined>()

  /// Raw token from the ID provider, e.g. Google or Github
  const token = ref<string | undefined>()

  /// DDB profile data for the logged in user
  const user = ref<User | undefined>()

  /// Returns the last selected topic or a random one if none are selected
  const lastSelectedTopic = computed(() => {
    if (selectedTopics.value.length) {
      return selectedTopics.value[selectedTopics.value.length - 1];
    } else {
      return undefined;
    }
  });

  /// Show a random question from the selected topics or all topics
  function showRandomQuestion() {
    if (selectedTopics.value.length) {
      currentTopic.value = selectedTopics.value[Math.floor(Math.random() * selectedTopics.value.length)];
    } else {
      currentTopic.value = TOPICS[Math.floor(Math.random() * TOPICS.length)].id;
    }
    currentTopicKey.value = Date.now().toString(36); // e.g. 0.cbm9x4v2kyi
  }

  const reset = () => {
    question.value = undefined;
    selectedTopics.value = [];
    currentTopic.value = undefined;
    currentTopicKey.value = undefined;
    email.value = undefined;
    token.value = undefined;
    user.value = undefined;
  }

  return {
    question,
    selectedTopics,
    lastSelectedTopic,
    currentTopic,
    currentTopicKey,
    email,
    token,
    user,
    reset,
    showRandomQuestion
  }
})