use scryfall::set::SetType;

use crate::card_query::CardsBySet;
use crate::setinfo::SetInfo;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Combine {
    Commander,
    MysteryAndTheList,
    DuelDecks,
}

impl Combine {
    fn sets_to_combine<'a>(&'a self, cards_by_set: &'a CardsBySet) -> Vec<&'a SetInfo> {
        cards_by_set
            .keys()
            .filter(|key| match self {
                Combine::Commander => key.set_type() == Some(SetType::Commander),
                Combine::MysteryAndTheList => {
                    key.set_name() == "The List" || key.set_name() == "Mystery Booster"
                }
                Combine::DuelDecks => key.set_type() == Some(SetType::DuelDeck),
            })
            .collect()
    }

    fn combined_set_name(&self) -> String {
        match self {
            Combine::Commander => "Commander (all sets)".to_owned(),
            Combine::MysteryAndTheList => "Mystery & The List".to_owned(),
            Combine::DuelDecks => "Duel Decks (all)".to_owned(),
        }
    }

    pub fn apply(&self, cards_by_set: &CardsBySet) -> CardsBySet {
        let sets_to_combine = self.sets_to_combine(cards_by_set);
        let combined_set_info = SetInfo::create_virtual_set(self.combined_set_name());
        let mut combined = CardsBySet::new();

        for set in &sets_to_combine {
            let cards = cards_by_set.get(&set).unwrap();
            combined
                .entry(combined_set_info.clone())
                .or_insert(Vec::new())
                .extend(cards.to_vec().into_iter());
        }

        for (key, value) in cards_by_set.iter() {
            if !sets_to_combine.contains(&key) {
                combined
                    .entry(key.to_owned())
                    .or_insert(Vec::new())
                    .extend(value.to_vec().into_iter());
            }
        }

        combined.to_owned()
    }
}
