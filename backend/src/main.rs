use routes::create_app;

mod handlers;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = create_app();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind to address");
    axum::serve(listener, app).await.expect("server failed");
}
