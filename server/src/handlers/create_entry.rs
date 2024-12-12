use crate::models::{entry, request, response};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub async fn create_entry(Json(payload): Json<request::CreateEntryRequest>) -> Response {
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
