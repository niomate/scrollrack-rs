use clap::Parser;

use scrollrack_core::output;
use scrollrack_core::parse::parse_card_infos;
use scrollrack_core::query_stuff::query_and_merge_all;
use std::error;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(short, long)]
    path: String,
}

pub fn cli() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    let cards_by_set = parse_card_infos(&args.path).map(query_and_merge_all)?;
    let outfile = output::gen_outfile_name(&args.path);
    output::write_to_file::<output::SortByName>(cards_by_set, &outfile)
}
