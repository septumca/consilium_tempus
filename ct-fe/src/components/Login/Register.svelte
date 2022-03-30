<script lang="ts">
  import { register } from "../../lib/services/api";
  import { useNavigate } from "svelte-navigator";
  import { useFocus } from "svelte-navigator";
  import sha256 from 'crypto-js/sha256';

  const registerFocus = useFocus();

  const navigate = useNavigate();

  let user: string = "";
  let password: string = "";

  const handleRegister = async () => {
    await register({ name: user, password: sha256(password).toString() });

    navigate('/login', { replace: true });
  }
</script>

<div class="login">
  <div>
    <div use:registerFocus>Register</div>
    <div>
      <div>
        Name: <input id="login-user-name" bind:value={user} />
      </div>
      <div>
        Password: <input id="login-user-password" bind:value={password} />
      </div>
    </div>
    <button on:click={handleRegister} disabled={user === "" || password === ""}>
      <div>💡 Register</div>
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
