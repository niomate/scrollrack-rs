use lazy_static::lazy_static;
use regex::Regex;
use scryfall::{set::Set, uri::Uri};

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct CardInfo {
    name: String,
    quantity: u8,
    collector_number: u8,
}

#[derive(Debug)]
pub struct SetInfo {
    set_name: String,
}

impl PartialEq for SetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.set_name == other.set_name
    }
}

impl Eq for SetInfo {}

impl std::hash::Hash for SetInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.set_name.hash(state)
    }
}

impl SetInfo {
    pub fn new(set_name: &str, set_uri: Uri<Set>) -> Self {
        SetInfo {
            set_name: set_name.to_string(),
        }
    }

    /// Get a reference to the set info's set name.
    #[must_use]
    pub fn set_name(&self) -> &str {
        self.set_name.as_ref()
    }
}

impl CardInfo {
    pub fn new(name: String, quantity: u8) -> Self {
        CardInfo {
            name,
            quantity,
            collector_number: u8::MAX,
        }
    }
    pub fn is_basic(&self) -> bool {
        self.name == "Island"
            || self.name == "Plains"
            || self.name == "Mountain"
            || self.name == "Swamp"
            || self.name == "Forest"
            || self.name == "Wastes"
    }

    /// Get a reference to the card info's name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get the card info's quantity.
    #[must_use]
    pub fn quantity(&self) -> u8 {
        self.quantity
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
            Some(cap) => Ok(CardInfo::new(
                cap.get(3)
                    .expect("Something went wrong parsing the card name")
                    .as_str()
                    .to_string(),
                cap.get(2).map_or(1, |m| m.as_str().parse().unwrap_or(1)),
            )),
        }
    }
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
