

<script lang="ts">
  import { clearLoggedUser } from "../../lib/services/auth";
  import { useNavigate } from "svelte-navigator";

  const navigate = useNavigate();

  const onLogout = () => {
    clearLoggedUser();
    navigate("/login", { replace: true });
  }
</script>

<div>
  <button on:click={onLogout}>
    <div>🚪 Logout</div>
  </button>
</div>
