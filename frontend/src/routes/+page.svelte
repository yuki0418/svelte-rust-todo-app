<script lang="ts">
  import TodoCard from "$lib/components/TodoCard.svelte";

  const { data } = $props();
</script>

<h1>My Todos</h1>
<div class="container">
  {#each data.todos as todo}
    <TodoCard {todo} />
  {/each}
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 500px;
  }
</style>
