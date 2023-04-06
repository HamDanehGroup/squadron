pub struct Args {
    pub file_name: String,
}

impl Args {
    pub fn build(file_name: &str) -> Self {
        Self {
            file_name: String::from(file_name),
        }
    }
}
