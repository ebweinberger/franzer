use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
    pub id: u64,
    pub content_type: ContentType,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Text,
    File,
}
