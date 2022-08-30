use r2048::*;
use std::{process::exit, thread::sleep, time::Duration};

fn main() {
    let mut game: Board = Board::new();
    println!("Starting State:\n\n{}", game);
    for d in [
        Direction::LEFT,
        Direction::RIGHT,
        Direction::UP,
        Direction::DOWN,
    ] {
        match game.shift(d) {
            Ok(()) => {
                println!("\r{}", game)
            }
            Err(e) => {
                println!("Game Over! - : {}", e);
                sleep(Duration::from_secs(3));
                exit(0);
            }
        }
    }
}
