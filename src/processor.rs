use std::str::FromStr;

use crate::errors::Error;
use crate::types::Game;

pub fn process(line: &str) -> Result<(), Error> {
    let game = Game::from_str(line)?;
    println!("GAME -> {:?}", game);
    Ok(())
}
