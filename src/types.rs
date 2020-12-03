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

#[derive(Debug)]
pub enum Suite {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Card(Rank, Suite);

#[derive(Debug)]
pub struct Board([Card; 5]);

impl FromStr for Board {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars();

        // use ::regex::Regex;

        // fn main() {
        //     let re = Regex::new(r"([2-9]|10|[AJQK])([cdhs])").unwrap();
        //     let text = "AdKh10c2s";
        //     let res: Vec<_> = re.find_iter(text)
        //         .map(|m| m.as_str())
        //         .map(|m| {
        //             let c = re.captures(m).unwrap();
        //             let rank = c.get(1).unwrap().as_str();
        //             let suite = c.get(2).unwrap().as_str();
        //             (rank, suite)
        //         })
        //         .collect();
        //     println!("{:?}", res);
        // }


        Ok(Self([
            Card(Rank::Ace, Suite::Hearts),
            Card(Rank::Ace, Suite::Hearts),
            Card(Rank::Ace, Suite::Hearts),
            Card(Rank::Ace, Suite::Hearts),
            Card(Rank::Ace, Suite::Hearts),
        ]))
    }
}
