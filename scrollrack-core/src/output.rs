use crate::cardinfo::SetInfo;
use crate::query_stuff::CardsBySet;
use chrono::naive::NaiveDate;
use itertools::Itertools;
use std::error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn gen_outfile_name(in_name: &str) -> String {
    format!(
        "{}-by-set.txt",
        Path::new(&in_name).file_stem().unwrap().to_str().unwrap()
    )
}

pub trait SetInfoSortKey {
    type ReturnType: Ord;
    fn get_key(set_info: &SetInfo) -> Self::ReturnType;
}

pub struct SortByName;
impl SetInfoSortKey for SortByName {
    type ReturnType = String;
    fn get_key(set_info: &SetInfo) -> String {
        set_info.set_name().to_string()
    }
}

pub struct SortByDate;
impl SetInfoSortKey for SortByDate {
    type ReturnType = NaiveDate;
    fn get_key(set_info: &SetInfo) -> NaiveDate {
        set_info
            .set_uri()
            .fetch()
            .unwrap()
            .released_at
            .unwrap_or(NaiveDate::from_yo(1993, 1))
    }
}

pub fn write_to_file<P>(cards_by_set: CardsBySet, path: &str) -> Result<(), String>
where
    P: SetInfoSortKey,
{
    let mut outfile = File::create(path).map_err(|err| format!("Could not open file: {}", &err))?;
    let out_string: String = cards_by_set
        .keys()
        .sorted_by_key(|set_info| P::get_key(set_info))
        .map(|k| {
            format!("{}:\n", k.set_name())
                + &cards_by_set[k]
                    .iter()
                    .map(|card| format!("\t- {}", card.name))
                    .join("\n")
        })
        .collect();

    outfile
        .write_all(out_string.as_bytes())
        .map_err(|err| format!("Could not write to file: {}", err))
}
