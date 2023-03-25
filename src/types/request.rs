use serde_json::Value;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub header: Value,
    pub url: String,
}

impl Request {
    pub fn build(method: &str, url: &str, header: &Value) -> Self {
        Self {
            method: String::from(method),
            url: String::from(url),
            header: header.clone(),
        }
    }
}
