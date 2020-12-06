use crate::card::Card;
use crate::card::Rank;
use crate::game::Variant;

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Combination {
    HighCard { rank: Rank },
    Pair { rank: Rank, kicker: Rank },
    TwoPairs { low: Rank, high: Rank, kicker: Rank },
    ThreeOfAKind { rank: Rank, kicker: Rank },
    Straight { rank: Rank },
    Flush { rank: Rank },
    FullHouse { two: Rank, three: Rank },
    FourOfAKind { rank: Rank, kicker: Rank },
    StraightFlush { rank: Rank },
}

impl PartialOrd for Combination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::HighCard { rank: rank_a } => match other {
                Self::HighCard { rank: rank_b } => rank_a.partial_cmp(rank_b),
                Self::Pair { rank: _, kicker: _ } => Some(Ordering::Less),
                Self::TwoPairs {
                    low: _,
                    high: _,
                    kicker: _,
                } => Some(Ordering::Less),
                Self::ThreeOfAKind { rank: _, kicker: _ } => Some(Ordering::Less),
                Self::Straight { rank: _ } => Some(Ordering::Less),
                Self::Flush { rank: _ } => Some(Ordering::Less),
                Self::FullHouse { two: _, three: _ } => Some(Ordering::Less),
                Self::FourOfAKind { rank: _, kicker: _ } => Some(Ordering::Less),
                Self::StraightFlush { rank: _ } => Some(Ordering::Less),
            },
            Self::Pair {
                rank: pair_rank_a,
                kicker: kicker_rank_a,
            } => match other {
                Self::HighCard { rank: _ } => Some(Ordering::Greater),
                Self::Pair {
                    rank: pair_rank_b,
                    kicker: kicker_rank_b,
                } => match pair_rank_a.partial_cmp(pair_rank_b) {
                    Some(Ordering::Equal) => kicker_rank_a.partial_cmp(kicker_rank_b),
                    ord => ord,
                },
                Self::TwoPairs {
                    low: _,
                    high: _,
                    kicker: _,
                } => Some(Ordering::Less),
                Self::ThreeOfAKind { rank: _, kicker: _ } => Some(Ordering::Less),
                Self::Straight { rank: _ } => Some(Ordering::Less),
                Self::Flush { rank: _ } => Some(Ordering::Less),
                Self::FullHouse { two: _, three: _ } => Some(Ordering::Less),
                Self::FourOfAKind { rank: _, kicker: _ } => Some(Ordering::Less),
                Self::StraightFlush { rank: _ } => Some(Ordering::Less),
            },
            Self::TwoPairs {
                low: low_pair_rank_a,
                high: high_pair_rank_a,
                kicker: kicker_a,
            } => match other {
                Self::HighCard { rank: _ } => Some(Ordering::Greater),
                Self::Pair { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::TwoPairs {
                    low: low_pair_rank_b,
                    high: high_pair_rank_b,
                    kicker: kicker_b,
                } => match high_pair_rank_a.partial_cmp(high_pair_rank_b) {
                    Some(Ordering::Equal) => match low_pair_rank_a.partial_cmp(low_pair_rank_b) {
                        Some(Ordering::Equal) => kicker_a.partial_cmp(kicker_b),
                        ord => ord,
                    },
                    ord => ord,
                },
                Self::ThreeOfAKind { rank: _, kicker: _ } => Some(Ordering::Less),
                Self::Straight { rank: _ } => Some(Ordering::Less),
                Self::Flush { rank: _ } => Some(Ordering::Less),
                Self::FullHouse { two: _, three: _ } => Some(Ordering::Less),
                Self::FourOfAKind { rank: _, kicker: _ } => Some(Ordering::Less),
                Self::StraightFlush { rank: _ } => Some(Ordering::Less),
            },
            Self::ThreeOfAKind {
                rank: rank_a,
                kicker: kicker_a,
            } => match other {
                Self::HighCard { rank: _ } => Some(Ordering::Greater),
                Self::Pair { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::TwoPairs {
                    low: _,
                    high: _,
                    kicker: _,
                } => Some(Ordering::Greater),
                Self::ThreeOfAKind {
                    rank: rank_b,
                    kicker: kicker_b,
                } => match rank_a.partial_cmp(rank_b) {
                    Some(Ordering::Equal) => kicker_a.partial_cmp(kicker_b),
                    ord => ord,
                },
                Self::Straight { rank: _ } => Some(Ordering::Less),
                Self::Flush { rank: _ } => Some(Ordering::Less),
                Self::FullHouse { two: _, three: _ } => Some(Ordering::Less),
                Self::FourOfAKind { rank: _, kicker: _ } => Some(Ordering::Less),
                Self::StraightFlush { rank: _ } => Some(Ordering::Less),
            },
            Self::Straight { rank: rank_a } => match other {
                Self::HighCard { rank: _ } => Some(Ordering::Greater),
                Self::Pair { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::TwoPairs {
                    low: _,
                    high: _,
                    kicker: _,
                } => Some(Ordering::Greater),
                Self::ThreeOfAKind { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::Straight { rank: rank_b } => match rank_a {
                    Rank::Ace => match rank_b {
                        Rank::Ace => Some(Ordering::Equal),
                        _ => Some(Ordering::Less),
                    },
                    rank_a => match rank_b {
                        Rank::Ace => Some(Ordering::Equal),
                        rank_b => rank_a.partial_cmp(rank_b),
                    },
                },
                Self::Flush { rank: _ } => Some(Ordering::Less),
                Self::FullHouse { two: _, three: _ } => Some(Ordering::Less),
                Self::FourOfAKind { rank: _, kicker: _ } => Some(Ordering::Less),
                Self::StraightFlush { rank: _ } => Some(Ordering::Less),
            },
            Self::Flush { rank: rank_a } => match other {
                Self::HighCard { rank: _ } => Some(Ordering::Greater),
                Self::Pair { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::TwoPairs {
                    low: _,
                    high: _,
                    kicker: _,
                } => Some(Ordering::Greater),
                Self::ThreeOfAKind { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::Straight { rank: _ } => Some(Ordering::Greater),
                Self::Flush { rank: rank_b } => rank_a.partial_cmp(rank_b),
                Self::FullHouse { two: _, three: _ } => Some(Ordering::Less),
                Self::FourOfAKind { rank: _, kicker: _ } => Some(Ordering::Less),
                Self::StraightFlush { rank: _ } => Some(Ordering::Less),
            },
            Self::FullHouse {
                two: rank_two_a,
                three: rank_three_a,
            } => match other {
                Self::HighCard { rank: _ } => Some(Ordering::Greater),
                Self::Pair { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::TwoPairs {
                    low: _,
                    high: _,
                    kicker: _,
                } => Some(Ordering::Greater),
                Self::ThreeOfAKind { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::Straight { rank: _ } => Some(Ordering::Greater),
                Self::Flush { rank: _ } => Some(Ordering::Greater),
                Self::FullHouse {
                    two: rank_two_b,
                    three: rank_three_b,
                } => match rank_three_a.partial_cmp(rank_three_b) {
                    Some(Ordering::Equal) => rank_two_a.partial_cmp(rank_two_b),
                    ord => ord,
                },
                Self::FourOfAKind { rank: _, kicker: _ } => Some(Ordering::Less),
                Self::StraightFlush { rank: _ } => Some(Ordering::Less),
            },
            Self::FourOfAKind {
                rank: rank_a,
                kicker: kicker_a,
            } => match other {
                Self::HighCard { rank: _ } => Some(Ordering::Greater),
                Self::Pair { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::TwoPairs {
                    low: _,
                    high: _,
                    kicker: _,
                } => Some(Ordering::Greater),
                Self::ThreeOfAKind { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::Straight { rank: _ } => Some(Ordering::Greater),
                Self::Flush { rank: _ } => Some(Ordering::Greater),
                Self::FullHouse { two: _, three: _ } => Some(Ordering::Greater),
                Self::FourOfAKind {
                    rank: rank_b,
                    kicker: kicker_b,
                } => match rank_a.partial_cmp(rank_b) {
                    Some(Ordering::Equal) => kicker_a.partial_cmp(kicker_b),
                    ord => ord,
                },
                Self::StraightFlush { rank: _ } => Some(Ordering::Less),
            },
            Self::StraightFlush { rank: rank_a } => match other {
                Self::HighCard { rank: _ } => Some(Ordering::Greater),
                Self::Pair { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::TwoPairs {
                    low: _,
                    high: _,
                    kicker: _,
                } => Some(Ordering::Greater),
                Self::ThreeOfAKind { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::Straight { rank: _ } => Some(Ordering::Greater),
                Self::Flush { rank: _ } => Some(Ordering::Greater),
                Self::FullHouse { two: _, three: _ } => Some(Ordering::Greater),
                Self::FourOfAKind { rank: _, kicker: _ } => Some(Ordering::Greater),
                Self::StraightFlush { rank: rank_b } => match rank_a {
                    Rank::Ace => match rank_b {
                        Rank::Ace => Some(Ordering::Equal),
                        _ => Some(Ordering::Less),
                    },
                    rank_a => match rank_b {
                        Rank::Ace => Some(Ordering::Equal),
                        rank_b => rank_a.partial_cmp(rank_b),
                    },
                },
            },
        }
    }
}

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
        let mut ranks = variant.0.iter().map(|card| card.0).collect::<Vec<_>>();
        ranks.sort();

        match ranks.as_slice() {
            &[Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace] => {
                Some(Self::Straight { rank: Rank::Ace })
            }
            &[Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six] => {
                Some(Self::Straight { rank: Rank::Two })
            }
            &[Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven] => {
                Some(Self::Straight { rank: Rank::Three })
            }
            &[Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight] => {
                Some(Self::Straight { rank: Rank::Four })
            }
            &[Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine] => {
                Some(Self::Straight { rank: Rank::Five })
            }
            &[Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten] => {
                Some(Self::Straight { rank: Rank::Six })
            }
            &[Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack] => {
                Some(Self::Straight { rank: Rank::Seven })
            }
            &[Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen] => {
                Some(Self::Straight { rank: Rank::Eight })
            }
            &[Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King] => {
                Some(Self::Straight { rank: Rank::Nine })
            }
            &[Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace] => {
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
