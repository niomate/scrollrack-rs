use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::http::Method;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
use rocket::{Request, Response};
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

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api/endpoints",
            routes![single_card, decklist_table, decklist_raw, all_options],
        )
        .attach(CORS)
}
