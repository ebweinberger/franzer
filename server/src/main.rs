use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};

use models::{entry, request, response};

mod models;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "7878";
    let endpoint: String = HOST.to_owned() + ":" + PORT;

    let app = Router::new()
        .route("/entries", post(entries))
        .route("/create_entry", post(create_entry));
    let listener = tokio::net::TcpListener::bind(endpoint).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Rust server is listening on port {PORT}");
}

async fn entries(Json(payload): Json<request::EntriesRequest>) -> Response {
    if payload.namespace == "eli" {
        println!("heyo!");
    }
    let response = response::EntriesResponse { entries: 3 };

    return (StatusCode::OK, Json(response)).into_response();
}

async fn create_entry(Json(payload): Json<request::CreateEntryRequest>) -> Response {
    if payload.content_type == entry::ContentType::File {
        return (
            StatusCode::NOT_IMPLEMENTED,
            "Content type 'file' is not implemented",
        )
            .into_response();
    }

    println!("{}", serde_json::to_string_pretty(&payload).unwrap());

    let response = response::CreateEntryResponse {
        entry: entry::Entry {
            id: 1,
            content_type: payload.content_type,
        },
    };
    return (StatusCode::CREATED, Json(response)).into_response();
}
