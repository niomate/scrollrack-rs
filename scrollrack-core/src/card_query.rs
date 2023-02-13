use crate::cardinfo::CardInfo;
use crate::rules::postprocess;
use crate::rules::prefilter;
use crate::scryfall_card_wrapper::ScryfallCardWrapper;
use crate::setinfo::SetInfo;
use scryfall::card;
use scryfall::search::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub type CardsBySet = HashMap<SetInfo, Vec<ScryfallCardWrapper>>;
type PremappingEntry = (SetInfo, card::Card);
type Premapping = Vec<PremappingEntry>;

fn generate_premapping_entry(card: &card::Card) -> PremappingEntry {
    (
        SetInfo::with_set_type(&card.set_name, card.set_type),
        card.to_owned(),
    )
}

pub struct CardQuery {
    include_promos: bool,
    _inverted_mapping: bool,
    prefilter_rules: HashSet<prefilter::PrefilterRule>,
    post_process_rules: HashSet<postprocess::PostProcessRule>,
    cards: Vec<CardInfo>,
}

pub struct CardQueryBuilder {
    include_promos: bool,
    _inverted_mapping: bool,
    prefilter_rules: HashSet<prefilter::PrefilterRule>,
    post_process_rules: HashSet<postprocess::PostProcessRule>,
    cards: Vec<CardInfo>,
}

impl CardQueryBuilder {
    fn new() -> Self {
        CardQueryBuilder {
            include_promos: false,
            _inverted_mapping: false,
            prefilter_rules: HashSet::new(),
            post_process_rules: HashSet::new(),
            cards: vec![],
        }
    }

    pub fn invert_mapping(&mut self, value: bool) -> &mut Self {
        self._inverted_mapping = value;
        self
    }

    pub fn with_prefilter(&mut self, rule: prefilter::PrefilterRule) -> &mut Self {
        self.prefilter_rules.insert(rule);
        self
    }

    pub fn with_postprocess(&mut self, rule: postprocess::PostProcessRule) -> &mut Self {
        self.post_process_rules.insert(rule);
        self
    }

    pub fn include_promos(&mut self, value: bool) -> &mut Self {
        self.include_promos = value;
        self
    }

    pub fn cards(&mut self, cards: Vec<CardInfo>) -> &mut Self {
        self.cards.extend(cards);
        self
    }

    pub fn done(&self) -> CardQuery {
        CardQuery {
            include_promos: self.include_promos,
            _inverted_mapping: self._inverted_mapping,
            prefilter_rules: self.prefilter_rules.to_owned(),
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
        let res = self.merge_results(self.cards.iter().map(|c| self.single_query(&c)).flatten());
        let mut processed = self.postprocess(res);
        self.dedup(&mut processed)
    }

    fn merge_results(&self, results: impl Iterator<Item = PremappingEntry>) -> CardsBySet {
        let mut merged = CardsBySet::new();
        results.into_iter().for_each(|res| {
            merged.entry(res.0).or_insert(Vec::new()).push(res.1.into());
        });

        self.dedup(&mut merged)
    }

    fn dedup(&self, cards_by_set: &mut CardsBySet) -> CardsBySet {
        cards_by_set.values_mut().for_each(|cards| {
            cards.sort_by_key(|card| card.card_name().to_owned());
            cards.dedup_by_key(|card| card.card_name().to_owned())
        });

        cards_by_set.to_owned()
    }

    fn prefilter(&self, card: &card::Card) -> bool {
        self.prefilter_rules
            .iter()
            .fold(true, |acc, rule| acc && rule.check(card))
    }

    fn postprocess(&self, cards_by_set: CardsBySet) -> CardsBySet {
        self.post_process_rules
            .iter()
            .fold(cards_by_set, |acc, rule| rule.apply(acc))
    }

    fn single_query(&self, card_info: &CardInfo) -> Premapping {
        SearchOptions::new()
            .unique(UniqueStrategy::Prints)
            .query(exact(card_info.name()))
            .search_all()
            .map_or(vec![], |res| {
                res.iter()
                    .filter(|card| self.prefilter(card))
                    .map(generate_premapping_entry)
                    .collect()
            })
    }
}
