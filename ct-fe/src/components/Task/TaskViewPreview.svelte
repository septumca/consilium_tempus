<script lang="ts">
  import type { Task } from "../../lib/types/cttypes.type";
  import TaskViewPreviewBase from "./TaskViewPreviewBase.svelte";

  export let data: Task;
  export let onDelete: (_id: string) => Promise<void> = async (_: string) => {};

  const handleDelete = () => {
    onDelete(data._id);
  }
</script>

<TaskViewPreviewBase data={data}>
  <div>
    <button on:click={handleDelete}>🗑️ Delete</button>
  </div>
</TaskViewPreviewBase>
