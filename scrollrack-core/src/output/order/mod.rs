use crate::card_query::CardsBySet;
use crate::scryfall_card_wrapper::ScryfallCardWrapper;
use crate::setinfo::SetInfo;

use chrono::NaiveDate;
use itertools::Itertools;

pub type SortedCardsBySet = Vec<(SetInfo, Vec<ScryfallCardWrapper>)>;

pub trait SetInfoOrder {
    type ReturnType: Ord;
    fn get_key(set_info: &SetInfo) -> Self::ReturnType;
    fn get_combined_key(
        set_info: &SetInfo,
        _cards_in_set: &Vec<ScryfallCardWrapper>,
    ) -> Self::ReturnType {
        Self::get_key(set_info)
    }
    fn sort(cards_by_set: CardsBySet) -> SortedCardsBySet {
        cards_by_set
            .into_iter()
            .sorted_by_key(|(set_info, cards)| Self::get_combined_key(set_info, cards))
            .collect()
    }
}

pub struct SortByName;
impl SetInfoOrder for SortByName {
    type ReturnType = String;
    fn get_key(set_info: &SetInfo) -> Self::ReturnType {
        set_info.set_name().to_string()
    }
}

pub struct SortByDate;
impl SetInfoOrder for SortByDate {
    type ReturnType = NaiveDate;
    // TODO: Cache set infos ?
    fn get_key(_set_info: &SetInfo) -> Self::ReturnType {
        // set_info
        //     .set_uri()
        //     .fetch()
        //     .unwrap()
        //     .released_at
        //     // Year in which MTG was first released
        //     .unwrap_or(NaiveDate::from_yo_opt(1993, 1))
        NaiveDate::from_yo_opt(1993, 1).expect("invalid or out-of-range date")
    }
}

pub struct SortByCardAmount;
impl SetInfoOrder for SortByCardAmount {
    type ReturnType = i32;

    fn get_key(_set_info: &SetInfo) -> Self::ReturnType {
        panic!("To use SortByCardAmount as a key, use get_combined_key instead.")
    }

    fn get_combined_key(
        _set_info: &SetInfo,
        cards_in_set: &Vec<ScryfallCardWrapper>,
    ) -> Self::ReturnType {
        -(cards_in_set.len() as i32)
    }
}

pub fn sort<P: SetInfoOrder>(cards: CardsBySet) -> SortedCardsBySet {
    P::sort(cards)
}
