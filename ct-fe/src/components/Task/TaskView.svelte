<script lang="ts">
  import type { Task } from "../../lib/types/cttypes.type";
  import refDataStore from "../../lib/stores/refdata";

  export let data: Task;
  $: status = $refDataStore.statuses.find(s => s.code === data.status)?.name;
</script>

<div class="card">
  <div class="left-controls">
    <div><pre>{data.creator.name}</pre></div>
    <div><pre>{data.created_on}</pre></div>
    <div><pre>{status}</pre></div>
  </div>
  <div class="right-controls">
    {#if data.assignee !== null}
      <pre>{data.assignee.name}</pre>
    {/if}
  </div>
  <div class="title">
    <pre>{data.name}</pre>
  </div>
  <div class="content">
    {#if data.description !== null}
      <pre>{data.description}</pre>
    {/if}
  </div>
</div>

<style>
  .left-controls {
    grid-area: lc;
  }
  .title {
    grid-area: tl;
  }
  .right-controls {
    grid-area: rc;
    text-align: right;
  }
  .content {
    grid-area: ct;
  }

  .card {
    box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2);
    transition: 0.3s;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    border: 2px solid #3a3a3a;
    display: grid;
    row-gap: 8px;
    grid-template-columns: 1fr 1fr;
    grid-template-areas:
      "tl tl"
      "lc rc"
      "ct ct"
      "ct ct"
  }
</style>
