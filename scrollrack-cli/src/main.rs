mod cli;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    cli::cli()
}
