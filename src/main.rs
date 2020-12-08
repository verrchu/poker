use std::io::BufRead;
use std::str::FromStr;

mod card;
mod combination;
mod game;

use game::Game;

fn main() {
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        process(&line.unwrap())
    }
}

fn process(line: &str) {
    let game = Game::from_str(line).unwrap();
    let _ordered_hands = game.sort_hands();
}
