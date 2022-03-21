<script lang="ts">
  import type { Task } from "../../lib/types/cttypes.type";
  import TaskPreviewBase from "./TaskViewPreviewBase.svelte";

  export let data: Task;

  const handleDragStart = (e) => {
    e.dataTransfer.dropEffect = "move";
    e.dataTransfer.setData("task-id", data._id);
  }
</script>

<div
  draggable=true
  on:dragstart={handleDragStart}
>
  <TaskPreviewBase data={data} />
</div>


