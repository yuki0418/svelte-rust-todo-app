<script lang="ts">
	import { enhance } from '$app/forms';

	type Props = {
		addAction: string;
	};

	let { addAction }: Props = $props();
</script>

<form method="POST" action={addAction} use:enhance>
	<div class="input-field">
		<input type="text" name="title" />
		<button>Add</button>
	</div>
</form>

<style>
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
</style>
