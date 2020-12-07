mod impl_game;

use std::convert::TryInto;
use std::str::FromStr;

use crate::card::Card;

#[derive(Debug, Clone)]
pub enum Game {
    TexasHoldem(Board, Vec<HandOf2>),
    OmahaHoldem(Board, Vec<HandOf4>),
    FiveCardDraw(Vec<HandOf5>),
}

impl FromStr for Game {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ').collect::<Vec<_>>();

        let raw_game = tokens.remove(0);

        match raw_game {
            "texas-holdem" => {
                let raw_board = tokens.remove(0);
                let board = Board::from_str(raw_board).unwrap();
                let hands = tokens
                    .into_iter()
                    .map(|token| HandOf2::from_str(token).unwrap())
                    .collect::<Vec<HandOf2>>();
                Ok(Self::TexasHoldem(board, hands))
            }
            "omaha-holdem" => {
                let raw_board = tokens.remove(0);
                let board = Board::from_str(raw_board).unwrap();
                let hands = tokens
                    .into_iter()
                    .map(|token| HandOf4::from_str(token).unwrap())
                    .collect::<Vec<HandOf4>>();
                Ok(Self::OmahaHoldem(board, hands))
            }
            "five-card-draw" => {
                let hands = tokens
                    .into_iter()
                    .map(|token| HandOf5::from_str(token).unwrap())
                    .collect::<Vec<HandOf5>>();
                Ok(Self::FiveCardDraw(hands))
            }
            _ => panic!("unknown game value: {}", raw_game),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Board([Card; 5]);

impl FromStr for Board {
    type Err = std::io::Error;

    fn from_str(board: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = super::card::parse_cards(board).try_into().unwrap();

        Ok(Board(cards))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HandOf2([Card; 2]);

impl FromStr for HandOf2 {
    type Err = std::io::Error;

    fn from_str(hand: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 2] = super::card::parse_cards(hand).try_into().unwrap();

        Ok(HandOf2(cards))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HandOf4([Card; 4]);

impl FromStr for HandOf4 {
    type Err = std::io::Error;

    fn from_str(hand: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 4] = super::card::parse_cards(hand).try_into().unwrap();

        Ok(HandOf4(cards))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HandOf5([Card; 5]);

impl FromStr for HandOf5 {
    type Err = std::io::Error;

    fn from_str(hand: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = super::card::parse_cards(hand).try_into().unwrap();

        Ok(HandOf5(cards))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Variant(pub [Card; 5]);
