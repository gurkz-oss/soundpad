/* @refresh reload */
import { render } from "solid-js/web";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { SolidQueryDevtools } from "@tanstack/solid-query-devtools";
import { Router, Route } from "@solidjs/router";

import "./App.css";
import { createSignal, lazy, onMount, ParentProps, Suspense } from "solid-js";
import { checkForAppUpdates } from "./update";
import { ColorModeScript, ColorModeProvider } from "@kobalte/core/color-mode";
import { Nav } from "./components/nav";

const Home = lazy(() =>
  import("@/routes/index").then((mod) => ({ default: mod.HomePage }))
);
const About = lazy(() =>
  import("@/routes/about").then((mod) => ({ default: mod.About }))
);

function Root(props: ParentProps) {
  const client = new QueryClient();

  onMount(async () => {
    await checkForAppUpdates(false);
  });

  return (
    <Suspense>
      <QueryClientProvider client={client}>
        <ColorModeScript />
        <ColorModeProvider>
          <Nav />
          <main class="p-2">{props.children}</main>
        </ColorModeProvider>
        <SolidQueryDevtools />
      </QueryClientProvider>
    </Suspense>
  );
}

render(
  () => (
    <Router root={Root}>
      <Route path="/" component={Home} />
      <Route path="/about" component={About} />
    </Router>
  ),
  document.getElementById("root") as HTMLElement
);
