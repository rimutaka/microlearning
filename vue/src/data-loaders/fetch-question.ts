import { QUESTION_HANDLER_URL, TOKEN_HEADER_NAME, URL_PARAM_TOPIC, ANY_TOPIC, RECENT_HEADER_NAME, URL_PARAM_QID } from "@/constants";
import { type Question } from "@/interfaces";


/** Makes the best effort to fetch a single question. Returns `undefined` on error. */
export const fetchQuestion = async (topic: string, qid?: string, token?: string): Promise<Question | undefined | null> => {

  console.log(`Fetching question for: ${topic} / ${qid}`);

  if (!topic) {
    console.error("No topic provided. It's a bug. Exiting.");
    return undefined;
  }

  // add a token with the email, if there is one (logged in users)
  const headers = new Headers();
  if (token) headers.append(TOKEN_HEADER_NAME, token);

  // add list of recently viewed questions to the request
  const recent = localStorage.getItem(RECENT_HEADER_NAME);
  if (recent) headers.append(RECENT_HEADER_NAME, recent);

  try {
    // fetching by topic returns a random question
    // fetching with qid returns a specific question
    // fetching any topic has "any" for the topic
    const fetchParams = `${URL_PARAM_TOPIC}=${topic}`.concat(qid ? `&${URL_PARAM_QID}=${qid}` : "");
    console.log("fetch params", fetchParams);

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
        const question = <Question>await response.json();
        // console.log(question);
        console.log(`Loaded ${question.topic} / ${question.qid}`);
        storeRecentQuestionsInLS(question.qid); // add qid to the list of recent questions
        return question;
      } catch (error) {
        console.error("Failed to parse question.", error);
      }
    }
    else if (response.status === 404) {
      return null;
    }
    else {
      console.error("Failed to get question. Status: ", response.status);
    }
  } catch (error) {
    console.error("Failed to fetch question.", error);
  }
};

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