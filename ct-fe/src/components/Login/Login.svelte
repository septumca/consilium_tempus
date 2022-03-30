<script lang="ts">
  import { Link, useLocation, useNavigate } from "svelte-navigator";
  import userStore from "../../lib/stores/user";
  import { useFocus } from "svelte-navigator";
  import { setLoggedUser } from "../../lib/services/auth";
  import Select from "../General/Select.svelte";
  import { login } from "../../lib/services/api";
  import sha256 from 'crypto-js/sha256';

  const registerFocus = useFocus();

  const navigate = useNavigate();
  const location = useLocation();

  let user: string = "";
  let password: string = "";

  const handleLogin = async () => {
    const r = await login({
      name: user,
      password: sha256(password).toString()
    });

    setLoggedUser(r);
    const from = ($location.state && $location.state.from) || "/";
    navigate(from, { replace: true });
  }
</script>

<div class="login">
  <div>
    <div use:registerFocus>Login as</div>
    <div>
      <div>
        Name: <input id="login-user-name" bind:value={user} />
      </div>
      <div>
        Password: <input id="login-user-password" bind:value={password} />
      </div>
    </div>
    <button on:click={handleLogin} disabled={user === "" || password === ""}>
      <div>🔑 Login</div>
    </button>
    <div>
      <Link to="/register">Register</Link>
    </div>
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
