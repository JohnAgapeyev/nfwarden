use axum::{routing::get, Router};
use tokio;
use tower_http::services::{ServeDir, ServeFile};

async fn run_server() {
    let serve_dir = ServeDir::new("dist");
    let serve_file = ServeFile::new("dist/index.html");

    let app = Router::new()
        .route("/api/", get(|| async { "Hello, World!" }))
        .fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    run_server().await;
}
