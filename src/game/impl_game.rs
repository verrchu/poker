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
    pub fn sort_hands(&self) -> Vec<(Vec<Card>, Combination)> {
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
    use crate::card::Suit;
    use crate::combination::Combination;
    use crate::game::Board;
    use crate::game::Game;
    use crate::game::HandOf2;
    use crate::game::HandOf4;
    use crate::game::HandOf5;

    #[test]
    fn test_texas_holdem_ordering() {
        let game = Game::TexasHoldem(
            Board([
                Card(Rank::Queen, Suit::Spades),
                Card(Rank::King, Suit::Diamonds),
                Card(Rank::King, Suit::Spades),
                Card(Rank::Seven, Suit::Clubs),
                Card(Rank::Jack, Suit::Diamonds),
            ]),
            vec![
                HandOf2([Card(Rank::King, Suit::Hearts), Card(Rank::Two, Suit::Clubs)]),
                HandOf2([
                    Card(Rank::King, Suit::Clubs),
                    Card(Rank::Seven, Suit::Diamonds),
                ]),
                HandOf2([
                    Card(Rank::Ace, Suit::Diamonds),
                    Card(Rank::Ten, Suit::Hearts),
                ]),
                HandOf2([
                    Card(Rank::Six, Suit::Diamonds),
                    Card(Rank::Six, Suit::Hearts),
                ]),
            ],
        );

        assert_eq!(
            game.sort_hands(),
            vec![
                (
                    vec![
                        Card(Rank::Six, Suit::Diamonds),
                        Card(Rank::Six, Suit::Hearts),
                    ],
                    Combination::TwoPairs {
                        low: Rank::Six,
                        high: Rank::King,
                        kicker: Rank::Queen
                    }
                ),
                (
                    vec![Card(Rank::King, Suit::Hearts), Card(Rank::Two, Suit::Clubs),],
                    Combination::ThreeOfAKind {
                        rank: Rank::King,
                        kicker: Rank::Queen
                    }
                ),
                (
                    vec![
                        Card(Rank::Ace, Suit::Diamonds),
                        Card(Rank::Ten, Suit::Hearts),
                    ],
                    Combination::Straight { rank: Rank::Ten }
                ),
                (
                    vec![
                        Card(Rank::King, Suit::Clubs),
                        Card(Rank::Seven, Suit::Diamonds),
                    ],
                    Combination::FullHouse {
                        two: Rank::Seven,
                        three: Rank::King
                    }
                )
            ]
        );
    }

    #[test]
    fn test_omaha_holdem_ordering() {
        let game = Game::OmahaHoldem(
            Board([
                Card(Rank::Queen, Suit::Spades),
                Card(Rank::King, Suit::Diamonds),
                Card(Rank::King, Suit::Spades),
                Card(Rank::Seven, Suit::Clubs),
                Card(Rank::Jack, Suit::Diamonds),
            ]),
            vec![
                HandOf4([
                    Card(Rank::King, Suit::Hearts),
                    Card(Rank::Two, Suit::Clubs),
                    Card(Rank::Eight, Suit::Clubs),
                    Card(Rank::Two, Suit::Diamonds),
                ]),
                HandOf4([
                    Card(Rank::King, Suit::Clubs),
                    Card(Rank::Seven, Suit::Diamonds),
                    Card(Rank::Seven, Suit::Hearts),
                    Card(Rank::Seven, Suit::Spades),
                ]),
                HandOf4([
                    Card(Rank::Ace, Suit::Diamonds),
                    Card(Rank::Ten, Suit::Hearts),
                    Card(Rank::Ace, Suit::Clubs),
                    Card(Rank::Ten, Suit::Clubs),
                ]),
            ],
        );

        assert_eq!(
            game.sort_hands(),
            vec![
                (
                    vec![
                        Card(Rank::King, Suit::Hearts),
                        Card(Rank::Two, Suit::Clubs),
                        Card(Rank::Eight, Suit::Clubs),
                        Card(Rank::Two, Suit::Diamonds),
                    ],
                    Combination::ThreeOfAKind {
                        rank: Rank::King,
                        kicker: Rank::Queen
                    }
                ),
                (
                    vec![
                        Card(Rank::Ace, Suit::Diamonds),
                        Card(Rank::Ten, Suit::Hearts),
                        Card(Rank::Ace, Suit::Clubs),
                        Card(Rank::Ten, Suit::Clubs),
                    ],
                    Combination::Straight { rank: Rank::Ten }
                ),
                (
                    vec![
                        Card(Rank::King, Suit::Clubs),
                        Card(Rank::Seven, Suit::Diamonds),
                        Card(Rank::Seven, Suit::Hearts),
                        Card(Rank::Seven, Suit::Spades),
                    ],
                    Combination::FullHouse {
                        two: Rank::Seven,
                        three: Rank::King
                    }
                )
            ]
        );
    }

    #[test]
    fn test_five_card_draw_ordering() {
        let game = Game::FiveCardDraw(vec![
            HandOf5([
                Card(Rank::King, Suit::Hearts),
                Card(Rank::Two, Suit::Clubs),
                Card(Rank::Eight, Suit::Clubs),
                Card(Rank::Two, Suit::Diamonds),
                Card(Rank::Two, Suit::Hearts),
            ]),
            HandOf5([
                Card(Rank::King, Suit::Clubs),
                Card(Rank::Seven, Suit::Diamonds),
                Card(Rank::Seven, Suit::Hearts),
                Card(Rank::Seven, Suit::Spades),
                Card(Rank::Seven, Suit::Clubs),
            ]),
            HandOf5([
                Card(Rank::Ace, Suit::Diamonds),
                Card(Rank::Ten, Suit::Hearts),
                Card(Rank::Ace, Suit::Clubs),
                Card(Rank::Ten, Suit::Clubs),
                Card(Rank::Ten, Suit::Diamonds),
            ]),
        ]);

        assert_eq!(
            game.sort_hands(),
            vec![
                (
                    vec![
                        Card(Rank::King, Suit::Hearts),
                        Card(Rank::Two, Suit::Clubs),
                        Card(Rank::Eight, Suit::Clubs),
                        Card(Rank::Two, Suit::Diamonds),
                        Card(Rank::Two, Suit::Hearts),
                    ],
                    Combination::ThreeOfAKind {
                        rank: Rank::Two,
                        kicker: Rank::King
                    }
                ),
                (
                    vec![
                        Card(Rank::Ace, Suit::Diamonds),
                        Card(Rank::Ten, Suit::Hearts),
                        Card(Rank::Ace, Suit::Clubs),
                        Card(Rank::Ten, Suit::Clubs),
                        Card(Rank::Ten, Suit::Diamonds),
                    ],
                    Combination::FullHouse {
                        two: Rank::Ace,
                        three: Rank::Ten
                    }
                ),
                (
                    vec![
                        Card(Rank::King, Suit::Clubs),
                        Card(Rank::Seven, Suit::Diamonds),
                        Card(Rank::Seven, Suit::Hearts),
                        Card(Rank::Seven, Suit::Spades),
                        Card(Rank::Seven, Suit::Clubs),
                    ],
                    Combination::FourOfAKind {
                        rank: Rank::Seven,
                        kicker: Rank::King
                    }
                )
            ]
        );
    }
}
