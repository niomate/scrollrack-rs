use crate::cardinfo::{CardInfo, SetInfo};
use scryfall::card;
use scryfall::search::prelude::*;
use std::collections::HashMap;

pub type CardsBySet = HashMap<SetInfo, Vec<card::Card>>;

/// For a given card name, query the scryfall API to fetch all printings.
pub fn query(info: CardInfo, no_promos: bool) -> Vec<(SetInfo, card::Card)> {
    let card_query = if no_promos {
        Query::And(vec![exact(info.name()), not(PrintingIs::Promo)])
    } else {
        Query::And(vec![exact(info.name())])
    };
    SearchOptions::new()
        .unique(UniqueStrategy::Prints)
        .query(card_query)
        .search_all()
        .map_or(vec![], |res| {
            res.iter()
                .map(|scryinfo| {
                    (
                        SetInfo::new(&scryinfo.set_name, scryinfo.set_uri.to_owned()),
                        scryinfo.to_owned(),
                    )
                })
                .collect()
        })
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

pub fn query_and_merge_all(cards: Vec<CardInfo>, no_promos: bool) -> CardsBySet {
    merge_results(
        cards
            .into_iter()
            .map(|c| query(c, no_promos))
            .flatten()
            .collect(),
    )
}
