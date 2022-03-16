<script lang="ts">
  import { useFocus } from "svelte-navigator";
  import type { Task } from "src/lib/types/cttypes.type";
  import TaskViewDraggable from "./TaskViewDraggable.svelte";

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
    <div>
      <div><span>{c.name}</span></div>
      {#each tasksPerColumn[c.code] as t}
        <div>
          <TaskViewDraggable data={t} />
        </div>
      {/each}
    </div>
  {/each}
</div>

<style>
  .container {
    display: grid;
    grid-template-rows: 1fr;
    margin: 0rem 0.5rem;
    column-gap: 1rem;
    row-gap: 1rem;
  }
</style>
