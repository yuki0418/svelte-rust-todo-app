<script lang="ts">
	import AddField from '$lib/components/AddField.svelte';
	import TodoCard from '$lib/components/TodoCard.svelte';

	let { data } = $props();

	let keyword = $state('');

	const todoList = $derived.by(() => {
		let list = data.todos.sort((a, b) => (a.completed === b.completed ? 0 : a.completed ? 1 : -1));

		if (!keyword) return list;

		list = data.todos.filter((todo) => todo.title.includes(keyword));

		return list;
	});
</script>

<div class="input-field">
	<input type="text" placeholder="Search for keyword" bind:value={keyword} />
</div>

<AddField addAction="?/add" />

<div class="container">
	{#each todoList as todo}
		<TodoCard {todo} completeAction="?/complete" deleteAction="?/delete" />
	{/each}
</div>

<style>
	.container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		width: 500px;
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
	}
</style>
