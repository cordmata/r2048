use console::Style;
use core::fmt;

#[derive(Debug)]
pub struct Board(Vec<usize>);
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Board {
    const SIZE: usize = 4;

    pub fn new() -> Board {
        Board(vec![0; Board::SIZE * Board::SIZE])
    }

    pub fn shift(&self, d: Direction) -> Option<Board> {
        let mut total_score = 0;
        match d {
            Direction::LEFT => Some(Board(
                self.0
                    .chunks(Board::SIZE)
                    .map(combine_and_score)
                    .flat_map(|(row, score)| {
                        total_score += score;
                        row
                    })
                    .collect(),
            )),
            Direction::RIGHT => Some(Board(
                self.0
                    .chunks(Board::SIZE)
                    .map(|r| {
                        let reversed = r.iter().rev().cloned().collect::<Vec<_>>();
                        let (mut scored, score) = combine_and_score(&reversed);
                        scored.reverse();
                        (scored, score)
                    })
                    .flat_map(|(row, score)| {
                        total_score += score;
                        row
                    })
                    .collect(),
            )),
            Direction::UP => {
                let mut flipped = vec![];
                for col in 0..Board::SIZE {
                    for ele in (col..self.0.len()).step_by(4) {
                        flipped.push(self.0[ele]);
                    }
                }
                let left = Board(flipped).shift(Direction::LEFT).unwrap();
                let mut flopped = vec![];
                for col in 0..Board::SIZE {
                    for ele in (col..left.0.len()).step_by(4) {
                        flopped.push(left.0[ele]);
                    }
                }
                Some(Board(flopped))
            }
            Direction::DOWN => {
                let mut flipped = vec![];
                for col in 0..Board::SIZE {
                    for ele in (col..self.0.len()).step_by(4) {
                        flipped.push(self.0[ele]);
                    }
                }
                let left = Board(flipped).shift(Direction::RIGHT).unwrap();
                let mut flopped = vec![];
                for col in 0..Board::SIZE {
                    for ele in (col..left.0.len()).step_by(4) {
                        flopped.push(left.0[ele]);
                    }
                }
                Some(Board(flopped))
            }
        }
    }
}

fn combine_and_score(row: &[usize]) -> (Vec<usize>, usize) {
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

fn push_zeros(i: &[usize]) -> Vec<usize> {
    let mut new = vec![0; i.len()];
    i.iter()
        .filter(|&f| *f > 0)
        .enumerate()
        .for_each(|(idx, val)| new[idx] = *val);
    new
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row_separator = "+---------".repeat(Board::SIZE) + "+\n";
        let row_spacer = "|         ".repeat(Board::SIZE) + "|\n";

        for row in self.0.chunks(Board::SIZE) {
            write!(f, "{}", row_separator)?;
            write!(f, "{}", row_spacer)?;

            for cell in row.iter() {
                let style = match cell {
                    2 => Style::new().cyan(),
                    4 => Style::new().cyan().dim(),
                    8 => Style::new().cyan().bright(),
                    16 => Style::new().magenta(),
                    32 => Style::new().magenta().dim(),
                    64 => Style::new().magenta().bright(),
                    128 => Style::new().blue(),
                    256 => Style::new().blue().dim(),
                    512 => Style::new().blue().bright(),
                    1024 => Style::new().yellow(),
                    2048 => Style::new().green().bright(),
                    2048.. => Style::new().on_green(),
                    _ => Style::new(),
                };

                let num_out = match cell {
                    0 => String::from(""),
                    _ => cell.to_string(),
                };

                write!(f, "|{:^9}", style.apply_to(num_out))?;
            }
            write!(f, "|\n")?;
            write!(f, "{}", row_spacer)?;
        }
        write!(f, "{}", row_separator)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn board_should_shift() {
        let b = Board(vec![
            8, 8, 0, 2,
            8, 2, 8, 0,
            2, 0, 2, 0,
            2, 4, 4, 0,
        ]);

        let next = b.shift(Direction::LEFT).unwrap();
        assert_eq!(next.0, &[
            16, 2, 0, 0,
            8, 2, 8, 0,
            4, 0, 0, 0,
            2, 8, 0, 0,
        ]);

        let next = b.shift(Direction::RIGHT).unwrap();
        assert_eq!(next.0, &[
            0, 0, 16, 2,
            0, 8, 2, 8,
            0, 0, 0, 4,
            0, 0, 2, 8,
        ]);

        let next = b.shift(Direction::UP).unwrap();
        assert_eq!(next.0, &[
            16, 8, 8, 2,
            4,  2, 2, 0,
            0,  4, 4, 0,
            0,  0, 0, 0,
        ]);

        let next = b.shift(Direction::DOWN).unwrap();
        assert_eq!(next.0, &[
            0,  0, 0, 0,
            0,  8, 8, 0,
            16, 2, 2, 0,
            4,  4, 4, 2,
        ]);
    }

    #[test]
    fn should_push_zeros() {
        assert_eq!(push_zeros(&[0, 0, 0, 0]), &[0, 0, 0, 0]);
        assert_eq!(push_zeros(&[2, 0, 2, 0]), &[2, 2, 0, 0]);
        assert_eq!(push_zeros(&[0, 2, 4, 2]), &[2, 4, 2, 0]);
        assert_eq!(push_zeros(&[0, 0, 7, 0]), &[7, 0, 0, 0]);
    }

    #[test]
    fn should_combine_and_score() {
        assert_eq!(combine_and_score(&[2, 0, 0, 4]), (vec![2, 4, 0, 0], 0));
        assert_eq!(combine_and_score(&[2, 0, 2, 0]), (vec![4, 0, 0, 0], 4));
        assert_eq!(combine_and_score(&[2, 2, 2, 0]), (vec![4, 2, 0, 0], 4));
        assert_eq!(combine_and_score(&[2, 2, 4, 4]), (vec![4, 8, 0, 0], 12));
        assert_eq!(combine_and_score(&[4, 4, 32, 8]), (vec![8, 32, 8, 0], 8));
        assert_eq!(
            combine_and_score(&[0, 256, 256, 8]),
            (vec![512, 8, 0, 0], 512)
        );
    }
}
