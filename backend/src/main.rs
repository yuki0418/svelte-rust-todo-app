use routes::create_app;
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // build our application with a single route
    let app = create_app(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind to address");
    axum::serve(listener, app).await.expect("server failed");
}
