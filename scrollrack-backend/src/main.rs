use rocket::serde::{json::Json, Deserialize};
use scrollrack_core::cardinfo::CardInfo;
use scrollrack_core::card_query::CardQuery;
use scrollrack_core::output::format::OutputFormat;
use scrollrack_core::output::format::OutputTable;
use scrollrack_core::output::order::SortByName;
use scrollrack_core::output::render_to_string;
use scrollrack_core::parse;

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Card<'r> {
    card: &'r str,
}

#[post("/single_card", format = "json", data = "<card>")]
async fn single_card(card: &'_ str) -> &str {
    "not implemented"
}

#[post("/decklist", format = "json", data = "<cards>")]
async fn decklist(cards: Json<Vec<&'_ str>>) -> String {
    let cards_by_set = CardQuery::default()
        .run(parse::parse_card_infos(cards.iter()))
        .await;

    let formatter = Box::new(OutputTable {}) as Box<dyn OutputFormat>;
    render_to_string(cards_by_set, formatter, SortByName {}).unwrap_or("Error".to_owned())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/endpoints", routes![single_card, decklist])
}
