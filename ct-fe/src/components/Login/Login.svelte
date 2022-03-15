<script lang="ts">
  import { useLocation, useNavigate } from "svelte-navigator";
  import userStore from "../../lib/stores/user";
  import { useFocus } from "svelte-navigator";
  import { setLoggedUser } from "../../lib/services/auth";
  import Select from "../General/Select.svelte";

  const registerFocus = useFocus();

  const navigate = useNavigate();
  const location = useLocation();

  $: userList = $userStore.map(({ _id, name }) => ({ label: name, id: _id }));
  let selectedUser: string = "";

  const onLogin = () => {
    const u = $userStore.find(e => e._id === selectedUser) || null;

    if (u === null) {
      return;
    }

    setLoggedUser(u);
    const from = ($location.state && $location.state.from) || "/";
    navigate(from, { replace: true });
  }
</script>

<div class="login">
  <div>
    <div use:registerFocus>Login as</div>
    <div>
      <Select
        id="login-user-list"
        data={userList}
        bind:value={selectedUser}
      />
    </div>
    <button on:click={onLogin} disabled={selectedUser === ""}>
      <div>🔑 Login</div>
    </button>
  </div>
</div>


<style>
  .login {
    text-align: center;
    min-height: 100vh;
    display: grid;
    place-items: center;
  }

  .login > div > div {
    margin-bottom: 1rem;
  }
</style>
