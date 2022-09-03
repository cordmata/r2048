use console::Term;
use r2048::*;
use std::{error::Error, process::exit};

fn main() {
    game_loop(Term::stdout(), Board::new()).unwrap_or_else(|e| {
        println!("Game Over! {e}");
        exit(0);
    });
}

fn game_loop(term: Term, mut game: Board) -> Result<(), Box<dyn Error>> {
    term.clear_screen()?;
    println!("\r{game}");
    loop {
        match term.read_key()? {
            console::Key::ArrowLeft => game.shift_left()?,
            console::Key::ArrowRight => game.shift_right()?,
            console::Key::ArrowUp => game.shift_up()?,
            console::Key::ArrowDown => game.shift_down()?,
            _ => continue,
        };
        game.add_random_tile()?;
        term.clear_screen()?;
        println!("\r{game}");
    }
}
