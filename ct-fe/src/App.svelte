<script lang="ts">
  import { onMount } from "svelte";
  import Login from "./components/Login/Login.svelte";
  import Logout from "./components/Logout/Logout.svelte";
  import type { RefData, Task } from "./lib/types/cttypes.type";
  import type { User } from "./lib/types/cttypes.type";
  import { readAllTasks, readAllUsers, readRefData } from "./lib/services/api";
  import userStore from "./lib/stores/user";
  import taskStore from "./lib/stores/task";
  import refDataStore from "./lib/stores/refdata";
  import { Link, Route, Router } from "svelte-navigator";
  import PrivateRoute from "./components/General/PrivateRoute.svelte";
  import TaskList from "./components/Task/TaskList.svelte";
  import NewTask from "./components/Task/NewTask.svelte";

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

<main>
  <Router>
    <div>
      <Route path="login">
        <Login />
      </Route>

      <PrivateRoute path="/*">
        <div class="navbar">
          <nav>
            <Link to="/">Home</Link>
            <Link to="tasks">Tasks</Link>
            <Link to="users">Users</Link>
          </nav>
          <Logout />
        </div>
        <Route path="/">
          HOME
        </Route>
        <Route path="tasks">
          <TaskList />
          <Link to="new">New task</Link>
        </Route>
        <Route path="tasks/new">
          <NewTask />
        </Route>
        <Route path="users/*">
          USERS
          <Link to="new">New user</Link>
        </Route>
        <Route path="users/new">
          NEW USER
        </Route>
      </PrivateRoute>

    </div>
  </Router>
</main>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Roboto:wght@400;500&display=swap');

  :root {
    font-family: 'Roboto', sans-serif;
  }

  .navbar {
    border-bottom: 0.2rem solid rgb(0, 134, 187);
    padding-bottom: 0.5rem;
    margin-bottom: 0.5rem;
  }

  :global(input) {
    padding: 0.5rem;
  }

  :global(textarea) {
    padding: 0.5rem;
  }

  :global(button) {
    font-family: 'Roboto', sans-serif;
    font-size: 1rem;
    cursor: pointer;
    background-color: rgb(0, 134, 187);
    transition: background-color 0.3s;
    border: none;
    color: azure;
    padding: 0.5rem;
  }

  :global(button:disabled) {
    background-color: rgba(0, 134, 187, 0.5);
    transition: background-color 0.3s;
  }

  :global(button:hover:not(:disabled)) {
    background-color: rgb(0, 164, 230);
    transition: background-color 0.3s;
  }

</style>
