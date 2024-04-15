use crate::card_query::CardsBySet;
use crate::scryfall_card_wrapper::ScryfallCardWrapper;
use crate::setinfo::SetInfo;
use anyhow::Result;
use chrono::naive::NaiveDate;
use itertools::Itertools;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tabled::Table;

pub fn gen_outfile_name(in_name: &str) -> String {
    format!(
        "{}-by-set.txt",
        Path::new(in_name).file_stem().unwrap().to_str().unwrap()
    )
}

pub trait SetInfoOrder {
    type ReturnType: Ord;
    fn get_key(set_info: &SetInfo) -> Self::ReturnType;
}

pub struct SortByName;
impl SetInfoOrder for SortByName {
    type ReturnType = String;
    fn get_key(set_info: &SetInfo) -> String {
        set_info.set_name().to_string()
    }
}

pub struct SortByDate;
impl SetInfoOrder for SortByDate {
    type ReturnType = NaiveDate;
    // TODO: Cache set infos ?
    fn get_key(_set_info: &SetInfo) -> NaiveDate {
        // set_info
        //     .set_uri()
        //     .fetch()
        //     .unwrap()
        //     .released_at
        //     // Year in which MTG was first released
        //     .unwrap_or(NaiveDate::from_yo(1993, 1))
        NaiveDate::from_yo_opt(1993, 1).expect("invalid or out-of-range date")
    }
}

pub trait OutputFormat {
    fn render<P>(c: CardsBySet) -> String
    where
        P: SetInfoOrder;
}

pub struct OutputItemList;
impl OutputFormat for OutputItemList {
    fn render<P>(c: CardsBySet) -> String
    where
        P: SetInfoOrder,
    {
        c.keys()
            .sorted_by_key(|set_info| P::get_key(set_info))
            .map(|k| {
                format!(
                    "{}:\n{}",
                    k.set_name(),
                    c[k].iter()
                        .sorted_by_key(|card| card.card_name())
                        .map(|card| if k.virtual_set() {
                            "\t - ".to_owned() + card.card_name()
                        } else {
                            "\t - ".to_owned() + &card.format_detailed()
                        })
                        .join("\n")
                )
            })
            .join("\n\n")
    }
}

#[derive(tabled::Tabled)]
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
    fn render<P>(c: CardsBySet) -> String
    where
        P: SetInfoOrder,
    {
        c.keys()
            .sorted_by_key(|set_info| P::get_key(set_info))
            .map(|k| {
                format!(
                    "{}:\n{}",
                    k.set_name(),
                    Table::new(
                        c[k].iter()
                            .sorted_by_key(|card| card.card_name())
                            .map(|c| Into::<RenameLater>::into(c))
                    )
                )
            })
            .join("\n\n")
    }
}

pub fn render<F: OutputFormat, P: SetInfoOrder>(c: CardsBySet) -> String {
    F::render::<P>(c)
}

pub fn write_to_file(data: &str, path: &str) -> Result<()> {
    let mut outfile = File::create(path)?;
    Ok(outfile.write_all(data.as_bytes())?)
}

#[cfg(test)]
mod tests {
    use super::{gen_outfile_name, output_table, CardsBySet, SortByName};
    use crate::scryfall_card_wrapper::ScryfallCardWrapper;
    use crate::setinfo::SetInfo;

    #[test]
    fn test_output_string() {
        let mut c = CardsBySet::new();
        c.entry(SetInfo::new("Kaladesh"))
            .or_insert(vec![ScryfallCardWrapper::new(
                "Kaladesh".to_string(),
                "10".to_string(),
                "Ornithopter".to_string(),
                "1337".to_string(),
            )]);

        c.entry(SetInfo::new("Zendikar")).or_insert(vec![
            ScryfallCardWrapper::new(
                "Zendikar".to_string(),
                "10".to_string(),
                "Ornithopter".to_string(),
                "1337".to_string(),
            ),
            ScryfallCardWrapper::new(
                "Zendikar".to_string(),
                "0.2".to_string(),
                "Opt".to_string(),
                "1337".to_string(),
            ),
        ]);

        assert_eq!(
            output_table::<SortByName>(c),
            "Kaladesh:\n\t- Ornithopter (#1337): 10 EUR\n\nZendikar:\n\t- Opt (#1337): 0.2 EUR\n\t- Ornithopter (#1337): 10 EUR"
        )
    }

    #[test]
    fn test_gen_output_name() {
        assert_eq!(gen_outfile_name("test_name.txt"), "test_name-by-set.txt");
        assert_eq!(gen_outfile_name("test_deck.dck"), "test_deck-by-set.txt");
    }
}
