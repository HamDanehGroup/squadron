use clap::Parser;
use squadron::{get_info, parse_items, read_file_and_parse_to_json};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let parsed = read_file_and_parse_to_json(&args.filename);

    get_info(&parsed);
    parse_items(&parsed);
}
