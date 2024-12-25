import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { CONTRIBUTOR_DETAILS_LS_KEY, TOKEN_HEADER_NAME, RECENT_HEADER_NAME, URL_PARAM_TOPIC, URL_PARAM_QID, QUESTION_HANDLER_URL, ANY_TOPIC } from './constants'
import type { Question, User, ContributorProfile, LoadingStatus as LoadingStatusType } from './interfaces'
import { LoadingStatus } from './interfaces'

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

  /** Is set to true when the user clicks on Show Random Question,
   then back to false on loading the view that includes the sample question */
  const showingRandomQuestion = ref<boolean>(false)

  const reset = () => {
    question.value = undefined;
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

  // Stores the qid in a comma-separated list in the local storage.
  // Removes old entries if the list gets longer than 10 items.
  function storeRecentQuestionsInLS(paramQid: string) {
    const recent = localStorage.getItem(RECENT_HEADER_NAME);
    if (recent) {
      const recentList = recent.split(",");
      if (recentList.includes(paramQid)) {
        // remove the old entry
        recentList.splice(recentList.indexOf(paramQid), 1);
      }
      recentList.unshift(paramQid);
      if (recentList.length > 10) {
        console.log("Removing old entries from recent questions list");
        recentList.pop();
      }
      localStorage.setItem(RECENT_HEADER_NAME, recentList.join(","));
    } else {
      localStorage.setItem(RECENT_HEADER_NAME, paramQid);
    }
  }

  /// The topic always comes from props.topic
  /// The qid comes from props.qid if random is false.
  const loadQuestion = async (paramTopic: string, paramQid?: string) => {

    console.log(`Fetching question for: ${paramTopic} / ${paramQid}`);

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

    // add a token with the email, if there is one (logged in users)
    const headers = new Headers();
    if (token.value) headers.append(TOKEN_HEADER_NAME, token.value);

    // add list of recently viewed questions to the request
    const recent = localStorage.getItem(RECENT_HEADER_NAME);
    if (recent) headers.append(RECENT_HEADER_NAME, recent);

    try {
      // fetching by topic returns a random question
      // fetching with qid returns a specific question
      // fetching any topic has "any" for the topic
      const fetchParams = `${URL_PARAM_TOPIC}=${paramTopic}`.concat(paramQid ? `&${URL_PARAM_QID}=${paramQid}` : "");
      console.log("fetchParams", fetchParams);

      const response = await fetch(`${QUESTION_HANDLER_URL}${fetchParams}`,
        {
          headers: headers,
          signal: AbortSignal.timeout(5000),
        });
      console.log(`Fetched. Status: ${response.status}`);

      // a successful response should contain the saved question
      // an error may contain JSON or plain text, depending on where the errror occurred
      if (response.status === 200) {
        try {
          question.value = <Question>await response.json();
          // console.log(question);
          // console.log(question.topic);
          console.log(`Loaded ${question.value.topic} / ${question.value.qid}`);

          currentTopic.value = question.value.topic;
          questionStatus.value = LoadingStatus.Loaded;
          storeRecentQuestionsInLS(question.value.qid); // add qid to the list of recent questions

        } catch (error) {
          console.error(error);
          currentTopic.value = currentTopicBak;
          console.error("Failed to parse question.");
        }
      }
      else if (response.status === 404) {
        questionStatus.value = LoadingStatus.NoData;
        currentTopic.value = currentTopicBak;
      }
      else {
        questionStatus.value = LoadingStatus.Error;
        currentTopic.value = currentTopicBak;
        console.error("Failed to get question. Status: ", response.status);
      }
    } catch (error) {
      questionStatus.value = LoadingStatus.Error;
      currentTopic.value = currentTopicBak;
      console.error("Failed to get question.");
      console.error(error);
    }
  };

  return {
    question,
    selectedTopics,
    currentTopic,
    showingRandomQuestion,
    email,
    token,
    user,
    anonymousContributor,
    questionStatus,
    reset,
    resetQuestionToBlank,
    loadQuestion
  }
})