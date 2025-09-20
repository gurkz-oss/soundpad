/* @refresh reload */
import { render } from "solid-js/web";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { SolidQueryDevtools } from "@tanstack/solid-query-devtools";

import "./App.css";
import App from "./App";
import { createSignal, onMount, ParentProps } from "solid-js";
import { checkForAppUpdates } from "./update";

export const [selectedDevice, setSelectedDevice] = createSignal<string>("");

function Root(props: ParentProps) {
  const client = new QueryClient();

  onMount(async () => {
    await checkForAppUpdates(false);
  });

  return (
    <QueryClientProvider client={client}>
      {props.children}
      <SolidQueryDevtools />
    </QueryClientProvider>
  );
}

render(
  () => (
    <Root>
      <App />
    </Root>
  ),
  document.getElementById("root") as HTMLElement
);
