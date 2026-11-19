mod config;
mod error;
mod handler;
mod model;
use model::DB;

use axum::{Router, routing::get};

// rust doesn't have built in async runtime, so this is the job of tokio
#[tokio::main] // starts the async runtime
async fn main() {
    let cfg = config::config();
    let address = format!("{}:{}", cfg.host, cfg.port);
    let db = DB::default();

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
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("listening on {}", address);
    axum::serve(listener, app).await.unwrap();
}
