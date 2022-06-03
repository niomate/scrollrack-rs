use crate::cardinfo::SetInfo;
use crate::query_stuff::{CardsBySet, ScryfallCardWrapper};
use chrono::naive::NaiveDate;
use itertools::Itertools;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
    fn get_key(_set_info: &SetInfo) -> NaiveDate {
        // set_info
        //     .set_uri()
        //     .fetch()
        //     .unwrap()
        //     .released_at
        //     // Year in which MTG was first released
        //     .unwrap_or(NaiveDate::from_yo(1993, 1))
        NaiveDate::from_yo(1993, 1)
    }
}

pub fn output_string<P>(cards_by_set: CardsBySet) -> String
where
    P: SetInfoOrder,
{
    cards_by_set
        .keys()
        .sorted_by_key(|set_info| P::get_key(set_info))
        .map(|k| {
            format!(
                "{}:\n{}",
                k.set_name(),
                cards_by_set[k]
                    .iter()
                    .sorted_by_key(|card| card.card_name())
                    .map(|card| format!(
                        "\t- {} (#{}): {} EUR",
                        card.card_name(),
                        card.collector_number(),
                        card.price()
                    ))
                    .join("\n")
            )
        })
        .join("\n\n")
}

pub fn write_to_file(data: &str, path: &str) -> Result<(), String> {
    let mut outfile = File::create(path).map_err(|err| format!("Could not open file: {}", &err))?;
    outfile
        .write_all(data.as_bytes())
        .map_err(|err| format!("Could not write to file: {}", err))
}

#[cfg(test)]
mod tests {
    use crate::cardinfo::SetInfo;

    use super::{gen_outfile_name, output_string, CardsBySet, ScryfallCardWrapper, SortByName};

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
            output_string::<SortByName>(c),
            "Kaladesh:\n\t- Ornithopter (#1337): 10 EUR\n\nZendikar:\n\t- Opt (#1337): 0.2 EUR\n\t- Ornithopter (#1337): 10 EUR"
        )
    }

    #[test]
    fn test_gen_output_name() {
        assert_eq!(gen_outfile_name("test_name.txt"), "test_name-by-set.txt");
        assert_eq!(gen_outfile_name("test_deck.dck"), "test_deck-by-set.txt");
    }
}
