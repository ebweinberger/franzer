use crate::models::{request, response};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub async fn list_entries(Json(payload): Json<request::EntriesRequest>) -> Response {
    if payload.namespace == "eli" {
        println!("heyo!");
    }
    let response = response::EntriesResponse { entries: 3 };

    return (StatusCode::OK, Json(response)).into_response();
}
