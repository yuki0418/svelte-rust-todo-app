<script lang="ts">
  import type { Todo } from "$lib";
  import { enhance } from "$app/forms";

  type Props = {
    todo: Todo;
    completeAction: string;
    deleteAction: string;
  };

  let { todo, completeAction, deleteAction }: Props = $props();
</script>

<div class="card">
  <h2>{todo.title}</h2>
  <div class="actions">
    {#if todo.completed}
      ✅
    {:else}
      <form method="POST" action={completeAction} use:enhance>
        <button type="submit">Done</button>
        <input name="id" value={todo.id} hidden />
      </form>
    {/if}
    <form method="POST" action={deleteAction} use:enhance>
      <button type="submit"> Delete </button>
      <input name="id" value={todo.id} hidden />
    </form>
  </div>
</div>

<style>
  .card {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    border-radius: 3px;
    padding: 1rem;
    width: 100%;
    border: 1px solid #ccc;

    h2 {
      margin: 0;
    }

    .actions {
      display: flex;
      gap: 1rem;
    }
  }
</style>
