use crate::cardinfo::{CardInfo, SetInfo};
use scryfall::search::prelude::*;
use std::collections::{HashMap, HashSet};

pub type CardsBySet = HashMap<SetInfo, HashSet<CardInfo>>;

pub fn query(info: CardInfo) -> Vec<(SetInfo, CardInfo)> {
    SearchOptions::new()
        .unique(UniqueStrategy::Prints)
        .query(exact(info.name()))
        .search_all()
        .unwrap_or(vec![])
        .iter()
        .map(|scryinfo| (SetInfo::new(&scryinfo.set_name), info.clone()))
        .collect()
}

pub fn merge_results(results: Vec<(SetInfo, CardInfo)>) -> CardsBySet {
    let mut merged = CardsBySet::new();
    results.into_iter().for_each(|res| {
        merged.entry(res.0).or_insert(HashSet::new()).insert(res.1);
    });
    merged
}

pub fn query_and_merge_all(cards: Vec<CardInfo>) -> CardsBySet {
    merge_results(cards.into_iter().map(query).flatten().collect())
}
