/* @refresh reload */
import { render } from "solid-js/web";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { SolidQueryDevtools } from "@tanstack/solid-query-devtools";

import "./App.css";
import App from "./App";
import { createSignal, ParentProps } from "solid-js";

export const [selectedDevice, setSelectedDevice] = createSignal<string>("");

function Root(props: ParentProps) {
  const client = new QueryClient();
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
