use serde_json::Value;

pub fn parse_to_json(file_name: &String) -> Value {
    match std::fs::read_to_string(file_name) {
        Ok(file) => serde_json::from_str(file.as_str()).unwrap(),
        Err(msg) => panic!("{:?}", msg),
    }
}
