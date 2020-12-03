use std::str::FromStr;

use crate::errors::Error;
use crate::types::Game;

pub fn process(line: &str) -> Result<(), Error> {
    let mut tokens: Vec<&str> = line.split(' ').collect();

    let game = Game::from_str(tokens.remove(0))?;
    let raw_board = tokens.remove(0);
    let raw_hands: Vec<&str> = tokens;

    Ok(())
}
