use r2048::*;
use std::{process::exit, thread::sleep, time::Duration};

fn main() {
    let game: Board = Board::new();
    for d in [
        Direction::LEFT,
        Direction::RIGHT,
        Direction::UP,
        Direction::DOWN,
    ] {
        match game.shift(d) {
            Some(next) => {
                println!("\r{}", next)
            }
            None => {
                println!("Game Over!");
                sleep(Duration::from_secs(3));
                exit(0);
            }
        }
    }
}
