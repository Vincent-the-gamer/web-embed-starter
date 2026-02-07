use axum::{Json, routing::post};
use axum_embed::ServeEmbed;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(RustEmbed, Clone)]
#[folder = "webui/dist/"]
struct Assets;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    let serve_assets = ServeEmbed::<Assets>::new();
    let app = axum::Router::new()
        .fallback_service(serve_assets)
        .route("/ping", post(ping));
    println!("Server started!");
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Message {
    code: u32,
    message: String,
}

#[axum::debug_handler]
async fn ping(Json(payload): Json<Message>) -> Json<Message> {
    Json(payload)
}
