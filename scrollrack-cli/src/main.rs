use clap::ArgEnum;
use clap::Parser;

use scrollrack_core::output;
use scrollrack_core::parse;
use scrollrack_core::query_stuff::CardQuery;

use anyhow::Result;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum Ordering {
    ALPHA,
    DATE,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(help = "Path to the card list")]
    path: String,
    #[clap(short='O', long, arg_enum, default_value_t=Ordering::ALPHA, help="Specifies in which order the sets should be printed in the output file")]
    ordering: Ordering,
    #[clap(short, long, help = "Output sets per card instead of cards per set")]
    inverted: bool,
    #[clap(short, long, help = "Path to the output file")]
    output: Option<String>,
    #[clap(long, help = "Include promo sets in the output")]
    include_promos: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let lines = parse::read_lines(&args.path)?;
    let cards_by_set = CardQuery::with_options(args.include_promos, args.inverted)
        .query(parse::parse_card_infos(lines));

    let outfile = match args.output {
        Some(path) => path,
        None => output::gen_outfile_name(&args.path),
    };

    let out_string = match args.ordering {
        Ordering::ALPHA => output::output_string::<output::SortByName>(cards_by_set),
        Ordering::DATE => output::output_string::<output::SortByDate>(cards_by_set),
    };

    output::write_to_file(&out_string, &outfile)
}
