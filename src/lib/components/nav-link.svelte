<script lang="ts" module>
  export type Props = {
    href: string;
    class?: ClassValue;
    activeClass?: ClassValue;
    inactiveClass?: ClassValue;
    children: Snippet;
  };
</script>

<script lang="ts">
  import { page } from "$app/state";
  import type { Snippet } from "svelte";
  import type { ClassValue } from "svelte/elements";

  let {
    activeClass,
    href,
    inactiveClass,
    class: clazz,
    children,
  }: Props = $props();
  let isActive = $derived(page.url.pathname === href);
</script>

<a class={[clazz, isActive && activeClass, !isActive && inactiveClass]} {href}>
  {@render children()}
</a>
