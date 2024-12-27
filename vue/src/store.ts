import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { CONTRIBUTOR_DETAILS_LS_KEY, ANY_TOPIC } from './constants'
import type { Question, QuestionWithHistory, User, ContributorProfile, LoadingStatus as LoadingStatusType } from './interfaces'
import { LoadingStatus } from './interfaces'
import { fetchQuestion } from './data-loaders/fetch-question'

/// The main store for the application
export const useMainStore = defineStore('main', () => {
  /** The last loaded question */
  const question = ref<Question | undefined>()

  /** The last loaded list of questions */
  const questionsWithHistory = ref<QuestionWithHistory[] | undefined>()

  /// All topics selected by the user
  const selectedTopics = ref<string[]>([])

  /// Either a random topic or the last selected topic
  /// The component decides which one to use
  /// Used to show a new question
  const currentTopic = ref<string | undefined>()

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

  const reset = () => {
    question.value = undefined;
    questionsWithHistory.value = undefined;
    selectedTopics.value = [];
    currentTopic.value = undefined;
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
    };

    questionStatus.value = LoadingStatus.Loaded;
  }



  /// The topic always comes from props.topic
  /// The qid comes from props.qid if random is false.
  const loadQuestion = async (paramTopic: string, paramQid?: string) => {

    // make sure nothing is showing if the component is reused
    questionStatus.value = LoadingStatus.Loading;
    question.value = undefined;  // clear the value so that other components know it's being reloaded

    if (!paramTopic) {
      console.error("No topic provided - using any.");
      paramTopic = ANY_TOPIC;
    }

    // preserve and clear the current topic
    const currentTopicBak = currentTopic.value;
    currentTopic.value = undefined;

    const fetchedQuestion = await fetchQuestion(paramTopic, paramQid, token.value);

    if (fetchedQuestion) {
      // success
      question.value = fetchedQuestion;
      currentTopic.value = question.value.topic;
      questionStatus.value = LoadingStatus.Loaded;
    }
    else if (fetchQuestion === null) {
      // no question found
      questionStatus.value = LoadingStatus.NoData;
      currentTopic.value = currentTopicBak;
    }
    else {
      // error
      questionStatus.value = LoadingStatus.Error;
      currentTopic.value = currentTopicBak;
    }
  };

  return {
    question,
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