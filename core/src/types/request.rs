use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Request {
    pub name: String,
    pub parameters: HashMap<String, String>,
    pub method: String,
    pub url: String,
    pub header: Value,
}

impl Request {
    pub fn build(
        name: &str,
        parameters: &HashMap<String, String>,
        method: &str,
        url: &str,
        header: &Value,
    ) -> Self {
        Self {
            name: name.to_owned(),
            parameters: parameters.clone(),
            method: method.to_owned(),
            url: url.to_owned(),
            header: header.clone(),
        }
    }
}
