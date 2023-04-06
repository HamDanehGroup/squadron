use serde_json::Value;

use crate::types::Folder;

pub fn parse_postman_requests(parsed: &Value) -> Vec<Folder> {
    dbg!(parsed);
    vec![]
}
