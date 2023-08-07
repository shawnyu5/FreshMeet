import { Suspense } from "solid-js";
import Meetup from "~/components/Meetup";

export default function () {
  return <Meetup query={["tech", "programming", "coding"]} per_page={10} />;
}
