<script lang="ts">
  import TodoCard from "$lib/components/TodoCard.svelte";

  const { data } = $props();
</script>

<h1>My Todos</h1>
<div>
  {#each data.todos as todo}
    <TodoCard {todo} />
  {/each}
</div>
