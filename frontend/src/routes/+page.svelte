<script lang="ts">
  import TodoCard from "$lib/components/TodoCard.svelte";

  const { data } = $props();
</script>

<h1>My Todos</h1>
<div class="input-field">
  <input type="text" />
  <button>Add</button>
</div>
<div class="container">
  {#each data.todos as todo}
    <TodoCard {todo} />
  {/each}
</div>

<style>
  :global(html *) {
    box-sizing: border-box;
  }

  .input-field {
    width: 500px;
    display: flex;
    gap: 0.4rem;
    margin-bottom: 1rem;

    input {
      width: 100%;
      padding: 0.5rem;
    }
    button {
      padding: 0.5rem 1rem;
    }
  }

  .container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    width: 500px;
  }
</style>
