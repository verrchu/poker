use std::hash::Hash;

mod impl_combination;
mod impl_eq;
mod impl_ord;

use crate::card::Rank;

#[derive(Debug, Clone, Copy, Hash)]
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
        extra: [Rank; 2],
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
