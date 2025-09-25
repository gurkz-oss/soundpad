<script>
  import MyInstants from "./myinstants.svelte";
  import {
    Dialog,
    DialogTrigger,
    DialogHeader,
    DialogContent,
    DialogTitle,
    DialogDescription,
  } from "$lib/components/ui/dialog/index.js";
  import { buttonVariants } from "../ui/button";

  let dialogOpen = $state(false);

  function closeSongAdderDialog() {
    dialogOpen = false;
  }
</script>

<Dialog bind:open={dialogOpen}>
  <DialogTrigger class={buttonVariants({ variant: "default" })}>
    add a song
  </DialogTrigger>
  <DialogContent>
    <DialogHeader>
      <DialogTitle>what song would you like to add?</DialogTitle>
      <DialogDescription>add from myinstants</DialogDescription>
      <MyInstants {closeSongAdderDialog} />

      <!-- <DialogDescription>add from a file</DialogDescription>
      <FileAdder /> -->
    </DialogHeader>
  </DialogContent>
</Dialog>
