use std::str::FromStr;

use crate::errors::Error;

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
