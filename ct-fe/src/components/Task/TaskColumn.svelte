<script lang="ts">
  import { updateTask } from "../../lib/services/api";
  import { updateTaskStatus } from "../../lib/stores/task";

  import type { Task } from "../../lib/types/cttypes.type";
  import TaskViewDraggable from "./TaskViewDraggable.svelte";

  export let tasks: Array<Task> = [];
  export let name: String = "";
  export let code: number;

  const handleDragDrop = async (e) => {
    e.preventDefault();
    const taskId = e.dataTransfer.getData("task-id");
    await updateTask(taskId, { status: code });
    updateTaskStatus(taskId, code);
  }

</script>

<div
  on:drop={handleDragDrop}
  on:dragover={(ev) => { ev.preventDefault() }}
>
  <div><span>{name}</span></div>
  {#each tasks as t}
    <div on:drop={handleDragDrop} >
      <TaskViewDraggable data={t} />
    </div>
  {/each}
</div>

<style>
  div {
    background-color: rgb(235, 250, 253);
  }
</style>