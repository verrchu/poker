use std::hash::{Hash, Hasher};

use super::Combination;

use ::itertools::Itertools;

impl Hash for Combination {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Combination::HighCard { rank } => {
                rank.hash(state);
            }
            Combination::Pair { rank, extra } => {
                rank.hash(state);
                extra.iter().sorted().collect::<Vec<_>>().hash(state);
            }
            Combination::TwoPairs { low, high, extra } => {
                low.hash(state);
                high.hash(state);
                extra.iter().sorted().collect::<Vec<_>>().hash(state);
            }
            Combination::ThreeOfAKind { rank, extra } => {
                rank.hash(state);
                extra.iter().sorted().collect::<Vec<_>>().hash(state);
            }
            Combination::Straight { rank } => {
                rank.hash(state);
            }
            Combination::Flush { rank } => {
                rank.hash(state);
            }
            Combination::FullHouse { two, three } => {
                two.hash(state);
                three.hash(state);
            }
            Combination::FourOfAKind { rank, extra } => {
                rank.hash(state);
                extra.iter().sorted().collect::<Vec<_>>().hash(state);
            }
            Combination::StraightFlush { rank } => {
                rank.hash(state);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use crate::card::Rank;
    use crate::combination::Combination;

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn test_compare_high_card_hash_with_different_combination_hashes() {
        let lhs = Combination::HighCard { rank: Rank::Two };

        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Straight { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Flush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::StraightFlush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_high_card_hash_with_high_card_hash() {
        let lhs = Combination::HighCard { rank: Rank::Two };

        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::HighCard { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::HighCard { rank: Rank::Three };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_pair_hash_with_different_combination_hashes() {
        let lhs = Combination::Pair {
            rank: Rank::Two,
            extra: [Rank::Three, Rank::Four, Rank::Five],
        };

        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::HighCard { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Straight { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Flush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::StraightFlush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_pair_hash_with_pair_hash() {
        let lhs = Combination::Pair {
            rank: Rank::Two,
            extra: [Rank::Three, Rank::Four, Rank::Five],
        };

        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five],
            };
            calculate_hash(&rhs)
        });
        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Five, Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Five, Rank::Three, Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Four, Rank::Three, Rank::Five],
            };
            calculate_hash(&rhs)
        });
        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Five, Rank::Four, Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Four, Rank::Five, Rank::Three],
            };
            calculate_hash(&rhs)
        });

        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Three,
                extra: [Rank::Four, Rank::Five, Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Three,
                extra: [Rank::Jack, Rank::Five, Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Three,
                extra: [Rank::Jack, Rank::Queen, Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Three,
                extra: [Rank::Jack, Rank::Queen, Rank::King],
            };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_two_pairs_hash_with_different_combination_hashes() {
        let lhs = Combination::TwoPairs {
            low: Rank::Two,
            high: Rank::Three,
            extra: [Rank::Four],
        };

        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::HighCard { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Straight { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Flush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::StraightFlush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_two_pairs_hash_with_two_pairs_hash() {
        let lhs = Combination::TwoPairs {
            low: Rank::Three,
            high: Rank::Five,
            extra: [Rank::Four],
        };

        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::TwoPairs {
                low: Rank::Three,
                high: Rank::Five,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Five,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::TwoPairs {
                low: Rank::Three,
                high: Rank::Six,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::TwoPairs {
                low: Rank::Three,
                high: Rank::Five,
                extra: [Rank::Six],
            };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_three_of_a_kind_hash_with_different_combination_hashes() {
        let lhs = Combination::ThreeOfAKind {
            rank: Rank::Two,
            extra: [Rank::Three, Rank::Four],
        };

        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::HighCard { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Straight { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Flush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::StraightFlush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_three_of_a_kind_hash_with_three_of_a_kind_hash() {
        let lhs = Combination::ThreeOfAKind {
            rank: Rank::Two,
            extra: [Rank::Three, Rank::Four],
        };

        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Four, Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Three,
                extra: [Rank::Four, Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Five, Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Five, Rank::Six],
            };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_straight_hash_with_different_combination_hashes() {
        let lhs = Combination::Straight { rank: Rank::Two };

        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::HighCard { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Flush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::StraightFlush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_straight_hash_with_straight_hash() {
        let lhs = Combination::Straight { rank: Rank::Two };

        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::Straight { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Straight { rank: Rank::Three };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_flush_hash_with_different_combination_hashes() {
        let lhs = Combination::Flush { rank: Rank::Two };

        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::HighCard { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Straight { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::StraightFlush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_flush_hash_with_flush_hash() {
        let lhs = Combination::Flush { rank: Rank::Two };

        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::Flush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Flush { rank: Rank::Three };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_full_house_hash_with_different_combination_hashes() {
        let lhs = Combination::FullHouse {
            two: Rank::Two,
            three: Rank::Three,
        };

        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::HighCard { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Straight { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Flush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::StraightFlush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_full_house_hash_with_full_house_hash() {
        let lhs = Combination::FullHouse {
            two: Rank::Two,
            three: Rank::Three,
        };

        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FullHouse {
                two: Rank::Four,
                three: Rank::Three,
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Four,
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FullHouse {
                two: Rank::Five,
                three: Rank::Four,
            };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_four_of_a_kind_hash_with_different_combination_hashes() {
        let lhs = Combination::FourOfAKind {
            rank: Rank::Two,
            extra: [Rank::Three],
        };

        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::HighCard { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Straight { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Flush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::StraightFlush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_four_of_a_kind_hash_with_four_of_a_kind_hash() {
        let lhs = Combination::FourOfAKind {
            rank: Rank::Three,
            extra: [Rank::Four],
        };

        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::FourOfAKind {
                rank: Rank::Three,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FourOfAKind {
                rank: Rank::Three,
                extra: [Rank::Five],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FourOfAKind {
                rank: Rank::Four,
                extra: [Rank::Five],
            };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_straight_flush_hash_with_different_combination_hashes() {
        let lhs = Combination::StraightFlush { rank: Rank::Two };

        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::HighCard { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Pair {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four, Rank::Five],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::TwoPairs {
                low: Rank::Two,
                high: Rank::Three,
                extra: [Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::ThreeOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three, Rank::Four],
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Straight { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::Flush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FullHouse {
                two: Rank::Two,
                three: Rank::Three,
            };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::FourOfAKind {
                rank: Rank::Two,
                extra: [Rank::Three],
            };
            calculate_hash(&rhs)
        });
    }

    #[test]
    fn test_compare_straight_flush_hash_with_straight_flush_hash() {
        let lhs = Combination::StraightFlush { rank: Rank::Two };

        assert_eq!(calculate_hash(&lhs), {
            let rhs = Combination::StraightFlush { rank: Rank::Two };
            calculate_hash(&rhs)
        });
        assert_ne!(calculate_hash(&lhs), {
            let rhs = Combination::StraightFlush { rank: Rank::Three };
            calculate_hash(&rhs)
        });
    }
}
