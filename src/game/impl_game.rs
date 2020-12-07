use crate::card::Card;
use crate::combination::Combination;
use crate::game::Game;
use crate::game::Variant;

impl Game {
    pub fn ordered_hands(&self) -> Vec<Vec<Card>> {
        match self {
            Self::TexasHoldem(_board, _hands) => {
                unimplemented!()
            }
            Self::OmahaHoldem(_board, _hands) => {
                unimplemented!()
            }
            Self::FiveCardDraw(hands) => {
                let mut tmp = hands
                    .iter()
                    .map(|hand| (hand, Variant(hand.0)))
                    .map(|(hand, variant)| (hand, Combination::from_variant(variant)))
                    .collect::<Vec<_>>();

                tmp.sort_by(|(_, comb_a), (_, comb_b)| comb_a.partial_cmp(comb_b).unwrap());

                tmp.into_iter()
                    .map(|(hand, _combination)| hand.0.to_vec())
                    .collect::<Vec<_>>()
            }
        }
    }
}
