mod config;
mod error;
mod handler;
mod model;
mod repo;

use crate::repo::{PostRepo, SqlitePostRepo};

use axum::{Router, routing::get};

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};

use std::str::FromStr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

pub fn app<R: PostRepo>(repo: R) -> Router {
    Router::new()
        .route("/", get(handler::hello))
        .route(
            "/posts/{id}",
            get(handler::get_post::<R>)
                .delete(handler::delete_post::<R>)
                .patch(handler::update_post::<R>),
        )
        .route(
            "/posts",
            get(handler::get_all_post::<R>).post(handler::create_post::<R>),
        )
        .with_state(repo)
        .layer(TraceLayer::new_for_http())
}

// Tokio provides the asynchronous runtime.
#[tokio::main]
async fn main() {
    let cfg = config::config();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let address = format!("{}:{}", cfg.host, cfg.port);

    let options = SqliteConnectOptions::from_str(&cfg.database_url)
        .expect("Invalid database URL")
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("Failed to connect to SQLite");

    // Run database migrations
    sqlx::migrate!("./db/migrations")
        .run(&pool)
        .await
        .expect("Migration failed");

    // Initialize repository
    let repo = SqlitePostRepo { pool: pool.clone() };

    // Initialize application
    let app = app(repo);

    // Start HTTP server
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("Failed to bind address");

    tracing::info!(%address, "Server listening");

    axum::serve(listener, app).await.expect("Server failed");
}
