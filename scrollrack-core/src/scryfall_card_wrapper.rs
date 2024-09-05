use scryfall::card;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScryfallCardWrapper {
    set_code: String,
    set_name: String,
    price: String,
    card_name: String,
    collector_number: String,
}

impl ScryfallCardWrapper {
    pub fn new(
        set_code: String,
        set_name: String,
        price: String,
        card_name: String,
        collector_number: String,
    ) -> Self {
        Self {
            set_code,
            set_name,
            price,
            card_name,
            collector_number,
        }
    }

    pub fn set_code(&self) -> &str {
        self.set_code.as_ref()
    }

    pub fn set_name(&self) -> &str {
        self.set_name.as_ref()
    }

    pub fn price(&self) -> &str {
        self.price.as_ref()
    }

    pub fn card_name(&self) -> &str {
        self.card_name.as_ref()
    }

    pub fn collector_number(&self) -> &str {
        self.collector_number.as_ref()
    }

    pub fn format_detailed(&self) -> String {
        format!(
            "{} (#{}): {} EUR",
            self.card_name(),
            self.collector_number(),
            self.price()
        )
    }
}

impl From<card::Card> for ScryfallCardWrapper {
    fn from(c: card::Card) -> Self {
        ScryfallCardWrapper::new(
            c.set.to_string(),
            c.set_name,
            c.prices.eur.unwrap_or("--".to_string()),
            c.name,
            c.collector_number,
        )
    }
}
