use std::io::BufRead;
use std::str::FromStr;

mod card;
mod combination;
mod game;

fn main() {
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        process(&line.unwrap())
    }
}

fn process(line: &str) {
    let game = game::Game::from_str(line).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::card::Card;
    use crate::card::Rank;
    use crate::card::Suite;
    use crate::combination::Combination;
    use crate::game::Variant;

    #[test]
    fn test_combination_high_card_ordering() {
        let lhs = Combination::HighCard { rank: Rank::Two };

        assert!(lhs == Combination::HighCard { rank: Rank::Two });
        assert!(lhs < Combination::HighCard { rank: Rank::Three });
        assert!(
            lhs < Combination::Pair {
                rank: Rank::Two,
                kicker: Rank::Three
            }
        );
        assert!(
            lhs < Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                kicker: Rank::Four,
            }
        );
        assert!(
            lhs < Combination::ThreeOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs < Combination::Straight { rank: Rank::Ace });
        assert!(lhs < Combination::Flush { rank: Rank::Ace });
        assert!(
            lhs < Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert!(
            lhs < Combination::FourOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs < Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_combination_pair_ordering() {
        let lhs = Combination::Pair {
            rank: Rank::Two,
            kicker: Rank::Three,
        };

        assert!(lhs > Combination::HighCard { rank: Rank::Two });
        assert!(
            lhs == Combination::Pair {
                rank: Rank::Two,
                kicker: Rank::Three
            }
        );
        assert!(
            lhs < Combination::Pair {
                rank: Rank::Two,
                kicker: Rank::Four
            }
        );
        assert!(
            lhs < Combination::Pair {
                rank: Rank::Four,
                kicker: Rank::Two
            }
        );
        assert!(
            lhs < Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                kicker: Rank::Four,
            }
        );
        assert!(
            lhs < Combination::ThreeOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs < Combination::Straight { rank: Rank::Ace });
        assert!(lhs < Combination::Flush { rank: Rank::Ace });
        assert!(
            lhs < Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert!(
            lhs < Combination::FourOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs < Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_combination_two_pairs_ordering() {
        let lhs = Combination::TwoPairs {
            low: Rank::Two,
            high: Rank::Three,
            kicker: Rank::Four,
        };

        assert!(lhs > Combination::HighCard { rank: Rank::Two });
        assert!(
            lhs > Combination::Pair {
                rank: Rank::Two,
                kicker: Rank::Three
            }
        );
        assert!(
            lhs == Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                kicker: Rank::Four
            }
        );
        assert!(
            lhs < Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Five,
                kicker: Rank::Four
            }
        );
        assert!(
            lhs < Combination::TwoPairs {
                low: Rank::Five,
                high: Rank::Three,
                kicker: Rank::Four
            }
        );
        assert!(
            lhs < Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                kicker: Rank::Five
            }
        );
        assert!(
            lhs < Combination::ThreeOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs < Combination::Straight { rank: Rank::Ace });
        assert!(lhs < Combination::Flush { rank: Rank::Ace });
        assert!(
            lhs < Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert!(
            lhs < Combination::FourOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs < Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_combination_three_of_a_king_ordering() {
        let lhs = Combination::ThreeOfAKind {
            rank: Rank::Two,
            kicker: Rank::Three,
        };

        assert!(lhs > Combination::HighCard { rank: Rank::Two });
        assert!(
            lhs > Combination::Pair {
                rank: Rank::Two,
                kicker: Rank::Three
            }
        );
        assert!(
            lhs > Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                kicker: Rank::Four
            }
        );
        assert!(
            lhs == Combination::ThreeOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(
            lhs < Combination::ThreeOfAKind {
                rank: Rank::Four,
                kicker: Rank::Three,
            }
        );
        assert!(
            lhs < Combination::ThreeOfAKind {
                rank: Rank::Two,
                kicker: Rank::Four,
            }
        );
        assert!(lhs < Combination::Straight { rank: Rank::Ace });
        assert!(lhs < Combination::Flush { rank: Rank::Ace });
        assert!(
            lhs < Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert!(
            lhs < Combination::FourOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs < Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_combination_straight_ordering() {
        let lhs = Combination::Straight { rank: Rank::Ace };

        assert!(lhs > Combination::HighCard { rank: Rank::Two });
        assert!(
            lhs > Combination::Pair {
                rank: Rank::Two,
                kicker: Rank::Three
            }
        );
        assert!(
            lhs > Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                kicker: Rank::Four
            }
        );
        assert!(
            lhs > Combination::ThreeOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs == Combination::Straight { rank: Rank::Ace });
        assert!(lhs < Combination::Straight { rank: Rank::Two });
        assert!(lhs < Combination::Flush { rank: Rank::Ace });
        assert!(
            lhs < Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert!(
            lhs < Combination::FourOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs < Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_combination_flush_ordering() {
        let lhs = Combination::Flush { rank: Rank::Ace };

        assert!(lhs > Combination::HighCard { rank: Rank::Two });
        assert!(
            lhs > Combination::Pair {
                rank: Rank::Two,
                kicker: Rank::Three
            }
        );
        assert!(
            lhs > Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                kicker: Rank::Four
            }
        );
        assert!(
            lhs > Combination::ThreeOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs > Combination::Straight { rank: Rank::Ace });
        assert!(lhs == Combination::Flush { rank: Rank::Ace });
        assert!(lhs > Combination::Flush { rank: Rank::Two });
        assert!(
            lhs < Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert!(
            lhs < Combination::FourOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs < Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_combination_full_house_ordering() {
        let lhs = Combination::FullHouse {
            two: Rank::Two,
            three: Rank::Three,
        };

        assert!(lhs > Combination::HighCard { rank: Rank::Two });
        assert!(
            lhs > Combination::Pair {
                rank: Rank::Two,
                kicker: Rank::Three
            }
        );
        assert!(
            lhs > Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                kicker: Rank::Four
            }
        );
        assert!(
            lhs > Combination::ThreeOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs > Combination::Straight { rank: Rank::Ace });
        assert!(lhs > Combination::Flush { rank: Rank::Ace });
        assert!(
            lhs == Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert!(
            lhs < Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Four,
            }
        );
        assert!(
            lhs < Combination::FullHouse {
                two: Rank::Four,
                three: Rank::Three,
            }
        );
        assert!(
            lhs < Combination::FourOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs < Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_combination_four_of_a_kind_ordering() {
        let lhs = Combination::FourOfAKind {
            rank: Rank::Two,
            kicker: Rank::Three,
        };

        assert!(lhs > Combination::HighCard { rank: Rank::Two });
        assert!(
            lhs > Combination::Pair {
                rank: Rank::Two,
                kicker: Rank::Three
            }
        );
        assert!(
            lhs > Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                kicker: Rank::Four
            }
        );
        assert!(
            lhs > Combination::ThreeOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs > Combination::Straight { rank: Rank::Ace });
        assert!(lhs > Combination::Flush { rank: Rank::Ace });
        assert!(
            lhs > Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert!(
            lhs == Combination::FourOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(
            lhs < Combination::FourOfAKind {
                rank: Rank::Four,
                kicker: Rank::Three,
            }
        );
        assert!(
            lhs < Combination::FourOfAKind {
                rank: Rank::Two,
                kicker: Rank::Four,
            }
        );
        assert!(lhs < Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_combination_straight_flush_ordering() {
        let lhs = Combination::StraightFlush { rank: Rank::Ace };

        assert!(lhs > Combination::HighCard { rank: Rank::Two });
        assert!(
            lhs > Combination::Pair {
                rank: Rank::Two,
                kicker: Rank::Three
            }
        );
        assert!(
            lhs > Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                kicker: Rank::Four
            }
        );
        assert!(
            lhs > Combination::ThreeOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs > Combination::Straight { rank: Rank::Ace });
        assert!(lhs > Combination::Flush { rank: Rank::Ace });
        assert!(
            lhs > Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            }
        );
        assert!(
            lhs > Combination::FourOfAKind {
                rank: Rank::Two,
                kicker: Rank::Three,
            }
        );
        assert!(lhs == Combination::StraightFlush { rank: Rank::Ace });
        assert!(lhs < Combination::StraightFlush { rank: Rank::Two });
    }

    #[test]
    fn test_flush_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Jack, Suite::Diamonds),
            Card(Rank::Ace, Suite::Diamonds),
            Card(Rank::Seven, Suite::Diamonds),
            Card(Rank::Eight, Suite::Diamonds),
        ]);

        let result = Combination::try_flush(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::Flush { rank: Rank::Ace });
    }

    #[test]
    fn test_flush_from_variant_negative() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Jack, Suite::Diamonds),
            Card(Rank::Ace, Suite::Spades),
            Card(Rank::Seven, Suite::Diamonds),
            Card(Rank::Eight, Suite::Diamonds),
        ]);

        let result = Combination::try_flush(variant);

        assert!(result.is_none());
    }

    #[test]
    fn test_high_card_from_variant() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Jack, Suite::Diamonds),
            Card(Rank::Ace, Suite::Spades),
            Card(Rank::Seven, Suite::Diamonds),
            Card(Rank::Eight, Suite::Diamonds),
        ]);

        let result = Combination::try_high_card(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::HighCard { rank: Rank::Ace });
    }

    #[test]
    fn test_pair_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Jack, Suite::Diamonds),
            Card(Rank::Jack, Suite::Spades),
            Card(Rank::Seven, Suite::Diamonds),
            Card(Rank::Eight, Suite::Diamonds),
        ]);

        let result = Combination::try_pair(variant);

        assert!(result.is_some());
        assert!(
            result.unwrap()
                == Combination::Pair {
                    rank: Rank::Jack,
                    kicker: Rank::Eight
                }
        );
    }

    #[test]
    fn test_pair_from_variant_negative() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Jack, Suite::Diamonds),
            Card(Rank::Queen, Suite::Spades),
            Card(Rank::Seven, Suite::Diamonds),
            Card(Rank::Eight, Suite::Diamonds),
        ]);

        let result = Combination::try_pair(variant);

        assert!(result.is_none());
    }

    #[test]
    fn test_three_of_a_kind_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Jack, Suite::Diamonds),
            Card(Rank::Jack, Suite::Spades),
            Card(Rank::Jack, Suite::Hearts),
            Card(Rank::Eight, Suite::Diamonds),
        ]);

        let result = Combination::try_three_of_a_kind(variant);

        assert!(result.is_some());
        assert!(
            result.unwrap()
                == Combination::ThreeOfAKind {
                    rank: Rank::Jack,
                    kicker: Rank::Eight
                }
        );
    }

    #[test]
    fn test_three_of_a_kind_from_variant_negative() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Jack, Suite::Diamonds),
            Card(Rank::Jack, Suite::Spades),
            Card(Rank::Seven, Suite::Diamonds),
            Card(Rank::Eight, Suite::Diamonds),
        ]);

        let result = Combination::try_three_of_a_kind(variant);

        assert!(result.is_none());
    }

    #[test]
    fn test_four_of_a_kind_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Jack, Suite::Diamonds),
            Card(Rank::Jack, Suite::Spades),
            Card(Rank::Jack, Suite::Hearts),
            Card(Rank::Jack, Suite::Clubs),
        ]);

        let result = Combination::try_four_of_a_kind(variant);

        assert!(result.is_some());
        assert!(
            result.unwrap()
                == Combination::FourOfAKind {
                    rank: Rank::Jack,
                    kicker: Rank::Two
                }
        );
    }

    #[test]
    fn test_four_of_a_kind_from_variant_negative() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Jack, Suite::Diamonds),
            Card(Rank::Jack, Suite::Spades),
            Card(Rank::Jack, Suite::Hearts),
            Card(Rank::Eight, Suite::Diamonds),
        ]);

        let result = Combination::try_four_of_a_kind(variant);

        assert!(result.is_none());
    }

    #[test]
    fn test_full_house_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Two, Suite::Hearts),
            Card(Rank::Jack, Suite::Spades),
            Card(Rank::Jack, Suite::Hearts),
            Card(Rank::Jack, Suite::Clubs),
        ]);

        let result = Combination::try_full_house(variant);

        assert!(result.is_some());
        assert!(
            result.unwrap()
                == Combination::FullHouse {
                    three: Rank::Jack,
                    two: Rank::Two
                }
        );
    }

    #[test]
    fn test_full_house_from_variant_negative() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Two, Suite::Hearts),
            Card(Rank::Three, Suite::Spades),
            Card(Rank::Jack, Suite::Hearts),
            Card(Rank::Jack, Suite::Clubs),
        ]);

        let result = Combination::try_full_house(variant);

        assert!(result.is_none());
    }

    #[test]
    fn test_two_pairs_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Two, Suite::Hearts),
            Card(Rank::Jack, Suite::Spades),
            Card(Rank::Jack, Suite::Hearts),
            Card(Rank::Queen, Suite::Clubs),
        ]);

        let result = Combination::try_two_pairs(variant);

        assert!(result.is_some());
        assert!(
            result.unwrap()
                == Combination::TwoPairs {
                    low: Rank::Two,
                    high: Rank::Jack,
                    kicker: Rank::Queen
                }
        );
    }

    #[test]
    fn test_two_pairs_from_variant_negative() {
        let variant = Variant([
            Card(Rank::Two, Suite::Diamonds),
            Card(Rank::Two, Suite::Hearts),
            Card(Rank::Three, Suite::Spades),
            Card(Rank::Four, Suite::Hearts),
            Card(Rank::Jack, Suite::Clubs),
        ]);

        let result = Combination::try_two_pairs(variant);

        assert!(result.is_none());
    }

    #[test]
    fn test_straight_ace_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Ace, Suite::Diamonds),
            Card(Rank::Two, Suite::Hearts),
            Card(Rank::Three, Suite::Spades),
            Card(Rank::Four, Suite::Hearts),
            Card(Rank::Five, Suite::Clubs),
        ]);

        let result = Combination::try_straight(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::Straight { rank: Rank::Ace });
    }

    #[test]
    fn test_straight_two_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Two, Suite::Hearts),
            Card(Rank::Three, Suite::Spades),
            Card(Rank::Four, Suite::Hearts),
            Card(Rank::Five, Suite::Clubs),
            Card(Rank::Six, Suite::Diamonds),
        ]);

        let result = Combination::try_straight(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::Straight { rank: Rank::Two });
    }

    #[test]
    fn test_straight_three_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Three, Suite::Spades),
            Card(Rank::Four, Suite::Hearts),
            Card(Rank::Five, Suite::Clubs),
            Card(Rank::Six, Suite::Diamonds),
            Card(Rank::Seven, Suite::Hearts),
        ]);

        let result = Combination::try_straight(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::Straight { rank: Rank::Three });
    }

    #[test]
    fn test_straight_four_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Four, Suite::Hearts),
            Card(Rank::Five, Suite::Clubs),
            Card(Rank::Six, Suite::Diamonds),
            Card(Rank::Seven, Suite::Hearts),
            Card(Rank::Eight, Suite::Spades),
        ]);

        let result = Combination::try_straight(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::Straight { rank: Rank::Four });
    }

    #[test]
    fn test_straight_five_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Five, Suite::Clubs),
            Card(Rank::Six, Suite::Diamonds),
            Card(Rank::Seven, Suite::Hearts),
            Card(Rank::Eight, Suite::Spades),
            Card(Rank::Nine, Suite::Hearts),
        ]);

        let result = Combination::try_straight(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::Straight { rank: Rank::Five });
    }

    #[test]
    fn test_straight_six_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Six, Suite::Diamonds),
            Card(Rank::Seven, Suite::Hearts),
            Card(Rank::Eight, Suite::Spades),
            Card(Rank::Nine, Suite::Hearts),
            Card(Rank::Ten, Suite::Clubs),
        ]);

        let result = Combination::try_straight(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::Straight { rank: Rank::Six });
    }

    #[test]
    fn test_straight_seven_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Seven, Suite::Hearts),
            Card(Rank::Eight, Suite::Spades),
            Card(Rank::Nine, Suite::Hearts),
            Card(Rank::Ten, Suite::Clubs),
            Card(Rank::Jack, Suite::Diamonds),
        ]);

        let result = Combination::try_straight(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::Straight { rank: Rank::Seven });
    }

    #[test]
    fn test_straight_eight_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Eight, Suite::Spades),
            Card(Rank::Nine, Suite::Hearts),
            Card(Rank::Ten, Suite::Clubs),
            Card(Rank::Jack, Suite::Diamonds),
            Card(Rank::Queen, Suite::Hearts),
        ]);

        let result = Combination::try_straight(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::Straight { rank: Rank::Eight });
    }

    #[test]
    fn test_straight_nine_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Nine, Suite::Hearts),
            Card(Rank::Ten, Suite::Clubs),
            Card(Rank::Jack, Suite::Diamonds),
            Card(Rank::Queen, Suite::Hearts),
            Card(Rank::King, Suite::Spades),
        ]);

        let result = Combination::try_straight(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::Straight { rank: Rank::Nine });
    }

    #[test]
    fn test_straight_ten_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Ten, Suite::Clubs),
            Card(Rank::Jack, Suite::Diamonds),
            Card(Rank::Queen, Suite::Hearts),
            Card(Rank::King, Suite::Spades),
            Card(Rank::Ace, Suite::Hearts),
        ]);

        let result = Combination::try_straight(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::Straight { rank: Rank::Ten });
    }

    #[test]
    fn test_straight_from_variant_negative() {
        let variant = Variant([
            Card(Rank::Ten, Suite::Clubs),
            Card(Rank::King, Suite::Diamonds),
            Card(Rank::Queen, Suite::Hearts),
            Card(Rank::King, Suite::Spades),
            Card(Rank::Seven, Suite::Hearts),
        ]);

        let result = Combination::try_straight(variant);

        assert!(result.is_none());
    }

    #[test]
    fn test_straight_flush_ace_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Ace, Suite::Hearts),
            Card(Rank::Two, Suite::Hearts),
            Card(Rank::Three, Suite::Hearts),
            Card(Rank::Four, Suite::Hearts),
            Card(Rank::Five, Suite::Hearts),
        ]);

        let result = Combination::try_straight_flush(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::StraightFlush { rank: Rank::Ace });
    }

    #[test]
    fn test_straight_flush_two_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Two, Suite::Hearts),
            Card(Rank::Three, Suite::Hearts),
            Card(Rank::Four, Suite::Hearts),
            Card(Rank::Five, Suite::Hearts),
            Card(Rank::Six, Suite::Hearts),
        ]);

        let result = Combination::try_straight_flush(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::StraightFlush { rank: Rank::Two });
    }

    #[test]
    fn test_straight_flush_three_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Three, Suite::Hearts),
            Card(Rank::Four, Suite::Hearts),
            Card(Rank::Five, Suite::Hearts),
            Card(Rank::Six, Suite::Hearts),
            Card(Rank::Seven, Suite::Hearts),
        ]);

        let result = Combination::try_straight_flush(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::StraightFlush { rank: Rank::Three });
    }

    #[test]
    fn test_straight_flush_four_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Four, Suite::Hearts),
            Card(Rank::Five, Suite::Hearts),
            Card(Rank::Six, Suite::Hearts),
            Card(Rank::Seven, Suite::Hearts),
            Card(Rank::Eight, Suite::Hearts),
        ]);

        let result = Combination::try_straight_flush(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::StraightFlush { rank: Rank::Four });
    }

    #[test]
    fn test_straight_flush_five_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Five, Suite::Hearts),
            Card(Rank::Six, Suite::Hearts),
            Card(Rank::Seven, Suite::Hearts),
            Card(Rank::Eight, Suite::Hearts),
            Card(Rank::Nine, Suite::Hearts),
        ]);

        let result = Combination::try_straight_flush(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::StraightFlush { rank: Rank::Five });
    }

    #[test]
    fn test_straight_flush_six_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Six, Suite::Hearts),
            Card(Rank::Seven, Suite::Hearts),
            Card(Rank::Eight, Suite::Hearts),
            Card(Rank::Nine, Suite::Hearts),
            Card(Rank::Ten, Suite::Hearts),
        ]);

        let result = Combination::try_straight_flush(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::StraightFlush { rank: Rank::Six });
    }

    #[test]
    fn test_straight_flush_seven_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Seven, Suite::Hearts),
            Card(Rank::Eight, Suite::Hearts),
            Card(Rank::Nine, Suite::Hearts),
            Card(Rank::Ten, Suite::Hearts),
            Card(Rank::Jack, Suite::Hearts),
        ]);

        let result = Combination::try_straight_flush(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::StraightFlush { rank: Rank::Seven });
    }

    #[test]
    fn test_straight_flush_eight_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Eight, Suite::Hearts),
            Card(Rank::Nine, Suite::Hearts),
            Card(Rank::Ten, Suite::Hearts),
            Card(Rank::Jack, Suite::Hearts),
            Card(Rank::Queen, Suite::Hearts),
        ]);

        let result = Combination::try_straight_flush(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::StraightFlush { rank: Rank::Eight });
    }

    #[test]
    fn test_straight_flush_nine_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Nine, Suite::Hearts),
            Card(Rank::Ten, Suite::Hearts),
            Card(Rank::Jack, Suite::Hearts),
            Card(Rank::Queen, Suite::Hearts),
            Card(Rank::King, Suite::Hearts),
        ]);

        let result = Combination::try_straight_flush(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::StraightFlush { rank: Rank::Nine });
    }

    #[test]
    fn test_straight_flush_ten_from_variant_positive() {
        let variant = Variant([
            Card(Rank::Ten, Suite::Hearts),
            Card(Rank::Jack, Suite::Hearts),
            Card(Rank::Queen, Suite::Hearts),
            Card(Rank::King, Suite::Hearts),
            Card(Rank::Ace, Suite::Hearts),
        ]);

        let result = Combination::try_straight_flush(variant);

        assert!(result.is_some());
        assert!(result.unwrap() == Combination::StraightFlush { rank: Rank::Ten });
    }

    #[test]
    fn test_straight_flush_from_variant_negative() {
        let variant = Variant([
            Card(Rank::Ten, Suite::Hearts),
            Card(Rank::King, Suite::Hearts),
            Card(Rank::Queen, Suite::Hearts),
            Card(Rank::King, Suite::Hearts),
            Card(Rank::Seven, Suite::Hearts),
        ]);

        let result = Combination::try_straight_flush(variant);

        assert!(result.is_none());
    }
}
