use crate::scryfall_card_wrapper::ScryfallCardWrapper;
use crate::setinfo::SetInfo;
use serde::Serialize;
use tera::{Context, Tera};

use lazy_static::lazy_static;

use super::OutputFormat;

static HTML_TEMPLATE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../templates/html_template.html"
));

#[derive(Serialize)]
struct Entry {
    set_info: SetInfo,
    cards: Vec<ScryfallCardWrapper>,
}

impl From<&(SetInfo, Vec<ScryfallCardWrapper>)> for Entry {
    fn from(value: &(SetInfo, Vec<ScryfallCardWrapper>)) -> Self {
        Entry {
            set_info: value.0.to_owned(),
            cards: value.1.to_owned(),
        }
    }
}

pub struct OutputHTML;
impl OutputFormat for OutputHTML {
    fn render(&self, c: &Vec<(SetInfo, Vec<ScryfallCardWrapper>)>) -> String {
        let entries: Vec<Entry> = c.iter().map(|entry| entry.into()).collect();

        let mut context = Context::new();
        context.insert("title", "Cards sorted by set");
        context.insert("sets", &entries);

        Tera::one_off(HTML_TEMPLATE, &context, true).unwrap_or("".to_string())
    }

    fn render_set(&self, _set_info: &SetInfo, _cards: &Vec<ScryfallCardWrapper>) -> String {
        panic!("render_set is not implemented for OutputHTML");
    }

    fn get_file_extension(&self) -> String {
        "html".to_string()
    }
}
