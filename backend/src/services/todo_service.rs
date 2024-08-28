use crate::routes::AppState;

pub struct NewTodo {
    pub title: String,
}

#[derive(serde::Serialize)]
pub struct Todo {
    id: uuid::Uuid,
    title: String,
    completed: bool,
}

pub async fn create(state: AppState, new_todo: NewTodo) -> Result<(), sqlx::Error> {
    let pool = state.pool;

    sqlx::query!(
        r#"
          INSERT INTO todos (title, completed)
          VALUES ($1, false)
        "#,
        new_todo.title
    )
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn get_list(state: AppState) -> Result<Vec<Todo>, sqlx::Error> {
    let pool = state.pool;

    let todo_list = sqlx::query_as!(
        Todo,
        r#"
          SELECT id, title, completed
          FROM todos
        "#
    )
    .fetch_all(&pool)
    .await?;

    Ok(todo_list)
}

pub async fn complete(state: AppState, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    let pool = state.pool;

    sqlx::query!(
        r#"
          UPDATE todos
          SET completed = true
          WHERE id = $1
        "#,
        id
    )
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn archive(state: AppState, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    let pool = state.pool;

    sqlx::query!(
        r#"
            DELETE FROM todos
            WHERE id = $1
        "#,
        id
    )
    .execute(&pool)
    .await?;

    Ok(())
}
