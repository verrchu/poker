use crate::card::Card;
use crate::combination::Combination;
use crate::game::Board;
use crate::game::Game;
use crate::game::HandOf2;
use crate::game::HandOf4;
use crate::game::Variant;

impl Game {
    pub fn ordered_hands(&self) -> Vec<Vec<Card>> {
        match self {
            Self::TexasHoldem(board, hands) => {
                let mapping = hands
                    .iter()
                    .map(|hand| {
                        (
                            hand.0.to_vec(),
                            Self::texas_holdem_combination(*board, *hand),
                        )
                    })
                    .collect::<Vec<_>>();

                Self::sort_hands(mapping)
            }
            Self::OmahaHoldem(board, hands) => {
                let mapping = hands
                    .iter()
                    .map(|hand| {
                        (
                            hand.0.to_vec(),
                            Self::omaha_holdem_combination(*board, *hand),
                        )
                    })
                    .collect::<Vec<_>>();

                Self::sort_hands(mapping)
            }
            Self::FiveCardDraw(hands) => {
                let mapping = hands
                    .iter()
                    .map(|hand| (hand, Variant(hand.0)))
                    .map(|(hand, variant)| (hand.0.to_vec(), Combination::from_variant(variant)))
                    .collect::<Vec<_>>();

                Self::sort_hands(mapping)
            }
        }
    }

    fn sort_hands(mut mapping: Vec<(Vec<Card>, Combination)>) -> Vec<Vec<Card>> {
        mapping.sort_by(|(_, comb_a), (_, comb_b)| comb_a.partial_cmp(comb_b).unwrap());
        mapping
            .into_iter()
            .map(|(hand, _combination)| hand)
            .collect::<Vec<_>>()
    }

    fn texas_holdem_combination(_board: Board, _hand: HandOf2) -> Combination {
        unimplemented!()
    }

    fn omaha_holdem_combination(_board: Board, _hand: HandOf4) -> Combination {
        unimplemented!()
    }
}
