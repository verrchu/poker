use ::regex::Regex;

use std::str::FromStr;

use crate::errors::Error;

#[derive(Debug)]
pub enum Game {
    TexasHoldem,
    OmahaHoldem,
    FiveCardDraw,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "texas-holdem" => Ok(Self::TexasHoldem),
            "omaha-holdem" => Ok(Self::OmahaHoldem),
            "five-card-draw" => Ok(Self::FiveCardDraw),
            _ => Err(Error::GameParseError(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Suite {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl FromStr for Suite {
    type Err = Error;

    fn from_str(suite: &str) -> Result<Self, Self::Err> {
        match suite {
            "c" => Ok(Self::Clubs),
            "d" => Ok(Self::Diamonds),
            "h" => Ok(Self::Hearts),
            "s" => Ok(Self::Spades),
            _ => Err(Error::SuiteParseError(suite.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
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
    type Err = Error;

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
            "10" => Ok(Self::Ten),
            "J" => Ok(Self::Jack),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => Err(Error::RankParseError(rank.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Card(Rank, Suite);

#[derive(Debug)]
pub struct Board([Card; 5]);

impl FromStr for Board {
    type Err = Error;

    fn from_str(board: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([2-9]|10|[AJQK])([cdhs])").unwrap();
        let cards = re
            .find_iter(board)
            .map(|card| card.as_str())
            .map(|card| {
                let captures = re.captures(card).unwrap();

                let rank = captures.get(1).unwrap().as_str();
                let rank = Rank::from_str(rank)?;

                let suite = captures.get(2).unwrap().as_str();
                let suite = Suite::from_str(suite)?;

                Ok(Card(rank, suite))
            })
            .collect::<Result<Vec<Card>, Error>>()?;

        Ok(Self([
            cards.get(0).unwrap().clone(),
            cards.get(1).unwrap().clone(),
            cards.get(2).unwrap().clone(),
            cards.get(3).unwrap().clone(),
            cards.get(4).unwrap().clone(),
        ]))
    }
}
