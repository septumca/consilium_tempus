<script lang="ts">
  import { Link, useLocation, useNavigate } from "svelte-navigator";
  import { setLoggedUser } from "../../lib/services/auth";
  import { login } from "../../lib/services/api";
  import sha256 from 'crypto-js/sha256';
  import SignInBase from "./SignInBase.svelte";

  const navigate = useNavigate();
  const location = useLocation();

  let loginError = false;

  const handleLogin = async (user, password) => {
    loginError = false;
    const { status, data } = await login({
      name: user,
      password: sha256(password).toString()
    });

    if(status !== 200) {
      loginError = true;
      return;
    }

    setLoggedUser(data);
    const from = ($location.state && $location.state.from) || "/";
    navigate(from, { replace: true });
  }
</script>

<SignInBase title="Login as" buttonText="🔑 Login" onClick={handleLogin}>
  {#if loginError}
    <div class="err-msg">
      Error loggin in - incorrect name or password
    </div>
  {/if}
  <div>
    <Link to="/register">Register</Link>
  </div>
</SignInBase>

<style>
  .err-msg {
    color: red;
    font-weight: 300;
  }
</style>