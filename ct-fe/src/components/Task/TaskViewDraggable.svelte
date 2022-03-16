<script lang="ts">
  import type { Task } from "../../lib/types/cttypes.type";
  import TaskPreviewBase from "./TaskViewPreviewBase.svelte";

  export let data: Task;
</script>

<TaskPreviewBase data={data} />

