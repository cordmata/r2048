use console::Term;
use r2048::*;
use std::{error::Error, process::exit};

fn main() {
    let mut game: Board = Board::new();
    let term = Term::stdout();
    term.clear_screen().expect("Should be able to clear screen");
    println!("\r{}", game);
    loop {
        match term.read_key() {
            Ok(key) => {
                term.clear_screen().expect("Should be able to clear screen");
                let direction = match key {
                    console::Key::ArrowLeft => Direction::LEFT,
                    console::Key::ArrowRight => Direction::RIGHT,
                    console::Key::ArrowUp => Direction::UP,
                    console::Key::ArrowDown => Direction::DOWN,
                    _ => continue,
                };
                match game.shift(direction) {
                    Ok(()) => {
                        match game.add_random_tile() {
                            Ok(_) => {}
                            Err(e) => bail(e),
                        };
                        println!("\r{}", game)
                    }
                    Err(e) => bail(e),
                }
            }
            Err(e) => bail(Box::new(e)),
        }
    }
}

fn bail(e: Box<dyn Error>) {
    println!("Game Over! {}", e);
    exit(0);
}
