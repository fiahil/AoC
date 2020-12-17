use std::fs::File;
use std::str::FromStr;

use anyhow::{Error, Result};

mod input;

#[derive(Debug, Default)]
struct Pass {
    row: usize,
    column: usize,
    sid: usize,
}

impl FromStr for Pass {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row, column) = s.split_at(7);

        println!("> row: {} | column {}", row, column);

        let row = row
            .chars()
            .fold((0, 127), |acc, e| {
                let middle = (acc.1 - acc.0) / 2 + acc.0;
                println!("> e {} | acc {:?} | middle {}", e, acc, middle);

                match e {
                    'F' => (acc.0, middle),
                    'B' => (middle + 1, acc.1),
                    _ => unreachable!(),
                }
            })
            .0;

        let column = column
            .chars()
            .fold((0, 8), |acc, e| {
                let middle = (acc.1 - acc.0) / 2 + acc.0;
                println!("> e {} | acc {:?} | middle {}", e, acc, middle);

                match e {
                    'L' => (acc.0, middle),
                    'R' => (middle, acc.1),
                    _ => unreachable!(),
                }
            })
            .0;

        println!("row {:?} | column {:?}", row, column);

        Ok(Pass {
            row,
            column,
            sid: row * 8 + column,
        })
    }
}

fn transform(p: String) -> Result<Vec<Pass>> {
    p.lines().map(|s| Pass::from_str(s)).collect()
}

fn main() -> Result<()> {
    let boarding_passes = input::input(File::open("data/input.txt")?, transform)?;

    println!("{:?}", boarding_passes.iter().max_by_key(|e| e.sid));

    for i in 0..=127 {
        for j in 0..=8 {
            if let None = boarding_passes.iter().find(|p| p.sid == i * 8 + j) {
                println!("! {:?}", i * 8 + j);
            }
        }
    }

    Ok(())
}
