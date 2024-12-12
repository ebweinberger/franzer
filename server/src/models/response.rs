use super::entry;
use serde::Serialize;

#[derive(Serialize)]
pub struct EntriesResponse {
    pub entries: u8,
}

#[derive(Serialize)]
pub struct CreateEntryResponse {
    pub entry: entry::Entry,
}
