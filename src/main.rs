use std::io::BufRead;
use std::str::FromStr;

mod card;
mod combination;
mod game;

fn main() {
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        process(&line.unwrap())
    }
}

fn process(line: &str) {
    let game = game::Game::from_str(line).unwrap();
}
