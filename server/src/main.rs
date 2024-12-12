use axum::{routing::post, Router};

use handlers::{create_entry::create_entry, list_entries::list_entries};

mod handlers;
mod models;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "7878";
    let endpoint: String = HOST.to_owned() + ":" + PORT;

    let app = Router::new()
        .route("/entries", post(list_entries))
        .route("/create_entry", post(create_entry));
    let listener = tokio::net::TcpListener::bind(endpoint).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Rust server is listening on port {PORT}");
}
