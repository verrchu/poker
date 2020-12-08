pub mod rank;
pub use rank::Rank;
pub mod suite;
pub use suite::Suite;

use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Card(pub Rank, pub Suite);

pub fn parse_cards(s: &str) -> Vec<Card> {
    let (ranks, suites): (Vec<_>, Vec<_>) = s
        .chars()
        .into_iter()
        .enumerate()
        .partition(|(i, _elem)| i % 2 == 0);

    ranks
        .into_iter()
        .zip(suites.into_iter())
        .map(|((_, rank), (_, suite))| {
            Card(
                Rank::from_str(&rank.to_string()).unwrap(),
                Suite::from_str(&suite.to_string()).unwrap(),
            )
        })
        .collect::<Vec<_>>()
}
