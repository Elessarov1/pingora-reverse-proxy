use std::env;
use axum::Router;
use axum::routing::get;

async fn health() -> String {
    format!("Up, answered from 127.0.0.1:{}", env::var("PORT").unwrap())
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "".to_string());
    let addr = format!("127.0.0.1:{}", port);

    let app = Router::new()
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    println!("Server running at {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}