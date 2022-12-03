use console::Style;
use core::fmt;
use rand::{seq::IteratorRandom, Rng};
use std::{error::Error, iter::repeat};

#[derive(Debug, Default)]
pub struct Board {
    values: Vec<usize>,
    score: usize,
}

type BoardMutationResult = Result<(), Box<dyn Error>>;

impl Board {
    const SIZE: usize = 4;

    pub fn new() -> Self {
        let mut board = Board {
            values: vec![0; Board::SIZE * Board::SIZE],
            score: 0,
        };
        for _ in 0..2 {
            board
                .add_random_tile()
                .expect("Board should have initial capacity for 2 items.");
        }
        board
    }

    pub fn shift_left(&mut self) -> BoardMutationResult {
        self.values = self
            .values
            .chunks(Board::SIZE)
            .map(score_row)
            .flat_map(|(row, score)| {
                self.score += score;
                row
            })
            .collect();
        Ok(())
    }

    pub fn shift_right(&mut self) -> BoardMutationResult {
        self.values = self
            .values
            .chunks(Board::SIZE)
            .flat_map(|row| {
                let reversed: Vec<_> = row.iter().rev().cloned().collect();
                let (mut scored, score) = score_row(&reversed);
                self.score += score;
                scored.reverse();
                scored
            })
            .collect();
        Ok(())
    }

    pub fn shift_up(&mut self) -> BoardMutationResult {
        self.values = transpose(&self.values);
        self.shift_left()?;
        self.values = transpose(&self.values);
        Ok(())
    }

    pub fn shift_down(&mut self) -> BoardMutationResult {
        self.values = transpose(&self.values);
        self.shift_right()?;
        self.values = transpose(&self.values);
        Ok(())
    }

    pub fn add_random_tile(&mut self) -> BoardMutationResult {
        let mut rng = rand::thread_rng();
        match self
            .values
            .iter()
            .enumerate()
            // collect the indices of the tiles with value 0
            .filter_map(|(idx, val)| match val {
                0 => Some(idx),
                _ => None,
            })
            // choose a random tile
            .choose(&mut rng)
        {
            Some(idx) => {
                // 9 times out of 10 return a 2, but sometimes throw a 4 in there
                self.values[idx] = match rng.gen_range(1..=10) {
                    10 => 4,
                    _ => 2,
                };
                Ok(())
            }
            None => Err("No space left for tile.".into()),
        }
    }
}

fn transpose(orig: &[usize]) -> Vec<usize> {
    let mut flipped = vec![];
    let num_rows = Board::SIZE;
    for col in 0..num_rows {
        for ele in (col..orig.len()).step_by(num_rows) {
            flipped.push(orig[ele]);
        }
    }
    flipped
}

/// Add together each adjacent pair of equal non-zero values to get the row's score.
/// Return a new vector with the results of scoring collapsed to the left.
///
/// score_row(&[2, 2, 4, 4]) ==> (vec![4, 8, 0, 0], 12)
fn score_row(row: &[usize]) -> (Vec<usize>, usize) {
    let mut next = push_zeros(row);
    let mut score = 0;
    for idx in 0..next.len() {
        let val = next[idx];
        if idx < next.len() - 1 && val == next[idx + 1] {
            next[idx] = val * 2;
            score += val * 2;
            next[idx + 1] = 0;
        }
    }
    (push_zeros(&next), score)
}

/// return a new vec with all non-zero values at the front
fn push_zeros(i: &[usize]) -> Vec<usize> {
    i.iter()
        .filter(|&&f| f > 0)
        .cloned()
        .chain(repeat(0))
        .take(i.len())
        .collect()
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row_separator = "+---------".repeat(Board::SIZE) + "+";
        let row_spacer = "|         ".repeat(Board::SIZE) + "|";

        writeln!(f, "Score: {}", self.score)?;
        for row in self.values.chunks(Board::SIZE) {
            writeln!(f, "{row_separator}")?;
            writeln!(f, "{row_spacer}")?;

            for cell in row.iter() {
                let style = match cell {
                    2 => Style::new().bright(),
                    4 => Style::new(),
                    8 => Style::new().magenta(),
                    16 => Style::new().red(),
                    32 => Style::new().red().dim(),
                    64 => Style::new().on_red(),
                    128 => Style::new().yellow().bright(),
                    256 => Style::new().yellow(),
                    512 => Style::new().yellow().dim(),
                    1024 => Style::new().on_yellow(),
                    2048 => Style::new().green(),
                    2049.. => Style::new().on_green(),
                    _ => Style::new(),
                };

                let num_out = match cell {
                    0 => String::from(""),
                    _ => cell.to_string(),
                };

                write!(f, "|{:^9}", style.apply_to(num_out))?;
            }
            writeln!(f, "|")?;
            writeln!(f, "{row_spacer}")?;
        }
        writeln!(f, "{row_separator}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn new_test_board() -> Board {
        Board {
            values: vec![
                8, 8, 0, 2,
                8, 2, 8, 0,
                2, 0, 2, 0,
                2, 4, 4, 0,
            ],
            score: 0,
        }
    }

    #[test]
    #[rustfmt::skip]
    fn board_should_shift_left() {
        let mut b = new_test_board();
        b.shift_left().unwrap();
        assert_eq!(b.values, &[
            16, 2, 0, 0,
            8, 2, 8, 0,
            4, 0, 0, 0,
            2, 8, 0, 0,
        ]);
        assert_eq!(b.score, 28);
    }

    #[test]
    #[rustfmt::skip]
    fn board_should_shift_right() {
        let mut b = new_test_board();
        b.shift_right().unwrap();
        assert_eq!(b.values, &[
            0, 0, 16, 2,
            0, 8, 2, 8,
            0, 0, 0, 4,
            0, 0, 2, 8,
        ]);
        assert_eq!(b.score, 28);
    }

    #[test]
    #[rustfmt::skip]
    fn board_should_shift_up() {
        let mut b = new_test_board();
        b.shift_up().unwrap();
        assert_eq!(b.values, &[
            16, 8, 8, 2,
            4,  2, 2, 0,
            0,  4, 4, 0,
            0,  0, 0, 0,
        ]);
        assert_eq!(b.score, 20);
    }

    #[test]
    #[rustfmt::skip]
    fn board_should_shift_down() {
        let mut b = new_test_board();
        b.shift_down().unwrap();
        assert_eq!(b.values, &[
            0,  0, 0, 0,
            0,  8, 8, 0,
            16, 2, 2, 0,
            4,  4, 4, 2,
        ]);
        assert_eq!(b.score, 20);
    }

    #[test]
    #[rustfmt::skip]
    fn should_transpose() {
        assert_eq!(
            transpose(&[
                2, 1, 2, 3,
                4, 1, 2, 3,
                8, 1, 2, 3,
                2, 1, 2, 3,
            ]), &[
                2, 4, 8, 2,
                1, 1, 1, 1,
                2, 2, 2, 2,
                3, 3, 3, 3,
            ]);
    }

    #[test]
    fn new_board_should_have_2_tiles_with_values() {
        let b = Board::new();
        assert_eq!(b.values.iter().filter(|v| [2, 4].contains(v)).count(), 2);
    }

    #[test]
    fn should_push_zeros() {
        assert_eq!(push_zeros(&[0, 0, 0, 0]), &[0, 0, 0, 0]);
        assert_eq!(push_zeros(&[2, 0, 2, 0]), &[2, 2, 0, 0]);
        assert_eq!(push_zeros(&[0, 2, 4, 2]), &[2, 4, 2, 0]);
        assert_eq!(push_zeros(&[0, 0, 7, 0]), &[7, 0, 0, 0]);
    }

    #[test]
    fn should_score_row() {
        assert_eq!(score_row(&[2, 0, 0, 4]), (vec![2, 4, 0, 0], 0));
        assert_eq!(score_row(&[2, 0, 2, 0]), (vec![4, 0, 0, 0], 4));
        assert_eq!(score_row(&[2, 2, 2, 0]), (vec![4, 2, 0, 0], 4));
        assert_eq!(score_row(&[2, 2, 4, 4]), (vec![4, 8, 0, 0], 12));
        assert_eq!(score_row(&[4, 4, 32, 8]), (vec![8, 32, 8, 0], 8));
        assert_eq!(score_row(&[0, 256, 256, 8]), (vec![512, 8, 0, 0], 512));
    }
}
