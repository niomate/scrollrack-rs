use clap::ArgEnum;
use clap::Parser;

use scrollrack_core::output;
use scrollrack_core::parse::parse_card_infos;
use scrollrack_core::query_stuff::query_and_merge_all;

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
    #[clap(long, help = "Exclude promo sets from the output")]
    no_promos: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let cards_by_set = parse_card_infos(&args.path)
        .map(|c| query_and_merge_all(c, args.no_promos))
        .map_err(|err| format!("Error: {}", err))?;

    let outfile = match args.output {
        Some(path) => path,
        None => output::gen_outfile_name(&args.path),
    };

    match args.ordering {
        Ordering::ALPHA => output::write_to_file::<output::SortByName>(cards_by_set, &outfile),
        Ordering::DATE => output::write_to_file::<output::SortByDate>(cards_by_set, &outfile),
    }
}
