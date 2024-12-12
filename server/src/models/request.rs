use super::entry;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct EntriesRequest {
    pub namespace: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateEntryRequest {
    pub namespace: String,
    pub content_type: entry::ContentType,
    pub content: String,
}
