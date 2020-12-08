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
    pub fn ordered_hands(&self) -> Vec<(Vec<Card>, Combination)> {
        match self {
            Self::TexasHoldem(board, hands) => hands
                .iter()
                .map(|hand| {
                    (
                        hand.0.to_vec(),
                        Self::texas_holdem_combination(*board, *hand),
                    )
                })
                .sorted_by(|(_, comb_a), (_, comb_b)| comb_a.cmp(comb_b))
                .collect::<Vec<_>>(),
            Self::OmahaHoldem(board, hands) => hands
                .iter()
                .map(|hand| {
                    (
                        hand.0.to_vec(),
                        Self::omaha_holdem_combination(*board, *hand),
                    )
                })
                .sorted_by(|(_, comb_a), (_, comb_b)| comb_a.cmp(comb_b))
                .collect::<Vec<_>>(),
            Self::FiveCardDraw(hands) => hands
                .iter()
                .map(|hand| (hand, Variant(hand.0)))
                .map(|(hand, variant)| (hand.0.to_vec(), Combination::from_variant(variant)))
                .sorted_by(|(_, comb_a), (_, comb_b)| comb_a.cmp(comb_b))
                .collect::<Vec<_>>(),
        }
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

#[cfg(test)]
mod tests {
    use crate::card::Card;
    use crate::card::Rank;
    use crate::card::Suite;
    use crate::combination::Combination;
    use crate::game::Board;
    use crate::game::Game;
    use crate::game::HandOf2;

    #[test]
    fn test_texas_holdem_ordering() {
        let game = Game::TexasHoldem(
            Board([
                Card(Rank::Queen, Suite::Spades),
                Card(Rank::King, Suite::Diamonds),
                Card(Rank::King, Suite::Spades),
                Card(Rank::Seven, Suite::Clubs),
                Card(Rank::Jack, Suite::Diamonds),
            ]),
            vec![
                HandOf2([
                    Card(Rank::King, Suite::Hearts),
                    Card(Rank::Two, Suite::Clubs),
                ]),
                HandOf2([
                    Card(Rank::King, Suite::Clubs),
                    Card(Rank::Seven, Suite::Diamonds),
                ]),
                HandOf2([
                    Card(Rank::Ace, Suite::Diamonds),
                    Card(Rank::Ten, Suite::Hearts),
                ]),
                HandOf2([
                    Card(Rank::Six, Suite::Diamonds),
                    Card(Rank::Six, Suite::Hearts),
                ]),
            ],
        );

        assert_eq!(
            game.ordered_hands(),
            vec![
                (
                    vec![
                        Card(Rank::Six, Suite::Diamonds),
                        Card(Rank::Six, Suite::Hearts),
                    ],
                    Combination::TwoPairs {
                        low: Rank::Six,
                        high: Rank::King,
                        kicker: Rank::Queen
                    }
                ),
                (
                    vec![
                        Card(Rank::King, Suite::Hearts),
                        Card(Rank::Two, Suite::Clubs),
                    ],
                    Combination::ThreeOfAKind {
                        rank: Rank::King,
                        kicker: Rank::Queen
                    }
                ),
                (
                    vec![
                        Card(Rank::Ace, Suite::Diamonds),
                        Card(Rank::Ten, Suite::Hearts),
                    ],
                    Combination::Straight { rank: Rank::Ten }
                ),
                (
                    vec![
                        Card(Rank::King, Suite::Clubs),
                        Card(Rank::Seven, Suite::Diamonds),
                    ],
                    Combination::FullHouse {
                        two: Rank::Seven,
                        three: Rank::King
                    }
                )
            ]
        );
    }
}
