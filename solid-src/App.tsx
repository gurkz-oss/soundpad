import { HomePage } from "./routes";
import { ColorModeProvider, ColorModeScript } from "@kobalte/core";

import "./App.css";
import { Suspense } from "solid-js";

export default function App() {
  return (
    <>
      <Suspense>
        <ColorModeScript />
        <ColorModeProvider>
          <HomePage />
        </ColorModeProvider>
      </Suspense>
    </>
  );
}
