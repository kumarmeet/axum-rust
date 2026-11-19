mod config;
mod error;
mod handler;
mod model;

use std::str::FromStr;

use axum::{Router, routing::get};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

// rust doesn't have built in async runtime, so this is the job of tokio
#[tokio::main] // starts the async runtime
async fn main() {
    let cfg = config::config();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let address = format!("{}:{}", cfg.host, cfg.port);
    let options = SqliteConnectOptions::from_str(&cfg.database_url)
        .unwrap()
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS posts( id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL,body TEXT NOT NULL)",
    )
    .execute(&pool)
    .await
    .unwrap();

    // let db = DB::default();

    let app = Router::new()
        .route("/", get(handler::hello))
        .route(
            "/posts/{id}",
            get(handler::get_post)
                .delete(handler::delete_post)
                .patch(handler::update_post),
        )
        .route(
            "/posts",
            get(handler::get_all_post).post(handler::create_post),
        )
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    tracing::info!(%address, "listening");
    axum::serve(listener, app).await.unwrap();
}
