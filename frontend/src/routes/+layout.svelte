<script>
	let { children } = $props();
</script>

<h1>My Todo App</h1>

{@render children()}

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
</style>
