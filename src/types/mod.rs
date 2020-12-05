pub mod card;
pub mod combination;
pub mod game;

pub use combination::Combination;

pub use card::Card;
pub use card::Rank;
pub use card::Suite;

pub use game::Board;
pub use game::Game;
pub use game::HandOf2;
pub use game::HandOf4;
pub use game::HandOf5;

#[cfg(test)]
mod tests {
    use super::Combination;
    use super::Rank;

    #[test]
    fn test_card_rank_ordering() {
        assert!(Rank::Three > Rank::Two);
        assert!(Rank::Four > Rank::Three);
        assert!(Rank::Five > Rank::Four);
        assert!(Rank::Six > Rank::Five);
        assert!(Rank::Seven > Rank::Six);
        assert!(Rank::Eight > Rank::Seven);
        assert!(Rank::Nine > Rank::Eight);
        assert!(Rank::Ten > Rank::Nine);
        assert!(Rank::Jack > Rank::Ten);
        assert!(Rank::Queen > Rank::Jack);
        assert!(Rank::King > Rank::Queen);
        assert!(Rank::Ace > Rank::King);
    }

    #[test]
    fn test_card_rank_equality() {
        assert!(Rank::Three == Rank::Three);
        assert!(Rank::Four == Rank::Four);
        assert!(Rank::Five == Rank::Five);
        assert!(Rank::Six == Rank::Six);
        assert!(Rank::Seven == Rank::Seven);
        assert!(Rank::Eight == Rank::Eight);
        assert!(Rank::Nine == Rank::Nine);
        assert!(Rank::Ten == Rank::Ten);
        assert!(Rank::Jack == Rank::Jack);
        assert!(Rank::Queen == Rank::Queen);
        assert!(Rank::King == Rank::King);
        assert!(Rank::Ace == Rank::Ace);
    }

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
}
