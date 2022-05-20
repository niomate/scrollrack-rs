use crate::cardinfo::SetInfo;
use crate::query_stuff::CardsBySet;
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

pub trait SetInfoSortKey
{
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

pub struct _SortByDate;
impl SetInfoSortKey for _SortByDate {
    type ReturnType =  u32;
    fn get_key(_set_info: &SetInfo) -> u32 {
        unimplemented!()
    }
}

pub fn write_to_file<P>(
    cards_by_set: CardsBySet,
    path: &str,
) -> Result<(), Box<dyn error::Error>>
where
    P: SetInfoSortKey,
{
    let mut outfile = File::create(path)?;
    cards_by_set
        .keys()
        .sorted_by_key(|set_info| P::get_key(set_info))
        .for_each(|k| {
            outfile
                .write_all(format!("{}:\n", k.set_name()).as_bytes())
                .unwrap();
            for card in &cards_by_set[k] {
                outfile
                    .write_all(format!("\t- {}\n", card.name).as_bytes())
                    .unwrap();
            }
            outfile.write_all(b"\n").unwrap();
        });
    Ok(())
}
