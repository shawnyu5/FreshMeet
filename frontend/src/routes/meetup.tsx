import Meetup from "~/components/Meetup";

export default function () {
  return <Meetup query={["tech", "programming"]} per_page={10} />;
}
