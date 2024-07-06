use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};
use tokio;
use tower_http::services::{ServeDir, ServeFile};

async fn get_tables() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

fn get_api_v1_router() -> Router {
    Router::new().route("/get_tables", get(get_tables))
}

fn get_api_router() -> Router {
    Router::new().nest("/v1", get_api_v1_router())
}

async fn run_server() {
    let serve_dir = ServeDir::new("dist");
    let serve_file = ServeFile::new("dist/index.html");

    let app = Router::new()
        .nest("/api", get_api_router())
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
