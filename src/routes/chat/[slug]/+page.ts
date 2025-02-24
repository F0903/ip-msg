import { getCorrespondence } from "$lib/api/messages";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
  const to_uuid = params.slug;
  console.log("loading chat for uuid: " + to_uuid);
  const messages = await getCorrespondence(to_uuid);

  return {
    toUuid: to_uuid,
    messages: messages,
  };
};
