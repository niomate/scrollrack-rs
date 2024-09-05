use crate::scryfall_card_wrapper::ScryfallCardWrapper;
use crate::setinfo::SetInfo;
use itertools::Itertools;

pub mod html;
pub mod list;
pub mod table;

pub use html::OutputHTML;
pub use list::OutputItemList;
pub use table::OutputTable;

pub trait OutputFormat {
    fn render(c: &Vec<(SetInfo, Vec<ScryfallCardWrapper>)>) -> String {
        c.iter()
            .map(|(set_info, cards)| {
                format!(
                    "{}:\n{}",
                    set_info.set_name(),
                    Self::render_set(&set_info, &cards)
                )
            })
            .join("\n\n")
    }

    fn render_set(set_info: &SetInfo, cards: &Vec<ScryfallCardWrapper>) -> String;

    fn get_file_extension() -> String;
}
