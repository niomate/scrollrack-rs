use crate::cardinfo::CardInfo;
use crate::rules::postprocess;
use crate::scryfall_card_wrapper::ScryfallCardWrapper;
use crate::setinfo::SetInfo;
use rayon::prelude::*;
use scryfall::card;
use scryfall::search::prelude::*;
use scryfall::set;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

pub type CardsBySet = HashMap<SetInfo, Vec<ScryfallCardWrapper>>;
type PremappingEntry = (SetInfo, card::Card);
type Premapping = Vec<PremappingEntry>;

fn generate_premapping_entry(card: &card::Card) -> PremappingEntry {
    // println!("{}, {}, {}", card.set_name, card.set_type, card.set);
    (
        SetInfo::with_set_type(&card.set_name, card.set_type),
        card.to_owned(),
    )
}

pub struct CardQuery {
    _inverted_mapping: bool,
    post_process_rules: HashSet<postprocess::Combine>,
    cards: Vec<CardInfo>,
}

pub struct CardQueryBuilder {
    _inverted_mapping: bool,
    post_process_rules: HashSet<postprocess::Combine>,
    cards: Vec<CardInfo>,
}

impl CardQueryBuilder {
    fn new() -> Self {
        CardQueryBuilder {
            _inverted_mapping: false,
            post_process_rules: HashSet::new(),
            cards: vec![],
        }
    }

    pub fn invert_mapping(&mut self, value: bool) -> &mut Self {
        self._inverted_mapping = value;
        self
    }

    pub fn postprocess(&mut self, rule: postprocess::Combine) -> &mut Self {
        self.post_process_rules.insert(rule);
        self
    }

    pub fn cards(&mut self, cards: Vec<CardInfo>) -> &mut Self {
        self.cards.extend(cards);
        self
    }

    pub fn done(&self) -> CardQuery {
        CardQuery {
            _inverted_mapping: self._inverted_mapping,
            post_process_rules: self.post_process_rules.to_owned(),
            cards: self.cards.clone(),
        }
    }
}

impl CardQuery {
    pub fn build() -> CardQueryBuilder {
        CardQueryBuilder::new()
    }

    pub fn run(&self) -> CardsBySet {
        let mut merged = CardsBySet::new();

        self.cards
            .iter()
            .map(|c| self.single_query(&c))
            .flatten()
            .for_each(|res| {
                merged.entry(res.0).or_insert(Vec::new()).push(res.1.into());
            });

        let mut processed = self.postprocess(merged);
        self.dedup(&mut processed)
    }

    pub fn run_par(&self) -> CardsBySet {
        let merged = CardsBySet::new();
        let merged_ref = Arc::new(RwLock::new(merged));

        self.cards
            .par_iter()
            .map(|c| self.single_query(&c))
            .flatten()
            .for_each(|res| {
                Arc::clone(&merged_ref)
                    .write()
                    .expect("Could not get write lock")
                    .entry(res.0)
                    .or_insert(Vec::new())
                    .push(res.1.into());
            });

        let mut processed = self.postprocess(
            merged_ref
                .read()
                .expect("Could not get read lock")
                .to_owned(),
        );
        self.dedup(&mut processed)
    }

    fn dedup(&self, cards_by_set: &mut CardsBySet) -> CardsBySet {
        cards_by_set.par_iter_mut().for_each(move |(_set, cards)| {
            cards.sort_by_key(|card| card.card_name().to_owned());
            cards.dedup_by_key(|card| card.card_name().to_owned())
        });

        cards_by_set.to_owned()
    }

    fn postprocess(&self, cards_by_set: CardsBySet) -> CardsBySet {
        self.post_process_rules
            .iter()
            .fold(cards_by_set, |acc, rule| rule.apply(&acc))
    }

    fn single_query(&self, card_info: &CardInfo) -> Premapping {
        SearchOptions::new()
            .unique(UniqueStrategy::Prints)
            .query(
                exact(card_info.name()).and(
                    game(card::Game::Paper)
                        .and(not(set_type(set::SetType::Masterpiece)
                            .or(set_type(set::SetType::TreasureChest))
                            .or(set_type(set::SetType::GiftBox))
                            .or(set_type(set::SetType::Vanguard))
                            .or(set_type(set::SetType::Memorabilia))
                            .or(set_type(set::SetType::Token))
                            .or(set_type(set::SetType::Promo))
                            .or(set("fmb1"))))
                        .and(language("english").or(language("german"))),
                ),
            )
            .search_all()
            .map_or(vec![], |res| {
                res.par_iter().map(generate_premapping_entry).collect()
            })
    }
}
