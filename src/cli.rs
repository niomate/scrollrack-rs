use clap::Parser;

use std::error;
use std::io::Write;

use itertools::Itertools;

use crate::output::gen_outfile_from_infile;
use crate::parse::parse_card_infos;
use crate::query_stuff::query_and_merge_all;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(short, long)]
    path: String,
}

pub fn cli() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    let cards_by_set = parse_card_infos(&args.path).map(query_and_merge_all)?;

    let mut outfile = gen_outfile_from_infile(&args.path);

    cards_by_set
        .keys()
        .sorted_by_key(|setinfo| setinfo.set_name())
        .for_each(|k| {
            outfile
                .write_all(format!("{}:\n", k.set_name()).as_bytes())
                .unwrap();
            for card in &cards_by_set[k] {
                outfile
                    .write_all(format!("\t- {}\n", card.name()).as_bytes())
                    .unwrap();
            }
            outfile.write_all(b"\n").unwrap();
        });

    Ok(())
}
