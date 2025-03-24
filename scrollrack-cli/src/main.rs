use clap::ArgEnum;
use clap::Parser;

use scrollrack_core::card_query::CardQuery;
use scrollrack_core::output::render_to_file;
use scrollrack_core::output::{format, order};
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
    HTML,
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

    const formatter = match args.format {
        Output::TABLE => Box::new(format::OutputTable {}) as Box<dyn format::OutputFormat>,
        Output::LIST => Box::new(format::OutputItemList {}) as Box<dyn format::OutputFormat>,
        Output::HTML => Box::new(format::OutputHTML {}) as Box<dyn format::OutputFormat>,
    };

    match args.ordering {
        Ordering::ALPHA => render_to_file::<order::SortByName, formatter>(
            args.output.unwrap_or(args.path),
            cards_by_set,
        ),
        Ordering::DATE => render_to_file::<order::SortByDate, formatter>(
            args.output.unwrap_or(args.path),
            cards_by_set,
        ),
        Ordering::AMOUNT => render_to_file::<order::SortByCardAmount, formatter>(
            args.output.unwrap_or(args.path),
            cards_by_set,
        ),
    }
}
