<script>
	let { children } = $props();
</script>

<main>
	<h1>My Todo App</h1>

	{@render children()}
</main>

<style>
	:global(html *) {
		box-sizing: border-box;
	}

	main {
		font-family: Arial, sans-serif;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		height: 100vh;
	}
</style>
