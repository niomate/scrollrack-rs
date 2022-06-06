use crate::cardinfo::CardInfo;
use std::fs::File;
use std::io::BufRead;
use std::{io, path::Path};
use anyhow::Result;

pub fn read_lines<P>(filename: P) -> Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    File::open(filename)
        .map(|file| io::BufReader::new(file).lines().filter_map(|l| l.ok()))
        .map_err(|e| e.into())
}

pub fn parse_card_infos<P>(lines: impl Iterator<Item = P>) -> impl Iterator<Item = CardInfo>
where
    P: ToString,
{
    lines
        .map(|l| l.to_string())
        .filter(|l| !l.starts_with("#"))
        .filter_map(|l| CardInfo::try_from(&l[..]).ok())
        .filter(|c| !c.is_basic())
}

#[cfg(test)]
mod tests {
    use super::parse_card_infos;
    use crate::cardinfo::CardInfo;

    #[test]
    fn test_parse_simple() {
        let lines = vec![
            "2 Ornithopter".to_string(),
            "Emry, Lurker of the Loch".to_string(),
        ];

        let parsed = parse_card_infos(lines.iter()).collect::<Vec<_>>();
        assert_eq!(
            parsed,
            vec![
                CardInfo::new("Ornithopter", 2),
                CardInfo::new("Emry, Lurker of the Loch", 1)
            ]
        )
    }

    #[test]
    fn test_parse_empty() {
        let lines: Vec<String> = vec![];
        let parsed = parse_card_infos(lines.iter()).collect::<Vec<_>>();
        assert_eq!(parsed, vec![]);
    }

    #[test]
    fn test_parse_basics() {
        let lines = vec!["20 Island", "10 Forest", "10 Shadowborn Apostle"];

        let parsed = parse_card_infos(lines.iter()).collect::<Vec<_>>();
        assert_eq!(parsed, vec![CardInfo::new("Shadowborn Apostle", 10),])
    }

    #[test]
    fn test_u8_bounds_violated() {
        let lines = vec!["10000 Shadowborn Apostle"];

        let parsed = parse_card_infos(lines.iter()).collect::<Vec<_>>();

        // quantity is only stored as a u8 since you won't have more than 256 copies of one card in
        // any deck, so we set to 1 in this case
        assert_eq!(parsed, vec![CardInfo::new("Shadowborn Apostle", 1)])
    }
}
