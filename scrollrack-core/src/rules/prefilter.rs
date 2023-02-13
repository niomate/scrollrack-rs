use scryfall::card;
use scryfall::set;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum PrefilterRule {
    IsPaper,
    NoPromo,
    NoGiftBox,
    NoMysteryBoosterRetailEditionFoil,
    NoMasterpiece,
}

impl PrefilterRule {
    pub fn check(&self, card: &card::Card) -> bool {
        match self {
            PrefilterRule::IsPaper => card.games.contains(&card::Game::Paper),
            PrefilterRule::NoPromo => !card.promo,
            PrefilterRule::NoGiftBox => card.set_type != set::SetType::GiftBox,
            PrefilterRule::NoMysteryBoosterRetailEditionFoil => {
                card.set_name != "Mystery Booster Retail Edition Foils"
            }
            PrefilterRule::NoMasterpiece => card.set_type != set::SetType::Masterpiece,
        }
    }
}
