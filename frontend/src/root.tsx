// @refresh reload
import { Suspense } from "solid-js";
import {
  Body,
  ErrorBoundary,
  FileRoutes,
  Head,
  Html,
  Meta,
  Routes,
  Scripts,
  Title,
} from "solid-start";
import { Navbar } from "./components/Navbar";
import "./root.css";

export default function Root() {
  // <link
  //   rel="stylesheet"
  //   href="https://cdn.jsdelivr.net/npm/foundation-sites@6.7.5/dist/css/foundation.min.css"
  //   crossorigin="anonymous"
  // ></link>

  // <script
  //   src="https://cdn.jsdelivr.net/npm/foundation-sites@6.7.5/dist/js/foundation.min.js"
  //   crossorigin="anonymous"
  // ></script>
  return (
    <Html lang="en">
      <Head>
        <Title>SolidStart - Bare</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <link
          rel="stylesheet"
          href="https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css"
        />
      </Head>
      <Body>
        <Suspense>
          <ErrorBoundary fallback={(err) => err}>
            <Navbar
              labels={[
                { label: "Tech", path: "/meetup/tech" },
                { label: "Bars", path: "/meetup/bars" },
                { label: "Board games + hangout", path: "/meetup/board-games" },
              ]}
            />
            <Routes>
              <FileRoutes />
            </Routes>
          </ErrorBoundary>
        </Suspense>
        <Scripts />
      </Body>
    </Html>
  );
}
