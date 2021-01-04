use std::hash::{Hash, Hasher};

use super::Combination;

use ::itertools::Itertools;

impl Hash for Combination {
    fn hash<H: Hasher>(&self, state: &mut H) {
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
