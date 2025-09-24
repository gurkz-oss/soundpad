import { A } from "@solidjs/router";

export function Nav() {
  return (
    <nav class="pl-2 flex flex-row gap-2">
      <A
        activeClass="underline underline-offset-2 decoration-lime-500"
        end
        href="/"
      >
        home
      </A>
      <A
        activeClass="underline underline-offset-2 decoration-lime-500"
        href="/about"
      >
        about
      </A>
    </nav>
  );
}
