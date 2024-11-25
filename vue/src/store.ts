import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { TOPICS, CONTRIBUTOR_DETAILS_LS_KEY } from './constants'
import type { Question, User, ContributorProfile } from './constants'

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
  const componentKey = ref<string | undefined>()

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
  /// componentKey controls the question component :key to force an update
  function showRandomQuestion() {
    question.value = undefined;
    componentKey.value = Date.now().toString(36); // e.g. 0.cbm9x4v2kyi
  }

  const reset = () => {
    question.value = undefined;
    selectedTopics.value = [];
    currentTopic.value = undefined;
    componentKey.value = undefined;
    email.value = undefined;
    token.value = undefined;
    user.value = undefined;
  }

  /// Reset the question fields to a blank state.
  /// This is a hack and there should be a more elegant solution.
  const resetQuestionToBlank = () => {

    // get the contributor details from the local storage
    const contribLS = localStorage.getItem(CONTRIBUTOR_DETAILS_LS_KEY);
    const contribParsed = contribLS ? JSON.parse(contribLS) : <ContributorProfile>{ name: "" };

    question.value = {
      qid: "",
      topic: "",
      question: "",
      answers: [],
      correct: 0,
      contributor: contribParsed,
    };
  }

  return {
    question,
    selectedTopics,
    lastSelectedTopic,
    currentTopic,
    componentKey,
    email,
    token,
    user,
    reset,
    showRandomQuestion,
    resetQuestionToBlank
  }
})