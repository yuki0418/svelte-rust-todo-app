<script lang="ts">
  import type { Todo } from "$lib";
  import { enhance } from "$app/forms";

  type Props = {
    todo: Todo;
    action: string;
  };

  let { todo, action }: Props = $props();
</script>

<div class="card">
  <h2>{todo.title}</h2>
  <form method="POST" {action} use:enhance>
    <button type="submit">Done</button>
    <input name="id" value={todo.id} hidden />
  </form>
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
  }
</style>
