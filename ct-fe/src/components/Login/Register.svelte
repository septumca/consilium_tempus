<script lang="ts">
  import { register } from "../../lib/services/api";
  import { useNavigate } from "svelte-navigator";
  import sha256 from 'crypto-js/sha256';
  import SignInBase from "./SignInBase.svelte";

  const navigate = useNavigate();

  const handleRegister = async (user, password) => {
    await register({ name: user, password: sha256(password).toString() });

    navigate('/login', { replace: true });
  }
</script>

<SignInBase title="Register" buttonText="💡 Register" onClick={handleRegister} />
