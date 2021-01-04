use std::hash::Hash;

mod impl_combination;
mod impl_ordering;

use crate::card::Rank;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Combination {
    HighCard {
        rank: Rank,
    },
    Pair {
        rank: Rank,
        extra: [Rank; 3],
    },
    TwoPairs {
        low: Rank,
        high: Rank,
        extra: [Rank; 1],
    },
    ThreeOfAKind {
        rank: Rank,
        kicker: Rank,
    },
    Straight {
        rank: Rank,
    },
    Flush {
        rank: Rank,
    },
    FullHouse {
        two: Rank,
        three: Rank,
    },
    FourOfAKind {
        rank: Rank,
        extra: [Rank; 1],
    },
    StraightFlush {
        rank: Rank,
    },
}
