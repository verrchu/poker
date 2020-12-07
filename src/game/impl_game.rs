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
                let mut tmp = hands
                    .iter()
                    .map(|hand| (hand, Self::texas_holdem_combination(*board, *hand)))
                    .collect::<Vec<_>>();

                tmp.sort_by(|(_, comb_a), (_, comb_b)| comb_a.partial_cmp(comb_b).unwrap());

                tmp.into_iter()
                    .map(|(hand, _combination)| hand.0.to_vec())
                    .collect::<Vec<_>>()
            }
            Self::OmahaHoldem(board, hands) => {
                let mut tmp = hands
                    .iter()
                    .map(|hand| (hand, Self::omaha_holdem_combination(*board, *hand)))
                    .collect::<Vec<_>>();

                tmp.sort_by(|(_, comb_a), (_, comb_b)| comb_a.partial_cmp(comb_b).unwrap());

                tmp.into_iter()
                    .map(|(hand, _combination)| hand.0.to_vec())
                    .collect::<Vec<_>>()
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

    fn texas_holdem_combination(_board: Board, _hand: HandOf2) -> Combination {
        unimplemented!()
    }

    fn omaha_holdem_combination(_board: Board, _hand: HandOf4) -> Combination {
        unimplemented!()
    }
}
