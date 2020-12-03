use std::io;
use std::io::BufRead;

mod errors;
mod processor;
mod types;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        processor::process(&line?).unwrap();
    }

    Ok(())
}
