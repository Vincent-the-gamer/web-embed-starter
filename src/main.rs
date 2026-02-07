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
