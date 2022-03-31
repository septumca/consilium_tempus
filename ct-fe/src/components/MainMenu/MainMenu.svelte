<script lang="ts">
  import type { RefData, Task, User } from "src/lib/types/cttypes.type";
  import { readAllTasks, readAllUsers, readRefData } from "../../lib/services/api";
  import userStore from "../../lib/stores/user";
  import taskStore from "../../lib/stores/task";
  import refDataStore from "../../lib/stores/refdata";
  import { onMount } from "svelte";
  import { Link } from "svelte-navigator";
  import Logout from "../Logout/Logout.svelte";

  onMount(async () => {
    const ft: Promise<Array<Task>> = readAllTasks();
    const fu: Promise<Array<User>> = readAllUsers();
    const fr: Promise<RefData> = readRefData();

    const [t, u, r] = await Promise.all([ft, fu, fr]);
    taskStore.set(t);
    userStore.set(u);
    refDataStore.set(r);
  });
</script>


<div class="navbar">
  <nav>
    <Link to="/">Home</Link>
    <Link to="tasks">Tasks</Link>
  </nav>
  <Logout />
</div>

<style>
  .navbar {
    border-bottom: 0.2rem solid rgb(0, 134, 187);
    padding-bottom: 0.5rem;
    margin-bottom: 0.5rem;
  }
</style>