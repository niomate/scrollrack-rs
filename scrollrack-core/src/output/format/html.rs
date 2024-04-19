use crate::scryfall_card_wrapper::ScryfallCardWrapper;
use crate::setinfo::SetInfo;
use serde::Serialize;
use tera::{Context, Tera};

use lazy_static::lazy_static;

use super::OutputFormat;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

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

        TEMPLATES
            .render("html_template.html", &context)
            .unwrap_or("".to_string())
    }

    fn render_set(&self, _set_info: &SetInfo, _cards: &Vec<ScryfallCardWrapper>) -> String {
        panic!("render_set is not implemented for OutputHTML");
    }

    fn get_file_extension(&self) -> String {
        "html".to_string()
    }
}
