use std::str::FromStr;

use crate::errors::Error;
use crate::types::{Board, Game};

pub fn process(line: &str) -> Result<(), Error> {
    let mut tokens: Vec<&str> = line.split(' ').collect();

    let game = Game::from_str(tokens.remove(0))?;
    println!("GAME -> {:?}", game);
    let board = Board::from_str(tokens.remove(0))?;
    println!("BOARD -> {:?}", board);
    let raw_hands: Vec<&str> = tokens;

    Ok(())
}
