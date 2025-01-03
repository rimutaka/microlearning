import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useRoute } from 'vue-router'

import { CONTRIBUTOR_DETAILS_LS_KEY, ANY_TOPIC, URL_PARAM_TOPIC } from './constants'
import type { Question, QuestionWithHistory, User, ContributorProfile, LoadingStatus as LoadingStatusType } from './interfaces'
import { LoadingStatus, PublishStage } from './interfaces'
import { fetchQuestion } from './data-loaders/fetch-question'



/// The main store for the application
export const useMainStore = defineStore('main', () => {

  const route = useRoute();

  /** The last loaded question in HTML format */
  const question = ref<Question | undefined>()

  /** The last loaded question in Markdown format 
   * TODO: replace this with a flag inside Question to indicate the format */
  const questionMD = ref<Question | undefined>()

  /** The last loaded list of questions */
  const questionsWithHistory = ref<QuestionWithHistory[] | undefined>()

  /// All topics selected by the user
  const selectedTopics = ref<string[]>([])

  /// Email from the token from the ID provider, e.g. Google or Github
  const email = ref<string | undefined>()

  /// Raw token from the ID provider, e.g. Google or Github
  const token = ref<string | undefined>()

  /// DDB profile data for the logged in user
  const user = ref<User | undefined>()

  /// Any data in the contributor profile should be ignored if this value is true
  const anonymousContributor = ref<boolean | undefined>()

  /** Changes as per the state of loadQuestion().
   * Check if the question is present.
   */
  const questionStatus = ref<LoadingStatusType | undefined>()

  /** Set by components that load question lists.
 */
  const questionListStatus = ref<LoadingStatusType | undefined>()

  /** Is set to true when the user clicks on Show Random Question,
   then back to false on loading the view that includes the sample question */
  const showingRandomQuestion = ref<boolean>(false)

  /** Topic ID from the URL query parameter */
  const currentTopic = computed((): string | undefined => {
    console.log(`Detected topic change to ${route.query[URL_PARAM_TOPIC]}`);
    return route.query.topic ? <string>route.query[URL_PARAM_TOPIC] : undefined;
  });

  const reset = () => {
    question.value = undefined;
    questionMD.value = undefined;
    questionsWithHistory.value = undefined;
    selectedTopics.value = [];
    showingRandomQuestion.value = false;
    email.value = undefined;
    token.value = undefined;
    user.value = undefined;
    anonymousContributor.value = undefined;
    questionStatus.value = undefined;
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
      title: "",
      stage: PublishStage.Draft,
    };

    questionStatus.value = LoadingStatus.Loaded;
  }



  /** Loads a question from the DB or reuses the existing one if the topic and qid match.
   * Returns a random question for the topic if qid is not provided.
   * Updates the questionStatus value during the loading process.
   * Clear the question value to guarantee the latest version is loaded.
   */
  const loadQuestion = async (paramTopic: string, paramQid?: string) => {

    // check if the currently loaded question can be reused
    if (question.value && question.value.topic === paramTopic && question.value.qid === paramQid) {
      questionStatus.value = LoadingStatus.Loaded;
      console.log("Reusing the current question.");
      return;
    }

    // make sure nothing is showing if the component is reused
    questionStatus.value = LoadingStatus.Loading;
    question.value = undefined;  // clear the value so that other components know it's being reloaded

    if (!paramTopic) {
      console.error("No topic provided - using any.");
      paramTopic = ANY_TOPIC;
    }

    const fetchedQuestion = await fetchQuestion(paramTopic, paramQid, token.value);

    if (fetchedQuestion) {
      // success
      question.value = fetchedQuestion;
      questionStatus.value = LoadingStatus.Loaded;
    }
    else if (fetchQuestion === null) {
      // no question found
      questionStatus.value = LoadingStatus.NoData;
    }
    else {
      // error
      questionStatus.value = LoadingStatus.Error;
    }
  };

  return {
    question,
    questionMD,
    questionsWithHistory,
    selectedTopics,
    currentTopic,
    showingRandomQuestion,
    email,
    token,
    user,
    anonymousContributor,
    questionStatus,
    questionListStatus,
    reset,
    resetQuestionToBlank,
    loadQuestion
  }
})