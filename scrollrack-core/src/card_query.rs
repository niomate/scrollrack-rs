use crate::cardinfo::CardInfo;
use crate::rules::postprocess;
use crate::rules::postprocess::Combine;
use crate::scryfall_card_wrapper::ScryfallCardWrapper;
use crate::setinfo::SetInfo;
use futures::future;
use rayon::prelude::*;
use scryfall::card;
use scryfall::search::prelude::*;
use scryfall::set;
use std::collections::HashMap;
use std::collections::HashSet;

pub type CardsBySet = HashMap<SetInfo, Vec<ScryfallCardWrapper>>;
type PremappingEntry = (SetInfo, card::Card);
type Premapping = Vec<PremappingEntry>;

fn generate_premapping_entry(card: &card::Card) -> PremappingEntry {
    (
        SetInfo::real_set(&card.set_name, &card.set.to_string(), card.set_type),
        card.to_owned(),
    )
}

pub struct CardQuery {
    _inverted_mapping: bool,
    post_process_rules: HashSet<postprocess::Combine>,
}

pub struct CardQueryBuilder {
    _inverted_mapping: bool,
    post_process_rules: HashSet<postprocess::Combine>,
}

impl CardQueryBuilder {
    fn new() -> Self {
        CardQueryBuilder {
            _inverted_mapping: false,
            post_process_rules: HashSet::new(),
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

    pub fn done(&self) -> CardQuery {
        CardQuery {
            _inverted_mapping: self._inverted_mapping,
            post_process_rules: self.post_process_rules.to_owned(),
        }
    }
}

impl Default for CardQuery {
    fn default() -> Self {
        CardQueryBuilder::new()
            .postprocess(Combine::Commander)
            .postprocess(Combine::MysteryAndTheList)
            .postprocess(Combine::DuelDecks)
            .done()
    }
}

impl CardQuery {
    pub fn build() -> CardQueryBuilder {
        CardQueryBuilder::new()
    }

    pub async fn run(&self, cards: Vec<CardInfo>) -> CardsBySet {
        let mut merged = CardsBySet::new();

        future::join_all(cards.iter().map(|c| self.single_query(&c)))
            .await
            .into_iter()
            .flatten()
            .for_each(|res| {
                merged.entry(res.0).or_insert(Vec::new()).push(res.1.into());
            });

        let mut processed = self.postprocess(merged);
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

    async fn single_query(&self, card_info: &CardInfo) -> Premapping {
        match SearchOptions::new()
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
            .await
        {
            Ok(cards) => cards.par_iter().map(generate_premapping_entry).collect(),
            Err(e) => {
                println!("Couldn't find {}: {:?}", card_info.name(), e);
                vec![]
            }
        }
    }
}
