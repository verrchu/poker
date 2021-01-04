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
mod tests {}
