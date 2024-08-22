<script lang="ts">
  import type { Todo } from "$lib";

  type Props = {
    todo: Todo;
  };

  let { todo }: Props = $props();
</script>

<div class="card">
  <h2>{todo.title}</h2>
</div>

<style>
  .card {
    border-radius: 0.5rem;
    padding: 1rem;
    width: 100%;
    border: 1px solid #ccc;

    h2 {
      margin: 0;
    }
  }
</style>
