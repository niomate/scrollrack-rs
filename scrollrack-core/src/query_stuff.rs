use crate::cardinfo::{CardInfo, SetInfo};
use scryfall::card;
use scryfall::search::prelude::*;
use std::collections::HashMap;

pub type CardsBySet = HashMap<SetInfo, Vec<ScryfallCardWrapper>>;

pub struct CardQuery {
    include_promos: bool,
    _inverted_mapping: bool,
}

pub struct ScryfallCardWrapper {
    set_name: String,
    price: String,
    card_name: String,
    collector_number: String,
}

impl ScryfallCardWrapper {
    pub fn new(
        set_name: String,
        price: String,
        card_name: String,
        collector_number: String,
    ) -> Self {
        Self {
            set_name,
            price,
            card_name,
            collector_number,
        }
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
}

impl From<card::Card> for ScryfallCardWrapper {
    fn from(c: card::Card) -> Self {
        ScryfallCardWrapper::new(
            c.set_name,
            c.prices.eur.unwrap_or("--".to_string()),
            c.name,
            c.collector_number,
        )
    }
}

impl CardQuery {
    pub fn with_options(include_promos: bool, _inverted_mapping: bool) -> Self {
        CardQuery {
            include_promos,
            _inverted_mapping,
        }
    }

    pub fn query(&self, cards: impl Iterator<Item = CardInfo>) -> CardsBySet {
        self.merge_results(cards.map(|c| self.single_query(&c)).flatten())
    }

    fn merge_results(&self, results: impl Iterator<Item = (SetInfo, card::Card)>) -> CardsBySet {
        let mut merged = CardsBySet::new();
        results.into_iter().for_each(|res| {
            merged.entry(res.0).or_insert(Vec::new()).push(res.1.into());
        });
        merged
            .values_mut()
            .for_each(|cards| cards.dedup_by_key(|card| card.card_name().to_owned()));
        merged
    }

    fn single_query(&self, info: &CardInfo) -> Vec<(SetInfo, card::Card)> {
        SearchOptions::new()
            .unique(UniqueStrategy::Prints)
            .query(self.assemble_query(info))
            .search_all()
            .map_or(vec![], |res| {
                res.iter()
                    .filter(|scryinfo| scryinfo.games.contains(&scryfall::card::Game::Paper))
                    .map(|scryinfo| (SetInfo::new(&scryinfo.set_name), scryinfo.to_owned()))
                    .collect()
            })
    }

    fn assemble_query(&self, info: &CardInfo) -> Query {
        let mut query_vec = vec![exact(info.name())];
        if !self.include_promos {
            query_vec.push(not(PrintingIs::Promo));
        }

        Query::And(query_vec)
    }
}
