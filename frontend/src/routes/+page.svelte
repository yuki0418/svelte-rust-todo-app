<script lang="ts">
  import TodoCard from "$lib/components/TodoCard.svelte";
  import { enhance } from "$app/forms";

  const { data } = $props();

  let keyword = $state("");

  let todoList = $derived.by(() => {
    let list = data.todos.sort((a, b) =>
      a.completed === b.completed ? 0 : a.completed ? 1 : -1
    );

    if (!keyword) return list;

    list = data.todos.filter((todo) => todo.title.includes(keyword));

    return list;
  });
</script>

<h1>My Todos</h1>

<div class="input-field">
  <input type="text" placeholder="Search for keyword" bind:value={keyword} />
</div>

<form method="POST" action="?/add" use:enhance>
  <div class="input-field">
    <input type="text" name="title" />
    <button>Add</button>
  </div>
</form>

<div class="container">
  {#each todoList as todo}
    <TodoCard {todo} action="?/complete" />
  {/each}
</div>

<style>
  :global(html *) {
    box-sizing: border-box;
  }

  :global(body) {
    font-family: Arial, sans-serif;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 100vh;
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
