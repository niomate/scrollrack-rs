use clap::ArgEnum;
use clap::Parser;

use scrollrack_core::card_query::CardQuery;
use scrollrack_core::output::{
    gen_outfile_name, render, write_to_file, OutputFormat, OutputItemList, OutputTable,
    SortByCardAmount, SortByDate, SortByName,
};
use scrollrack_core::parse;

use anyhow::Result;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum Ordering {
    ALPHA,
    DATE,
    AMOUNT,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum Output {
    TABLE,
    LIST,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(help = "Path to the card list")]
    path: String,
    #[clap(short='O', long, arg_enum, default_value_t=Ordering::ALPHA, help="Specifies in which order the sets should be printed in the output file")]
    ordering: Ordering,
    #[clap(short, long, arg_enum, default_value_t=Output::LIST, help="Specifies how the cards should be displayed")]
    format: Output,
    #[clap(short, long, help = "Output sets per card instead of cards per set")]
    inverted: bool,
    #[clap(short, long, help = "Path to the output file")]
    output: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let lines = parse::read_lines(&args.path)?;

    let cards_by_set = CardQuery::default()
        .run(parse::parse_card_infos(lines))
        .await;

    let outfile = match args.output {
        Some(path) => path,
        None => gen_outfile_name(&args.path),
    };

    let out_string = match (args.format, args.ordering) {
        (Output::LIST, Ordering::ALPHA) => render::<OutputItemList, SortByName>(&cards_by_set),
        (Output::LIST, Ordering::DATE) => render::<OutputItemList, SortByDate>(&cards_by_set),
        (Output::LIST, Ordering::AMOUNT) => {
            OutputItemList::render::<SortByCardAmount>(&cards_by_set)
        }
        (Output::TABLE, Ordering::ALPHA) => render::<OutputTable, SortByName>(&cards_by_set),
        (Output::TABLE, Ordering::DATE) => render::<OutputTable, SortByDate>(&cards_by_set),
        (Output::TABLE, Ordering::AMOUNT) => render::<OutputTable, SortByCardAmount>(&cards_by_set),
    };

    write_to_file(&out_string, &outfile)
}
