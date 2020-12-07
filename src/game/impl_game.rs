use std::convert::TryInto;

use ::itertools::Itertools;

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

    fn texas_holdem_combination(board: Board, hand: HandOf2) -> Combination {
        board
            .0
            .iter()
            .chain(hand.0.iter())
            .copied()
            .combinations(5)
            .map(|comb| Variant(comb.try_into().unwrap()))
            .map(Combination::from_variant)
            .sorted()
            .max()
            .unwrap()
    }

    fn omaha_holdem_combination(board: Board, hand: HandOf4) -> Combination {
        let hand_combinations = hand.0.iter().combinations(2);
        let board_combinations = board.0.iter().combinations(3);

        hand_combinations
            .into_iter()
            .cartesian_product(board_combinations.into_iter())
            .map(|(h, b)| {
                h.into_iter()
                    .chain(b.into_iter())
                    .copied()
                    .collect::<Vec<_>>()
            })
            .map(|cards| Variant(cards.try_into().unwrap()))
            .map(Combination::from_variant)
            .sorted()
            .max()
            .unwrap()
    }
}
