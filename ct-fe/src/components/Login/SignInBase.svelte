<script lang="ts">
  import { useFocus } from "svelte-navigator";

  export let onClick: (user: string, password: string) => Promise<void>;
  export let title: string;
  export let buttonText: string;

  let user: string = "";
  let password: string = "";

  const registerFocus = useFocus();

  const handleClick = () => {
    onClick(user, password);
  }
</script>

<div class="sign-in">
  <div>
    <div>{title}</div>
    <div>
      <div>Name:</div>
      <div>
        <input id="sign-in-user-name" use:registerFocus bind:value={user} />
      </div>
      <div>Password:</div>
      <div>
        <input id="sign-in-user-password" bind:value={password} />
      </div>
    </div>
    <button on:click={handleClick} disabled={user === "" || password === ""}>
      <div>{buttonText}</div>
    </button>
    <slot />
  </div>
</div>


<style>
  .sign-in {
    text-align: center;
    min-height: 100vh;
    display: grid;
    place-items: center;
  }

  .sign-in > div > div {
    margin-bottom: 1rem;
  }
</style>
