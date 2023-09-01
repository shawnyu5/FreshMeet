import Meetup from "~/components/Meetup";

export default function () {
  return <Meetup query={["hackathon"]} per_page={30} />;
}
