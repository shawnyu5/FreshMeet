import Meetup, { searchEvents } from "~/components/Meetup";

export default function () {
  return <Meetup query={["hackathon"]} per_page={30} />;
}

/**
 * @returns the number of hackathons events on meetup
 */
export async function count() {
  let results = await searchEvents("hackathon", 30, "");
  return results.nodes.length;
}
