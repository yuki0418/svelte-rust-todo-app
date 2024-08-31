<script lang="ts">
	import type { Todo } from '$lib/types';
	import { applyAction, enhance } from '$app/forms';

	type Props = {
		todo: Todo;
		completeAction: string;
		deleteAction: string;
	};

	let { todo, completeAction, deleteAction }: Props = $props();
</script>

<div class="card">
	<h2>{todo.title}</h2>
	<form method="POST" use:enhance>
		<input name="id" value={todo.id} hidden />
		{#if todo.completed}
			✅
		{:else}
			<button type="submit" formaction={completeAction}>Done</button>
		{/if}
		<button type="submit" formaction={deleteAction}> Delete </button>
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
