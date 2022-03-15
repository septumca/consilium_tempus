<script lang="ts">
  import taskStore from "../../lib/stores/task";
  import { useFocus } from "svelte-navigator";
  import TaskView from "./TaskView.svelte";

  const registerFocus = useFocus();
</script>

<div use:registerFocus class="container">
  {#each $taskStore as t}
    <TaskView data={t} />
  {/each}
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: 1fr;
    margin: 0px 8px;
    column-gap: 16px;
    row-gap: 16px;
  }

  @media (min-width: 40em) {
    .container {
      grid-template-columns: 1fr 1fr;
    }
  }

  @media (min-width: 64em) {
    .container {
      grid-template-columns: 1fr 1fr 1fr;
    }
  }
</style>
