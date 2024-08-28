import type { Todo } from "$lib";
import type { Actions } from "@sveltejs/kit";

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

export const actions = {
  add: async ({ request, fetch }) => {
    const formData = await request.formData();
    const title = formData.get("title");

    await fetch(`${API_URL}/todos`, {
      method: "POST",
      body: JSON.stringify({ title }),
      headers: {
        "Content-Type": "application/json",
      },
    });
  },
  complete: async ({ request, fetch }) => {
    const formData = await request.formData();
    const id = formData.get("id");

    await fetch(`${API_URL}/todos/${id}`, {
      method: "PUT",
    });
  },
  delete: async ({ request, fetch }) => {
    const formData = await request.formData();
    const id = formData.get("id");

    await fetch(`${API_URL}/todos/${id}`, {
      method: "DELETE",
    });
  },
} satisfies Actions;
