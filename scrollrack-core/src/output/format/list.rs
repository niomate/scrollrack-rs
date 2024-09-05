use super::OutputFormat;
use crate::scryfall_card_wrapper::ScryfallCardWrapper;
use crate::setinfo::SetInfo;
use itertools::Itertools;

pub struct OutputItemList;
impl OutputFormat for OutputItemList {
    fn render_set( set_info: &SetInfo, cards: &Vec<ScryfallCardWrapper>) -> String {
        cards
            .iter()
            .sorted_by_key(|card| card.card_name())
            .map(|card| {
                if set_info.virtual_set() {
                    "\t - ".to_owned() + card.card_name()
                } else {
                    "\t - ".to_owned() + &card.format_detailed()
                }
            })
            .join("\n")
    }

    fn get_file_extension() -> String {
        "txt".to_string()
    }
}
