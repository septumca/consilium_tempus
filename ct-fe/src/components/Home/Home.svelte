<script lang="ts">
  import { useFocus } from "svelte-navigator";
  import { deleteTask } from "../../lib/services/api";
  import taskStore, { removeTask } from "../../lib/stores/task";
  import { getLoggedUser } from "../../lib/services/auth";
  import TaskViewStub from "../Task/TaskViewPreview.svelte";

  let user = getLoggedUser();
  $: created_tasks = $taskStore.filter(t => t.creator._id === user._id);
  $: assigned_tasks = $taskStore.filter(t => t.assignee !== null && t.assignee._id === user._id);

  const onDelete = async (id: string) => {
    await deleteTask(id);
    removeTask(id);
  }

  const registerFocus = useFocus();
</script>

<div use:registerFocus>
  <div>Created by me</div>
  <div class="container">
    {#each created_tasks as t}
      <div>
        <TaskViewStub data={t} onDelete={onDelete} />
      </div>
    {/each}
  </div>
  <br />
  <div>Assigned to me</div>
  <div class="container">
    {#each assigned_tasks as t}
      <div>
        <TaskViewStub data={t} onDelete={onDelete} />
      </div>
    {/each}
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    column-gap: 1rem;
    row-gap: 1rem;
  }

  @media (min-width: 40em) {
    .container {
      grid-template-columns: repeat(3, 1fr);
    }
  }

  @media (min-width: 64em) {
    .container {
      grid-template-columns: repeat(6, 1fr);
    }
  }
</style>