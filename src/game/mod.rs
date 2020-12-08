mod impl_game;

use std::convert::TryInto;
use std::str::FromStr;

use crate::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board([Card; 5]);

impl FromStr for Board {
    type Err = std::io::Error;

    fn from_str(board: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = super::card::parse_cards(board).try_into().unwrap();

        Ok(Board(cards))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandOf2([Card; 2]);

impl FromStr for HandOf2 {
    type Err = std::io::Error;

    fn from_str(hand: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 2] = super::card::parse_cards(hand).try_into().unwrap();

        Ok(HandOf2(cards))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandOf4([Card; 4]);

impl FromStr for HandOf4 {
    type Err = std::io::Error;

    fn from_str(hand: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 4] = super::card::parse_cards(hand).try_into().unwrap();

        Ok(HandOf4(cards))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandOf5([Card; 5]);

impl FromStr for HandOf5 {
    type Err = std::io::Error;

    fn from_str(hand: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = super::card::parse_cards(hand).try_into().unwrap();

        Ok(HandOf5(cards))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variant(pub [Card; 5]);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::card::Card;
    use crate::card::Rank;
    use crate::card::Suit;
    use crate::game::Board;
    use crate::game::Game;
    use crate::game::HandOf2;
    use crate::game::HandOf4;
    use crate::game::HandOf5;

    #[test]
    fn test_game_texas_holdem_from_str() {
        let s = "texas-holdem AdAcAhAsKd QdJd Td9h";
        assert_eq!(
            Game::from_str(s).unwrap(),
            Game::TexasHoldem(
                Board([
                    Card(Rank::Ace, Suit::Diamonds),
                    Card(Rank::Ace, Suit::Clubs),
                    Card(Rank::Ace, Suit::Hearts),
                    Card(Rank::Ace, Suit::Spades),
                    Card(Rank::King, Suit::Diamonds)
                ]),
                vec![
                    HandOf2([
                        Card(Rank::Queen, Suit::Diamonds),
                        Card(Rank::Jack, Suit::Diamonds),
                    ]),
                    HandOf2([
                        Card(Rank::Ten, Suit::Diamonds),
                        Card(Rank::Nine, Suit::Hearts),
                    ]),
                ]
            )
        );
    }

    #[test]
    fn test_game_omaha_holdem_from_str() {
        let s = "omaha-holdem AdAcAhAsKd QdJdTd9h 3d4d5d6d";
        assert_eq!(
            Game::from_str(s).unwrap(),
            Game::OmahaHoldem(
                Board([
                    Card(Rank::Ace, Suit::Diamonds),
                    Card(Rank::Ace, Suit::Clubs),
                    Card(Rank::Ace, Suit::Hearts),
                    Card(Rank::Ace, Suit::Spades),
                    Card(Rank::King, Suit::Diamonds)
                ]),
                vec![
                    HandOf4([
                        Card(Rank::Queen, Suit::Diamonds),
                        Card(Rank::Jack, Suit::Diamonds),
                        Card(Rank::Ten, Suit::Diamonds),
                        Card(Rank::Nine, Suit::Hearts),
                    ]),
                    HandOf4([
                        Card(Rank::Three, Suit::Diamonds),
                        Card(Rank::Four, Suit::Diamonds),
                        Card(Rank::Five, Suit::Diamonds),
                        Card(Rank::Six, Suit::Diamonds),
                    ]),
                ]
            )
        );
    }

    #[test]
    fn test_game_five_card_draw_from_str() {
        let s = "five-card-draw AdAcAhAsKd QdJdTd9h3d";
        assert_eq!(
            Game::from_str(s).unwrap(),
            Game::FiveCardDraw(vec![
                HandOf5([
                    Card(Rank::Ace, Suit::Diamonds),
                    Card(Rank::Ace, Suit::Clubs),
                    Card(Rank::Ace, Suit::Hearts),
                    Card(Rank::Ace, Suit::Spades),
                    Card(Rank::King, Suit::Diamonds)
                ]),
                HandOf5([
                    Card(Rank::Queen, Suit::Diamonds),
                    Card(Rank::Jack, Suit::Diamonds),
                    Card(Rank::Ten, Suit::Diamonds),
                    Card(Rank::Nine, Suit::Hearts),
                    Card(Rank::Three, Suit::Diamonds),
                ]),
            ])
        );
    }

    #[test]
    fn test_board_from_str() {
        let s = "AdAcAhAsKd";
        assert_eq!(
            Board::from_str(s).unwrap(),
            Board([
                Card(Rank::Ace, Suit::Diamonds),
                Card(Rank::Ace, Suit::Clubs),
                Card(Rank::Ace, Suit::Hearts),
                Card(Rank::Ace, Suit::Spades),
                Card(Rank::King, Suit::Diamonds)
            ]),
        );
    }

    #[test]
    fn test_hand_of_2_from_str() {
        let s = "AdAc";
        assert_eq!(
            HandOf2::from_str(s).unwrap(),
            HandOf2([
                Card(Rank::Ace, Suit::Diamonds),
                Card(Rank::Ace, Suit::Clubs),
            ]),
        );
    }

    #[test]
    fn test_hand_of_4_from_str() {
        let s = "AdAcAhAs";
        assert_eq!(
            HandOf4::from_str(s).unwrap(),
            HandOf4([
                Card(Rank::Ace, Suit::Diamonds),
                Card(Rank::Ace, Suit::Clubs),
                Card(Rank::Ace, Suit::Hearts),
                Card(Rank::Ace, Suit::Spades),
            ]),
        );
    }

    #[test]
    fn test_hand_of_5_from_str() {
        let s = "AdAcAhAsKd";
        assert_eq!(
            HandOf5::from_str(s).unwrap(),
            HandOf5([
                Card(Rank::Ace, Suit::Diamonds),
                Card(Rank::Ace, Suit::Clubs),
                Card(Rank::Ace, Suit::Hearts),
                Card(Rank::Ace, Suit::Spades),
                Card(Rank::King, Suit::Diamonds)
            ]),
        );
    }
}
