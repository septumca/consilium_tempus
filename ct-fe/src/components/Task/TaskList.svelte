<script lang="ts">
  import { useFocus } from "svelte-navigator";
  import type { Task } from "../../lib/types/cttypes.type";
  import TaskColumn from "./TaskColumn.svelte";

  export let tasks: Array<Task> = [];
  export let columns: Array<{ name: string, code: number }> = [];
  $: tasksPerColumn = columns.reduce((a, c) => {
    return {
      ...a,
      [c.code]: tasks.filter(t => t.status === c.code),
    }
  }, {})

  const registerFocus = useFocus();
</script>

<div use:registerFocus class="container" style="grid-template-columns: repeat({columns.length}, 1fr);">
  {#each columns as c}
    <TaskColumn code={c.code} name={c.name} tasks={tasksPerColumn[c.code]} />
  {/each}
</div>

<style>
  .container {
    display: grid;
    grid-template-rows: 5fr;
    margin: 0rem 0.5rem;
    column-gap: 1rem;
    row-gap: 1rem;
    height: 85vh; /* TODO: use more general grid layout on the components around */
  }
</style>
