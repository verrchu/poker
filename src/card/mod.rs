pub mod rank;
pub use rank::Rank;
pub mod suit;
pub use suit::Suit;

use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Card(pub Rank, pub Suit);

pub fn parse_cards(s: &str) -> Vec<Card> {
    let (ranks, suits): (Vec<_>, Vec<_>) = s
        .chars()
        .into_iter()
        .enumerate()
        .partition(|(i, _elem)| i % 2 == 0);

    ranks
        .into_iter()
        .zip(suits.into_iter())
        .map(|((_, rank), (_, suit))| {
            Card(
                Rank::from_str(&rank.to_string()).unwrap(),
                Suit::from_str(&suit.to_string()).unwrap(),
            )
        })
        .collect::<Vec<_>>()
}
