<script>
  import { isUserLogged } from "../../lib/services/auth";
  import { useNavigate, useLocation } from "svelte-navigator";

  const navigate = useNavigate();
  const location = useLocation();

  const userLogged = isUserLogged();

  $: if (userLogged === false) {
    navigate("/login", {
      state: { from: $location.pathname },
      replace: true,
    });
  }
</script>

{#if userLogged}
  <slot />
{/if}