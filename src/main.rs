use std::io::BufRead;
use std::str::FromStr;

mod types;

use types::Game;

fn main() {
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        process(&line.unwrap())
    }
}

fn process(line: &str) {
    let game = Game::from_str(line).unwrap();
    println!("GAME -> {:?}", game);
}
