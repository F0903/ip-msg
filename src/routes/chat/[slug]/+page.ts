import { getCorrespondence } from "$lib/api/messages";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
  const toId = parseInt(params.slug);
  const messages = await getCorrespondence(toId);
  console.log(`loading chat page for id = ${toId}`);
  console.log(messages);

  return {
    toId: toId,
    messages: messages,
  };
};
