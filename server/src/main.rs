use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Error, Json, Router,
};

use serde::{Deserialize, Serialize};

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

async fn entries(Json(payload): Json<EntriesRequest>) -> (StatusCode, Json<EntriesResponse>) {
    if payload.namespace == "eli" {
        println!("heyo!");
    }
    let response = EntriesResponse { entries: 2 };

    return (StatusCode::OK, Json(response));
}

async fn create_entry(Json(payload): Json<CreateEntryRequest>) -> Response {
    if payload.content_type == ContentType::File {
        return (
            StatusCode::NOT_IMPLEMENTED,
            "Content type 'file' is not implemented yet",
        )
            .into_response();
    }

    // println!("{}", payload);

    let response = CreateEntryResponse {
        entry: Entry {
            id: 1,
            content_type: payload.content_type,
        },
    };
    return (StatusCode::CREATED, Json(response)).into_response();
}

#[derive(Deserialize)]
struct EntriesRequest {
    namespace: String,
}

#[derive(Serialize)]
struct EntriesResponse {
    entries: u8,
}

#[derive(Deserialize)]
struct CreateEntryRequest {
    namespace: String,
    content_type: ContentType,
    content: String,
}

#[derive(Serialize)]
struct CreateEntryResponse {
    entry: Entry,
}

#[derive(Deserialize, Serialize, Debug)]
struct Entry {
    id: u64,
    content_type: ContentType,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum ContentType {
    Text,
    File,
}
