use itertools::Itertools;
use scrollrack_core::card_query::CardQuery;
use scrollrack_core::parse;

use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let lines = parse::read_lines("docs/affinity.txt").unwrap();

    let cards_by_set = CardQuery::default().run(parse::parse_card_infos(lines));

    let cards_sorted = cards_by_set
        .keys()
        .sorted_by_key(|k| k.set_name())
        .map(|k| (k, cards_by_set.get(k).unwrap()))
        .collect::<Vec<_>>();

    cx.render(rsx!(
            ul {
                cards_sorted.iter().map(|(set, cards)| {
                    rsx!( li { "{set.set_name()}" ul {
                        cards.iter().map(|card| rsx!( li { "{card.format_detailed()}" }))
                    } } )
                })
            }
    ))
}
