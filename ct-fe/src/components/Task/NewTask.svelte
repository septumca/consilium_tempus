<script lang="ts">
  import { useFocus, useNavigate } from "svelte-navigator";
  import Select from "../General/Select.svelte";
  import { getLoggedUser } from "../../lib/services/auth";
  import type { CreateTask, RefData, User } from "../../lib/types/cttypes.type";
  import userStore from "../../lib/stores/user";
  import { addTask } from "../../lib/stores/task";
  import { createTask } from "../../lib/services/api";

  const registerFocus = useFocus();
  const navigate = useNavigate();
  const creator: User = getLoggedUser();

  $: userList = $userStore.map(({ _id, name }) => ({ label: name, id: _id }));
  let assignedUserId: string = "";
  let name: string = "";
  let description: string = "";

  const handleCreate = async () => {
    let data: CreateTask = { name, creator };
    let assignedUser = $userStore.find(e => e._id === assignedUserId);
    if (description) {
      data.description = description;
    }
    if (assignedUser) {
      data.assignee = assignedUser;
    }

    const r = await createTask(data);
    addTask(r);
    navigate('/tasks');
  }
</script>

<div class="new-task">
  <div>
    <input use:registerFocus placeholder="Task name" bind:value={name} >
  </div>
  <div>
    <textarea
      id="task-description"
      name="task-description"
      placeholder="Task description"
      rows="5"
      cols="60"
      bind:value={description}
    />
  </div>
  <div>
    <span>Asignee</span>
    <Select
      id="task-assignee"
      data={userList}
      bind:value={assignedUserId}
    />
  </div>
  <button on:click={handleCreate} disabled={name === ""}>
    <div>🗓️✔️ Create new task</div>
  </button>
</div>

<style>
  .new-task > div {
    margin-bottom: 1rem;
  }
</style>