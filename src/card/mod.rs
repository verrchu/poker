pub mod rank;
pub use rank::Rank;
pub mod suit;
pub use suit::Suit;

use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Card(pub Rank, pub Suit);

impl ToString for Card {
    fn to_string(&self) -> String {
        format!("{}{}", self.0.to_string(), self.1.to_string())
    }
}

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

#[cfg(test)]
mod tests {
    use crate::card::Card;
    use crate::card::Rank;
    use crate::card::Suit;

    #[test]
    fn test_parse_cards() {
        let expected: Vec<Card> = vec![];
        assert_eq!(expected, super::parse_cards(""));

        let expected: Vec<Card> = vec![Card(Rank::Ace, Suit::Diamonds)];
        assert_eq!(expected, super::parse_cards("Ad"));

        let expected: Vec<Card> = vec![
            Card(Rank::Ace, Suit::Diamonds),
            Card(Rank::Ten, Suit::Spades),
        ];
        assert_eq!(expected, super::parse_cards("AdTs"));

        let parsed = super::parse_cards("Ad".repeat(100).as_str());
        assert_eq!(parsed.len(), 100);
        assert!(parsed
            .into_iter()
            .all(|card| card == Card(Rank::Ace, Suit::Diamonds)));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            Card(Rank::Ace, Suit::Diamonds).to_string(),
            "Ad".to_string()
        );
    }
}
