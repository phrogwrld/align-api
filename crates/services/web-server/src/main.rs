use std::sync::Arc;

use axum::{routing::get, Router};
use clap::Parser;
use dotenvy::dotenv;
use lib_core::config::AppConfig;
use lib_core::error::Result;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();

	let config = Arc::new(AppConfig::parse());

	let app = Router::new().route("/", get(handler));

	let _guard = lib_core::logger::init(config.env);

	let listener = TcpListener::bind(format!("127.0.0.1:{:?}", config.port)).await.unwrap();
	info!("🚀 Server has launched on http://{:?}", listener.local_addr().unwrap());
	axum::serve(listener, app.into_make_service()).await.unwrap();

	println!("Hello, world!");

	Ok(())
}

async fn handler() -> &'static str {
	"Hello, world!"
}
