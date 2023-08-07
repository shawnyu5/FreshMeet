import { createResource, Suspense } from "solid-js";

async function fetchData(): Promise<string> {
  await new Promise((r) => setTimeout(r, 5000));
  return "hello world";
  // return new Promise((resolve) => {
  //   setTimeout(() => {
  //     resolve("Data fetched from API");
  //   }, 5000); // Simulate a 2-second delay
  // });
}

export default function () {
  const [data] = createResource(fetchData);
  return (
    <Suspense fallback={<p>loading...</p>}>
      <p>{data()}</p>
    </Suspense>
  );
}
