use std::io::BufRead;
use std::str::FromStr;

use ::itertools::Itertools;

mod card;
mod combination;
mod game;

use card::Card;
use game::Game;

fn main() {
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        process(&line.unwrap())
    }
}

fn process(line: &str) {
    let game = Game::from_str(line).unwrap();

    let ranked_hands = Game::rank_hands(game);
    let grouped_hands = Game::group_hands(ranked_hands);
    let sorted_hands = Game::sort_hands(grouped_hands);

    output(sorted_hands);
}

fn output(hands: Vec<Vec<Vec<Card>>>) {
    let line = hands
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .map(|hand| {
                    hand.into_iter()
                        .map(|card| card.to_string())
                        .format("")
                        .to_string()
                })
                .sorted()
                .format("=")
                .to_string()
        })
        .format(" ")
        .to_string();

    println!("{}", line);
}
