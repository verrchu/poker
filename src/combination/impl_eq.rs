use std::mem;

use crate::card::Rank;
use crate::combination::Combination;

use ::itertools::Itertools;

impl PartialEq for Combination {
    fn eq(&self, other: &Self) -> bool {
        if mem::discriminant(self) != mem::discriminant(other) {
            false
        } else {
            match self {
                Self::HighCard { rank: rank_a } => {
                    if let Self::HighCard { rank: rank_b } = other {
                        rank_a == rank_b
                    } else {
                        false
                    }
                }
                Self::Pair {
                    rank: rank_a,
                    extra: extra_a,
                } => {
                    if let Self::Pair {
                        rank: rank_b,
                        extra: extra_b,
                    } = other
                    {
                        rank_a == rank_b && compare_extra(extra_a, extra_b)
                    } else {
                        false
                    }
                }
                Self::TwoPairs {
                    low: low_pair_rank_a,
                    high: high_pair_rank_a,
                    extra: extra_a,
                } => {
                    if let Self::TwoPairs {
                        low: low_pair_rank_b,
                        high: high_pair_rank_b,
                        extra: extra_b,
                    } = other
                    {
                        high_pair_rank_a == high_pair_rank_b
                            && low_pair_rank_a == low_pair_rank_b
                            && compare_extra(extra_a, extra_b)
                    } else {
                        false
                    }
                }
                Self::ThreeOfAKind {
                    rank: rank_a,
                    extra: extra_a,
                } => {
                    if let Self::ThreeOfAKind {
                        rank: rank_b,
                        extra: extra_b,
                    } = other
                    {
                        rank_a == rank_b && compare_extra(extra_a, extra_b)
                    } else {
                        false
                    }
                }
                Self::Straight { rank: rank_a } => {
                    if let Self::Straight { rank: rank_b } = other {
                        rank_a == rank_b
                    } else {
                        false
                    }
                }
                Self::Flush { rank: rank_a } => {
                    if let Self::Flush { rank: rank_b } = other {
                        rank_a == rank_b
                    } else {
                        false
                    }
                }
                Self::FullHouse {
                    two: rank_two_a,
                    three: rank_three_a,
                } => {
                    if let Self::FullHouse {
                        two: rank_two_b,
                        three: rank_three_b,
                    } = other
                    {
                        rank_two_a == rank_two_b && rank_three_a == rank_three_b
                    } else {
                        false
                    }
                }
                Self::FourOfAKind {
                    rank: rank_a,
                    extra: extra_a,
                } => {
                    if let Self::FourOfAKind {
                        rank: rank_b,
                        extra: extra_b,
                    } = other
                    {
                        rank_a == rank_b && compare_extra(extra_a, extra_b)
                    } else {
                        false
                    }
                }
                Self::StraightFlush { rank: rank_a } => {
                    if let Self::StraightFlush { rank: rank_b } = other {
                        rank_a == rank_b
                    } else {
                        false
                    }
                }
            }
        }
    }
}

impl Eq for Combination {}

fn compare_extra(xs: &[Rank], ys: &[Rank]) -> bool {
    assert_eq!(xs.len(), ys.len());

    let xs = xs.iter().sorted();
    let ys = ys.iter().sorted();

    xs.zip(ys).all(|(x, y)| x == y)
}

#[cfg(test)]
mod tests {
    use crate::card::Rank;
    use crate::combination::Combination;

    #[test]
    fn test_compare_high_card_with_different_combinations() {
        let lhs = Combination::HighCard { rank: Rank::Two };

        assert_ne!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five]
            }
        );
        assert_ne!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_ne!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four]
            }
        );
        assert_ne!(lhs, Combination::Straight { rank: Rank::Two });
        assert_ne!(lhs, Combination::Flush { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three
            }
        );
        assert_ne!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three]
            }
        );
        assert_ne!(lhs, Combination::StraightFlush { rank: Rank::Two });
    }

    #[test]
    fn test_compare_high_card_with_high_card() {
        let lhs = Combination::HighCard { rank: Rank::Two };

        assert_eq!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_ne!(lhs, Combination::HighCard { rank: Rank::Three });
    }

    #[test]
    fn test_compare_pair_with_different_combinations() {
        let lhs = Combination::Pair {
            rank: Rank::Two,
            extra: [Rank::Three, Rank::Four, Rank::Five],
        };

        assert_ne!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_ne!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four]
            }
        );
        assert_ne!(lhs, Combination::Straight { rank: Rank::Two });
        assert_ne!(lhs, Combination::Flush { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three
            }
        );
        assert_ne!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three]
            }
        );
        assert_ne!(lhs, Combination::StraightFlush { rank: Rank::Two });
    }

    #[test]
    fn test_compare_pair_with_pair() {
        let lhs = Combination::Pair {
            rank: Rank::Two,
            extra: [Rank::Three, Rank::Four, Rank::Five],
        };

        assert_eq!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five]
            }
        );
        assert_eq!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Five, Rank::Four]
            }
        );
        assert_eq!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Five, Rank::Three, Rank::Four]
            }
        );
        assert_eq!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Four, Rank::Three, Rank::Five]
            }
        );
        assert_eq!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Five, Rank::Four, Rank::Three]
            }
        );
        assert_eq!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Four, Rank::Five, Rank::Three]
            }
        );

        assert_ne!(
            lhs,
            Combination::Pair {
                rank: Rank::Three,
                extra: [Rank::Four, Rank::Five, Rank::Three]
            }
        );
        assert_ne!(
            lhs,
            Combination::Pair {
                rank: Rank::Three,
                extra: [Rank::Jack, Rank::Five, Rank::Three]
            }
        );
        assert_ne!(
            lhs,
            Combination::Pair {
                rank: Rank::Three,
                extra: [Rank::Jack, Rank::Queen, Rank::Three]
            }
        );
        assert_ne!(
            lhs,
            Combination::Pair {
                rank: Rank::Three,
                extra: [Rank::Jack, Rank::Queen, Rank::King]
            }
        );
    }

    #[test]
    fn test_compare_two_pairs_with_different_combinations() {
        let lhs = Combination::TwoPairs {
            low: Rank::Two,
            high: Rank::Three,
            extra: [Rank::Four],
        };

        assert_ne!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five]
            }
        );
        assert_ne!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four]
            }
        );
        assert_ne!(lhs, Combination::Straight { rank: Rank::Two });
        assert_ne!(lhs, Combination::Flush { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three
            }
        );
        assert_ne!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three]
            }
        );
        assert_ne!(lhs, Combination::StraightFlush { rank: Rank::Two });
    }

    #[test]
    fn test_compare_two_pairs_with_two_pairs() {
        let lhs = Combination::TwoPairs {
            low: Rank::Three,
            high: Rank::Five,
            extra: [Rank::Four],
        };

        assert_eq!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Three,
                high: Rank::Five,
                extra: [Rank::Four],
            }
        );
        assert_ne!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Five,
                extra: [Rank::Four],
            }
        );
        assert_ne!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Three,
                high: Rank::Six,
                extra: [Rank::Four],
            }
        );
        assert_ne!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Three,
                high: Rank::Five,
                extra: [Rank::Six],
            }
        );
    }

    #[test]
    fn test_compare_three_of_a_kind_with_different_combinations() {
        let lhs = Combination::ThreeOfAKind {
            rank: Rank::Two,
            extra: [Rank::Three, Rank::Four],
        };

        assert_ne!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five]
            }
        );
        assert_ne!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_ne!(lhs, Combination::Straight { rank: Rank::Two });
        assert_ne!(lhs, Combination::Flush { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three
            }
        );
        assert_ne!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three]
            }
        );
        assert_ne!(lhs, Combination::StraightFlush { rank: Rank::Two });
    }

    #[test]
    fn test_compare_three_of_a_kind_with_three_of_a_kind() {
        let lhs = Combination::ThreeOfAKind {
            rank: Rank::Two,
            extra: [Rank::Three, Rank::Four],
        };

        assert_eq!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            }
        );
        assert_eq!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Four, Rank::Three],
            }
        );
        assert_ne!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Three,
                extra: [Rank::Four, Rank::Three],
            }
        );
        assert_ne!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Five, Rank::Three],
            }
        );
        assert_ne!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Five, Rank::Six],
            }
        );
    }

    #[test]
    fn test_compare_straight_with_different_combinations() {
        let lhs = Combination::Straight { rank: Rank::Two };

        assert_ne!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five]
            }
        );
        assert_ne!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_ne!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four]
            }
        );
        assert_ne!(lhs, Combination::Flush { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three
            }
        );
        assert_ne!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three]
            }
        );
        assert_ne!(lhs, Combination::StraightFlush { rank: Rank::Two });
    }

    #[test]
    fn test_compare_straight_with_straight() {
        let lhs = Combination::Straight { rank: Rank::Two };

        assert_eq!(lhs, Combination::Straight { rank: Rank::Two });
        assert_ne!(lhs, Combination::Straight { rank: Rank::Three });
    }

    #[test]
    fn test_compare_flush_with_different_combinations() {
        let lhs = Combination::Flush { rank: Rank::Two };

        assert_ne!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five]
            }
        );
        assert_ne!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_ne!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four]
            }
        );
        assert_ne!(lhs, Combination::Straight { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three
            }
        );
        assert_ne!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three]
            }
        );
        assert_ne!(lhs, Combination::StraightFlush { rank: Rank::Two });
    }

    #[test]
    fn test_compare_flush_with_flush() {
        let lhs = Combination::Flush { rank: Rank::Two };

        assert_eq!(lhs, Combination::Flush { rank: Rank::Two });
        assert_ne!(lhs, Combination::Flush { rank: Rank::Three });
    }

    #[test]
    fn test_compare_full_house_with_different_combinations() {
        let lhs = Combination::FullHouse {
            two: Rank::Two,
            three: Rank::Three,
        };

        assert_ne!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five]
            }
        );
        assert_ne!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_ne!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four]
            }
        );
        assert_ne!(lhs, Combination::Straight { rank: Rank::Two });
        assert_ne!(lhs, Combination::Flush { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three]
            }
        );
        assert_ne!(lhs, Combination::StraightFlush { rank: Rank::Two });
    }

    #[test]
    fn test_compare_full_house_with_full_house() {
        let lhs = Combination::FullHouse {
            two: Rank::Two,
            three: Rank::Three,
        };

        assert_eq!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three
            }
        );
        assert_ne!(
            lhs,
            Combination::FullHouse {
                two: Rank::Four,
                three: Rank::Three
            }
        );
        assert_ne!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Four
            }
        );
        assert_ne!(
            lhs,
            Combination::FullHouse {
                two: Rank::Five,
                three: Rank::Four
            }
        );
    }

    #[test]
    fn test_compare_four_of_a_kind_with_different_combinations() {
        let lhs = Combination::FourOfAKind {
            rank: Rank::Two,
            extra: [Rank::Three],
        };

        assert_ne!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five]
            }
        );
        assert_ne!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_ne!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four]
            }
        );
        assert_ne!(lhs, Combination::Straight { rank: Rank::Two });
        assert_ne!(lhs, Combination::Flush { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three
            }
        );
        assert_ne!(lhs, Combination::StraightFlush { rank: Rank::Two });
    }

    #[test]
    fn test_compare_four_of_a_kind_with_four_of_a_kind() {
        let lhs = Combination::FourOfAKind {
            rank: Rank::Three,
            extra: [Rank::Four],
        };

        assert_eq!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Three,
                extra: [Rank::Four],
            }
        );
        assert_ne!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Four],
            }
        );
        assert_ne!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Three,
                extra: [Rank::Five],
            }
        );
        assert_ne!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Four,
                extra: [Rank::Five],
            }
        );
    }

    #[test]
    fn test_compare_straight_flush_with_different_combinations() {
        let lhs = Combination::StraightFlush { rank: Rank::Two };

        assert_ne!(lhs, Combination::HighCard { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five]
            }
        );
        assert_ne!(
            lhs,
            Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four]
            }
        );
        assert_ne!(
            lhs,
            Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four]
            }
        );
        assert_ne!(lhs, Combination::Straight { rank: Rank::Two });
        assert_ne!(lhs, Combination::Flush { rank: Rank::Two });
        assert_ne!(
            lhs,
            Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three
            }
        );
        assert_ne!(
            lhs,
            Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three]
            }
        );
    }

    #[test]
    fn test_compare_straight_flush_with_straight_flush() {
        let lhs = Combination::StraightFlush { rank: Rank::Two };

        assert_eq!(lhs, Combination::StraightFlush { rank: Rank::Two });
        assert_ne!(lhs, Combination::StraightFlush { rank: Rank::Three });
    }
}
