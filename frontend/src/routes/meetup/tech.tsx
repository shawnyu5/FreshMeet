import { Suspense } from "solid-js";
import Meetup from "~/components/Meetup";

export default function () {
  return <Meetup query={["tech", "coding", "programming"]} per_page={10} />;
}
