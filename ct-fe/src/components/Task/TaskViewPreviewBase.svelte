<script lang="ts">
  import type { Task } from "../../lib/types/cttypes.type";

  export let data: Task;
  $: slotsCount = Object.keys($$slots).length;
</script>

<div class="card" style="grid-template-rows: repeat({slotsCount+2}, 1fr);">
  <div>
    <span>📔 {data.name}</span>
  </div>
  <slot></slot>
  <div>
    {#if data.assignee !== null}
      <span>💪 {data.assignee.name}</span>
    {/if}
  </div>
</div>

<style>
  .card {
    box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2);
    transition: 0.3s;
    padding: 0.5rem 1rem;
    border-radius: 0.25rem;
    border: 2px solid #3a3a3a;
    display: grid;
    row-gap: 0.25rem;
    align-items: center;
    grid-template-columns: 1fr;
    grid-template-rows: repeat(3, 1fr);
  }

  .card > div {
    height: 2rem;
    line-height: 2rem;
  }
</style>
