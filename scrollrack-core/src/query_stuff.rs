use crate::cardinfo::{CardInfo, SetInfo};
use scryfall::card;
use scryfall::search::prelude::*;
use std::collections::HashMap;

pub type CardsBySet = HashMap<SetInfo, Vec<card::Card>>;

pub fn query(info: CardInfo) -> Vec<(SetInfo, card::Card)> {
    SearchOptions::new()
        .unique(UniqueStrategy::Prints)
        .query(exact(info.name()))
        .search_all()
        .unwrap_or(vec![])
        .iter()
        .map(|scryinfo| (SetInfo::new(&scryinfo.set_name), scryinfo.to_owned()))
        .collect()
}

pub fn merge_results(results: Vec<(SetInfo, card::Card)>) -> CardsBySet {
    let mut merged = CardsBySet::new();
    results.into_iter().for_each(|res| {
        merged.entry(res.0).or_insert(Vec::new()).push(res.1);
    });
    merged
        .values_mut()
        .for_each(|cards| cards.dedup_by_key(|card| card.name.to_owned()));
    merged
}

pub fn query_and_merge_all(cards: Vec<CardInfo>) -> CardsBySet {
    merge_results(cards.into_iter().map(query).flatten().collect())
}
