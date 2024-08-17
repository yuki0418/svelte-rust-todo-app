use crate::routes::AppState;

pub struct NewTodo {
    pub title: String,
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
