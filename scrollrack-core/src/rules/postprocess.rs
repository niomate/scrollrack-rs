use scryfall::set::SetType;

use crate::card_query::CardsBySet;
use crate::setinfo::SetInfo;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum PostProcessRule {
    CombineCommanderSets,
}

impl PostProcessRule {
    pub fn apply(&self, cards_by_set: CardsBySet) -> CardsBySet {
        match self {
            PostProcessRule::CombineCommanderSets => {
                let mut combined = CardsBySet::new();
                let combined_set_info = SetInfo::create_virtual_set("Commander (all sets)");

                let sets_to_combine = cards_by_set
                    .keys()
                    .filter(|key| key.set_type() == Some(SetType::Commander))
                    .collect::<Vec<_>>();

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
    }
}
