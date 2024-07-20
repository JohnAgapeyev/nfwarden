use axum::{response::IntoResponse, response::Json, response::Result, routing::get, Router};
use serde_json::{json, Value};
use tokio;
use tower_http::services::{ServeDir, ServeFile};

use nftables::*;

async fn get_tables() -> Result<Json<NftOutput>, Json<String>> {
    let mut ctx = NftCtx::new();
    ctx.set_json();
    ctx.set_dry_run(true);

    let input = NftOutput {
        items: vec![NftObjects::List(NftList {
            tables: NftTableList {
                family: None,
                name: None,
                handle: None,
            },
        })],
    };

    //println!("{}", serde_json::to_string(&input).unwrap());

    //Json(ctx.run_cmd_str(&serde_json::to_string(&input).unwrap()))

    match ctx.run_cmd_str(&serde_json::to_string(&input).unwrap()) {
        Ok(raw) => {
            let parsed = serde_json::from_slice::<NftOutput>(raw.as_bytes()).unwrap();
            //println!("Serialized Input JSON Parsed:\n{parsed:#?}");
            return Ok(Json(parsed));
        }
        Err(err_msg) => {
            return Err(Json(err_msg));
        }
    }
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
