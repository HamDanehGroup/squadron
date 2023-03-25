use crate::types::Request;
use serde_json::Value;

pub fn parse_items(parsed: &Value) {
    if let Value::Array(items) = &parsed["item"] {
        for (index, item) in items.iter().enumerate() {
            let item = item["item"].as_array().unwrap();
            let head_info = item[0].as_object().unwrap();

            let request = head_info["request"].as_object().unwrap();

            let request = Request::build(
                request["method"].as_str().unwrap(),
                request["url"].as_str().unwrap(),
                &request["header"],
            );
            println!("ITEM_{}.request >>> {:#?}", index, &request);
        }
    }
}
