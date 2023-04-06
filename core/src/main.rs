use std::cmp::Ordering;

use core::procedures::{parse_args, select_type::select_type};

fn main() {
    let _args = parse_args();
    let file_type = select_type();
    let _requests = match file_type {
        Ok(file_type) if file_type.eq("Postman") => {}
        _ => {}
    };
}
