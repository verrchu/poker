use super::Rank;
use std::cmp::Ordering;

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
