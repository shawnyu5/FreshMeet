// @refresh reload
import { Suspense } from "solid-js";
import {
  A,
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
          <ErrorBoundary>
            <A href="/meetup/tech">Tech</A>
            <A href="/meetup/bars">Bars</A>
            <A href="/meetup/board-games">Board games / hangout</A>
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
