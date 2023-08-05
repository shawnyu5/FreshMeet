import Meetup from "~/components/Meetup";

export default function () {
  return (
    <Meetup
      query={["bars", "dating", "singles", "speed dating"]}
      per_page={10}
    />
  );
}
