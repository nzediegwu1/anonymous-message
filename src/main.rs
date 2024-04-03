use axum::{Extension, Router};
use dotenv::dotenv;
use sqlx::{migrate, postgres::PgPoolOptions};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod config;
mod core;
mod modules;
use crate::{config::Config, modules::auth::auth_routes};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::parse();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new().allow_origin(Any);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Unable to connect to database");

    migrate!()
        .run(&pool)
        .await
        .expect("Failed to run auto-migration");

    let app = Router::new()
        .nest("/auth", auth_routes())
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4040));
    tracing::debug!("Server started, listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}
