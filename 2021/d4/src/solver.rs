use std::fmt;

use anyhow::Result;

#[derive(Debug, Clone, Copy)]
enum Bingo {
    Empty(i32),
    Called(i32),
}

struct BingoBoard {
    board: [[Bingo; 5]; 5],
    wasted: bool,
}

impl fmt::Debug for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "BingoBoard [")?;
        for row in &self.board {
            for cell in row {
                match cell {
                    Bingo::Empty(n) => write!(f, "{:>2}-  ", n)?,
                    Bingo::Called(n) => write!(f, "{:>2}!  ", n)?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "]")
    }
}

#[derive(Debug)]
struct BingoGame {
    numbers: Vec<i32>,
    boards: Vec<BingoBoard>,
}

impl TryFrom<&str> for BingoBoard {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self> {
        let mut board = [[Bingo::Empty(0); 5]; 5];

        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.split_whitespace().enumerate() {
                board[i][j] = Bingo::Empty(c.parse()?);
            }
        }

        Ok(BingoBoard {
            board,
            wasted: false,
        })
    }
}

impl TryFrom<&String> for BingoGame {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let mut blocks = value.split("\n\n");

        let numbers = blocks
            .next()
            .ok_or(anyhow::anyhow!("no numbers"))?
            .split(',')
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        let boards = blocks
            .map(|b| BingoBoard::try_from(b))
            .collect::<Result<Vec<BingoBoard>, _>>()?;

        Ok(BingoGame { numbers, boards })
    }
}

impl BingoBoard {
    fn is_wasted(&self) -> bool {
        self.wasted
    }

    fn set_wasted(&mut self) {
        self.wasted = true;
    }

    fn is_bingo(&self) -> bool {
        for row in &self.board {
            if row.iter().all(|cell| match cell {
                Bingo::Called(_) => true,
                _ => false,
            }) {
                return true;
            }
        }

        for col in 0..5 {
            if self
                .board
                .iter()
                .map(|row| row[col])
                .all(|cell| match cell {
                    Bingo::Called(_) => true,
                    _ => false,
                })
            {
                return true;
            }
        }

        return false;
    }

    fn call(&mut self, n: i32) {
        for row in &mut self.board {
            for cell in row {
                if let Bingo::Empty(m) = cell {
                    if *m == n {
                        *cell = Bingo::Called(n);
                    }
                }
            }
        }
    }

    fn sum_empty(&self) -> i32 {
        self.board
            .iter()
            .flatten()
            .filter_map(|cell| match cell {
                Bingo::Empty(n) => Some(*n),
                _ => None,
            })
            .sum()
    }
}

pub fn part1(input: &String) -> Result<i32> {
    let mut game = BingoGame::try_from(input)?;

    println!("{:#?}", game);

    for number in &game.numbers {
        for board in &mut game.boards {
            board.call(*number);

            if board.is_bingo() {
                println!("BINGO ! : {:?}", board);
                println!(
                    "sum {} * number {} = {}",
                    board.sum_empty(),
                    number,
                    board.sum_empty() * number
                );
                return Ok(board.sum_empty() * number);
            }
        }
    }

    Ok(0)
}

pub fn part2(input: &String) -> Result<i32> {
    let mut game = BingoGame::try_from(input)?;

    println!("{:#?}", game);

    let mut count_of_bingo = game.boards.len();
    for number in &game.numbers {
        for board in &mut game.boards {
            board.call(*number);

            if board.is_bingo() && !board.is_wasted() {
                println!("BINGO ! : {:?}", board);

                board.set_wasted();
                count_of_bingo -= 1;

                if count_of_bingo == 0 {
                    println!(
                        "sum {} * number {} = {}",
                        board.sum_empty(),
                        number,
                        board.sum_empty() * number
                    );

                    return Ok(board.sum_empty() * number);
                }
            }
        }
    }

    Ok(0)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 4512);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 1924);

        Ok(())
    }
}
