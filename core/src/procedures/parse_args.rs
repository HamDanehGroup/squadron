use crate::types::Args;

pub fn parse_args() -> Args {
    let args = std::env::args().collect::<Vec<String>>();
    let mut args = args.into_iter();
    args.next();

    Args::build(
        &args
            .next()
            .unwrap_or_else(|| panic!("Please Specify File Path.")),
    )
}
