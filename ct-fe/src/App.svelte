<script lang="ts">
  import Login from "./components/Login/Login.svelte";
  import taskStore from "./lib/stores/task";
  import refDataStore from "./lib/stores/refdata";
  import { Link, Route, Router } from "svelte-navigator";
  import PrivateRoute from "./components/General/PrivateRoute.svelte";
  import TaskList from "./components/Task/TaskList.svelte";
  import NewTask from "./components/Task/NewTask.svelte";
  import Home from "./components/Home/Home.svelte";
  import Register from "./components/Login/Register.svelte";
  import MainMenu from "./components/MainMenu/MainMenu.svelte";
</script>

<main>
  <Router>
    <div>
      <Route path="register">
        <Register />
      </Route>

      <Route path="login">
        <Login />
      </Route>

      <PrivateRoute path="/*">
        <MainMenu />
        <Route path="/">
          <Home />
        </Route>
        <Route path="tasks">
          <TaskList tasks={$taskStore} columns={$refDataStore.statuses} />
          <Link to="new">New task</Link>
        </Route>
        <Route path="tasks/new">
          <NewTask />
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
