pub mod logger;
pub mod ping;

use crate::ping::ping;
use axum::routing::post;
use axum_embed::ServeEmbed;
use rust_embed::RustEmbed;
use tokio::net::TcpListener;

#[derive(RustEmbed, Clone)]
#[folder = "webui/dist/"]
struct Assets;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Log
    logger::init_logger();

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    let serve_assets = ServeEmbed::<Assets>::new();
    let app = axum::Router::new()
        .fallback_service(serve_assets)
        .route("/api/ping", post(ping));
    println!("Server started!");
    axum::serve(listener, app).await?;
    Ok(())
}
