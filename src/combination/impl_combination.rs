use std::collections::HashMap;

use ::itertools::Itertools;

use crate::card::Card;
use crate::card::Rank;
use crate::combination::Combination;
use crate::game::Variant;

impl Combination {
    pub fn from_variant(variant: Variant) -> Self {
        Self::try_straight_flush(variant)
            .or_else(|| Self::try_four_of_a_kind(variant))
            .or_else(|| Self::try_full_house(variant))
            .or_else(|| Self::try_flush(variant))
            .or_else(|| Self::try_straight(variant))
            .or_else(|| Self::try_three_of_a_kind(variant))
            .or_else(|| Self::try_two_pairs(variant))
            .or_else(|| Self::try_pair(variant))
            .or_else(|| Self::try_high_card(variant))
            .unwrap()
    }

    pub fn try_straight_flush(variant: Variant) -> Option<Self> {
        Self::try_flush(variant)
            .and_then(|_flush| Self::try_straight(variant))
            .and_then(|straight| {
                if let Self::Straight { rank } = straight {
                    Some(Self::StraightFlush { rank })
                } else {
                    None
                }
            })
    }

    pub fn try_four_of_a_kind(variant: Variant) -> Option<Self> {
        let groups = Self::group_ranks(variant.0);

        let rank = groups
            .clone()
            .into_iter()
            .find(|(_rank, n)| *n == 4)
            .map(|(rank, _)| rank);

        let kicker = rank.and(
            groups
                .into_iter()
                .filter(|(_rank, n)| *n == 1)
                .map(|(rank, _)| rank)
                .max(),
        );

        rank.and_then(|rank| kicker.map(|kicker| (rank, kicker)))
            .map(|(rank, kicker)| Self::FourOfAKind { rank, kicker })
    }

    pub fn try_full_house(variant: Variant) -> Option<Self> {
        let groups = Self::group_ranks(variant.0);

        let three = groups
            .clone()
            .into_iter()
            .find(|(_rank, n)| *n == 3)
            .map(|(rank, _)| rank);

        let two = three.and(
            groups
                .into_iter()
                .filter(|(_rank, n)| *n == 2)
                .map(|(rank, _)| rank)
                .max(),
        );

        three
            .and_then(|three| two.map(|two| (three, two)))
            .map(|(three, two)| Self::FullHouse { three, two })
    }

    pub fn try_flush(variant: Variant) -> Option<Self> {
        let cards = &variant.0;
        let suite = cards[0].1;

        if cards.iter().map(|card| card.1).all(|s| s == suite) {
            let max = cards.iter().map(|card| card.0).max().unwrap();
            Some(Self::Flush { rank: max })
        } else {
            None
        }
    }

    pub fn try_straight(variant: Variant) -> Option<Self> {
        let ranks = variant
            .0
            .iter()
            .map(|card| card.0)
            .sorted()
            .collect::<Vec<_>>();

        match ranks.as_slice() {
            [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace] => {
                Some(Self::Straight { rank: Rank::Ace })
            }
            [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six] => {
                Some(Self::Straight { rank: Rank::Two })
            }
            [Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven] => {
                Some(Self::Straight { rank: Rank::Three })
            }
            [Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight] => {
                Some(Self::Straight { rank: Rank::Four })
            }
            [Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine] => {
                Some(Self::Straight { rank: Rank::Five })
            }
            [Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten] => {
                Some(Self::Straight { rank: Rank::Six })
            }
            [Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack] => {
                Some(Self::Straight { rank: Rank::Seven })
            }
            [Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen] => {
                Some(Self::Straight { rank: Rank::Eight })
            }
            [Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King] => {
                Some(Self::Straight { rank: Rank::Nine })
            }
            [Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace] => {
                Some(Self::Straight { rank: Rank::Ten })
            }
            _ => None,
        }
    }

    pub fn try_three_of_a_kind(variant: Variant) -> Option<Self> {
        let groups = Self::group_ranks(variant.0);

        let rank = groups
            .clone()
            .into_iter()
            .find(|(_rank, n)| *n == 3)
            .map(|(rank, _)| rank);

        let kicker = rank.and(
            groups
                .into_iter()
                .filter(|(_rank, n)| *n == 1)
                .map(|(rank, _)| rank)
                .max(),
        );

        rank.and_then(|rank| kicker.map(|kicker| (rank, kicker)))
            .map(|(rank, kicker)| Self::ThreeOfAKind { rank, kicker })
    }

    pub fn try_two_pairs(variant: Variant) -> Option<Self> {
        let groups = Self::group_ranks(variant.0);

        let ranks = groups
            .clone()
            .into_iter()
            .filter(|(_rank, n)| *n == 2)
            .map(|(rank, _)| rank)
            .collect::<Vec<_>>();

        if ranks.len() != 2 {
            None
        } else {
            let kicker = groups
                .into_iter()
                .find(|(_rank, n)| *n == 1)
                .map(|(rank, _)| rank)
                .unwrap();

            if ranks[0] < ranks[1] {
                Some(Self::TwoPairs {
                    low: ranks[0],
                    high: ranks[1],
                    kicker,
                })
            } else {
                Some(Self::TwoPairs {
                    low: ranks[1],
                    high: ranks[0],
                    kicker,
                })
            }
        }
    }

    pub fn try_pair(variant: Variant) -> Option<Self> {
        let groups = Self::group_ranks(variant.0);

        let rank = groups
            .clone()
            .into_iter()
            .find(|(_rank, n)| *n == 2)
            .map(|(rank, _)| rank);

        let kicker = rank.and(
            groups
                .into_iter()
                .filter(|(_rank, n)| *n == 1)
                .map(|(rank, _)| rank)
                .max(),
        );

        rank.and_then(|rank| kicker.map(|kicker| (rank, kicker)))
            .map(|(rank, kicker)| Self::Pair { rank, kicker })
    }

    pub fn try_high_card(variant: Variant) -> Option<Self> {
        let cards = &variant.0;

        let rank = cards.iter().map(|card| card.0).max().unwrap();

        Some(Self::HighCard { rank })
    }

    fn group_ranks(cards: [Card; 5]) -> HashMap<Rank, u64> {
        cards
            .iter()
            .map(|card| card.0)
            .fold(HashMap::new(), |mut acc, x| {
                let n = match acc.get(&x) {
                    Some(n) => n + 1,
                    None => 1,
                };

                acc.insert(x, n);

                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Card;
    use crate::card::Rank;
    use crate::card::Suite;
    use crate::combination::Combination;
    use crate::game::Variant;

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
