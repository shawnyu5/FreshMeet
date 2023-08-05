import Meetup from "~/components/Meetup";

export default function () {
  return <Meetup query={["board games", "hangout"]} per_page={20} />;
}
