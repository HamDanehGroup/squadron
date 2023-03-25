use std::collections::HashMap;

use serde_json::Value;

pub fn get_info(parsed: &Value) {
    println!(
        "\nName of Schema : {:#?}",
        parsed["info"]["name"].as_str().unwrap()
    );
}
