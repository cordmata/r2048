use console::{
    Key::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp, Char},
    Term,
};
use r2048::*;
use std::{error::Error, process::exit};

fn main() {
    game_loop(Term::stdout(), Board::new()).unwrap_or_else(|e| {
        println!("Game Over! {e}");
        exit(0);
    });
}

fn game_loop(term: Term, mut game: Board) -> Result<(), Box<dyn Error>> {
    const FOOTER: &str = "🟢 Use arrow keys ⬅️ ⬆️ ⬇️ ➡️  to shift the board. \n\
                          ❌ Press 'q' to quit.";
    term.hide_cursor()?;
    loop {
        term.clear_screen()?;
        println!("\r{game}\n{FOOTER}\n");
        match term.read_key()? {
            ArrowLeft => game.shift_left()?,
            ArrowRight => game.shift_right()?,
            ArrowUp => game.shift_up()?,
            ArrowDown => game.shift_down()?,
            Char(c) if c == 'Q' || c == 'q' => return Err("You bailed".into()),
            _ => continue,
        };
        game.add_random_tile()?;
    }
}
