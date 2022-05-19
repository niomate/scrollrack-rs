use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use regex::Regex;
use scryfall::search::prelude::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    path: String,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct CardInfo {
    name: String,
    quantity: u8,
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct SetInfo {
    set_name: String,
}

impl CardInfo {
    fn is_basic(&self) -> bool {
        self.name == "Island"
            || self.name == "Plains"
            || self.name == "Mountain"
            || self.name == "Swamp"
            || self.name == "Forest"
            || self.name == "Wastes"
    }
}

impl TryFrom<&str> for CardInfo {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"((\d*) )?(.+)").unwrap();
        }

        match RE.captures(s) {
            None => Err("Could not parse"),
            Some(cap) => Ok(CardInfo {
                name: cap
                    .get(3)
                    .expect("Something went wrong parsing the card name")
                    .as_str()
                    .to_string(),
                quantity: cap.get(2).map_or(1, |m| m.as_str().parse().unwrap_or(1)),
            }),
        }
    }
}

mod output {

    use crate::{File, Path};

    fn gen_outfile_name(in_name: &str) -> String {
        format!(
            "{}-by-set.txt",
            Path::new(&in_name).file_stem().unwrap().to_str().unwrap()
        )
    }

    pub(crate) fn gen_outfile_from_infile(infile_path: &str) -> File {
        File::create(gen_outfile_name(infile_path)).expect("Could not open a new file")
    }
}

fn main() {
    let args = Args::parse();

    let mut cards_by_set = HashMap::<SetInfo, HashSet<CardInfo>>::new();

    if let Ok(lines) = read_lines(&args.path) {
        let card_infos = lines
            .filter_map(|l| l.ok())
            .filter(|l| !l.starts_with("#"))
            .filter_map(|l| CardInfo::try_from(&l[..]).ok())
            .filter(|c| !c.is_basic());

        for info in card_infos {
            SearchOptions::new()
                .unique(UniqueStrategy::Prints)
                .query(exact(&info.name))
                .search_all()
                .unwrap_or(vec![])
                .iter()
                .for_each(|scryinfo| {
                    cards_by_set
                        .entry(SetInfo {
                            set_name: scryinfo.set_name.clone(),
                        })
                        .or_insert(HashSet::new())
                        .insert(info.clone());
                });
        }
    }

    println!("{:?}", cards_by_set);

    let mut outfile = output::gen_outfile_from_infile(&args.path);

    cards_by_set.keys().for_each(|k| {
        outfile
            .write_all(format!("{}:\n", k.set_name).as_bytes())
            .unwrap();
        for card in &cards_by_set[k] {
            outfile
                .write_all(format!("\t- {}\n", card.name).as_bytes())
                .unwrap();
        }
        outfile.write_all(b"\n").unwrap();
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_from_string_mult() {
        let c = CardInfo::try_from("2 Ornithopter");
        assert!(c.is_ok());
        assert_eq!(c.as_ref().unwrap().quantity, 2);
        assert_eq!(c.as_ref().unwrap().name, "Ornithopter");
    }

    #[test]
    fn test_card_from_string_single() {
        let c = CardInfo::try_from("Ornithopter");
        assert!(c.is_ok());
        assert_eq!(c.as_ref().unwrap().quantity, 1);
        assert_eq!(c.as_ref().unwrap().name, "Ornithopter");
    }

    #[test]
    fn test_card_from_string_empty() {
        let c = CardInfo::try_from("");
        assert!(c.is_err());
    }
}
