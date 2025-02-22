import { getCorrespondence } from "$lib/api/messages";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
  const to_uuid = params.slug;
  const messages = await getCorrespondence(to_uuid);
  console.log(messages);

  return {
    to_uuid: to_uuid,
    messages: messages,
  };
};
