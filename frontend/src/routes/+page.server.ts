import type { Todo } from "$lib";

const API_URL = "http://127.0.0.1:3000";

export const load = async ({ fetch }) => {
  const response = await fetch(`${API_URL}/todos`, {
    method: "GET",
  });

  const data: Todo[] = await response.json();

  return {
    todos: data,
  };
};
