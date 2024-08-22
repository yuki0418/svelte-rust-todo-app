<script lang="ts">
  import type { Todo } from "$lib";

  type Props = {
    todo: Todo;
  };

  let { todo }: Props = $props();
</script>

<div>
  <h2>{todo.title}</h2>
</div>
