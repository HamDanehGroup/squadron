use super::Request;

pub struct Folder {
    pub name: String,
    pub requests: Vec<Request>,
}

impl Folder {
    pub fn build(name: &str, requests: Option<Vec<Request>>) -> Self {
        Self {
            name: String::from(name),
            requests: requests.unwrap_or_else(|| vec![]),
        }
    }

    pub fn add_request(&mut self, request: &Request) {
        self.requests.push(request.clone());
    }
}
