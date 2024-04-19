use super::OutputFormat;
use crate::scryfall_card_wrapper::ScryfallCardWrapper;
use crate::setinfo::SetInfo;
use itertools::Itertools;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct RenameLater {
    name: String,
    number: String,
    price: String,
}

impl From<&ScryfallCardWrapper> for RenameLater {
    fn from(value: &ScryfallCardWrapper) -> Self {
        RenameLater {
            name: value.card_name().into(),
            number: value.collector_number().into(),
            price: value.price().into(),
        }
    }
}

pub struct OutputTable;
impl OutputFormat for OutputTable {
    fn render_set(&self, _: &SetInfo, cards: &Vec<ScryfallCardWrapper>) -> String {
        Table::new(
            cards
                .iter()
                .sorted_by_key(|card| card.card_name())
                .map(|c| Into::<RenameLater>::into(c)),
        )
        .to_string()
    }

    fn get_file_extension(&self) -> String {
        "txt".to_string()
    }
}
