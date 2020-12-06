use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Card(pub Rank, pub Suite);

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Suite {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl FromStr for Suite {
    type Err = std::io::Error;

    fn from_str(suite: &str) -> Result<Self, Self::Err> {
        match suite {
            "c" => Ok(Self::Clubs),
            "d" => Ok(Self::Diamonds),
            "h" => Ok(Self::Hearts),
            "s" => Ok(Self::Spades),
            _ => panic!("unknown suite value: {}", suite),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Rank {
    type Err = std::io::Error;

    fn from_str(rank: &str) -> Result<Self, Self::Err> {
        match rank {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::Ten),
            "J" => Ok(Self::Jack),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => panic!("unknown rank value: {}", rank),
        }
    }
}

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
