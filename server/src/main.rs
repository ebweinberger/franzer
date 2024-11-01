use axum::{http::StatusCode, routing::post, Json, Router};

use serde::{Deserialize, Serialize};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "7878";
    let endpoint: String = HOST.to_owned() + ":" + PORT;

    let app = Router::new().route("/entries", post(entries));
    let listener = tokio::net::TcpListener::bind(endpoint).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Rust server is listening on port {PORT}");
}

async fn entries(Json(payload): Json<EntriesRequest>) -> (StatusCode, Json<EntriesResponse>) {
    if payload.namespace == "eli" {
        println!("heyo!");
    }
    let response = EntriesResponse { entries: 2 };

    return (StatusCode::OK, Json(response));
}

#[derive(Deserialize)]
struct EntriesRequest {
    namespace: String,
}

#[derive(Serialize)]
struct EntriesResponse {
    entries: u8,
}
