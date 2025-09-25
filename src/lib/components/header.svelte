<script lang="ts">
  import NavLink from "./nav-link.svelte";
</script>

<nav class="pl-2 flex flex-row gap-2">
  <NavLink
    activeClass="underline underline-offset-2 decoration-lime-500"
    href="/"
  >
    home
  </NavLink>
  <NavLink
    activeClass="underline underline-offset-2 decoration-lime-500"
    href="/about"
  >
    about
  </NavLink>
</nav>
