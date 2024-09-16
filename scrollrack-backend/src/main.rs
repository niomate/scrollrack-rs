use rocket::http::Method;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
use rocket_cors::{AllowedOrigins, CorsOptions};
use scrollrack_core::card_query::CardQuery;
use scrollrack_core::cardinfo::CardInfo;
use scrollrack_core::output::format::OutputFormat;
use scrollrack_core::output::format::OutputTable;
use scrollrack_core::output::order::{sort, SortByName, SortedCardsBySet};
use scrollrack_core::output::render_to_string;
use scrollrack_core::parse;

#[macro_use]
extern crate rocket;

#[post("/single_card", format = "json", data = "<card>")]
async fn single_card(card: &'_ str) -> &str {
    "not implemented"
}

#[post("/decklist/raw", format = "json", data = "<cards>")]
async fn decklist_raw(cards: Json<Vec<&'_ str>>) -> Json<SortedCardsBySet> {
    let cards_by_set = CardQuery::default()
        .run(parse::parse_card_infos(cards.iter()))
        .await;

    Json(sort::<SortByName>(cards_by_set))
}

#[post("/decklist/table", format = "json", data = "<cards>")]
async fn decklist_table(cards: Json<Vec<&'_ str>>) -> String {
    let cards_by_set = CardQuery::default()
        .run(parse::parse_card_infos(cards.iter()))
        .await;

    render_to_string::<SortByName, OutputTable>(cards_by_set).unwrap_or("Error".to_owned())
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .mount(
            "/api/endpoints",
            routes![single_card, decklist_table, decklist_raw,],
        )
        .attach(cors.to_cors().unwrap())
}
