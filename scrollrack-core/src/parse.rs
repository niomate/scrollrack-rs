use crate::cardinfo::CardInfo;
use std::error;
use std::fs::File;
use std::io::BufRead;
use std::{io, path::Path};

pub fn parse_card_infos<P>(
    filename: P,
) -> Result<impl Iterator<Item = CardInfo>, Box<dyn error::Error>>
where
    P: AsRef<Path>,
{
    File::open(filename)
        .map(|file| io::BufReader::new(file).lines())
        .map(|lines| {
            lines
                .filter_map(|l| l.ok())
                .filter(|l| !l.starts_with("#"))
                .filter_map(|l| CardInfo::try_from(&l[..]).ok())
                .filter(|c| !c.is_basic())
        })
        .map_err(|e| e.into())
}
