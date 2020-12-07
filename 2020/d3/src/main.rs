mod input;

use std::{fmt, fs::File};

use anyhow::Result;

#[derive(Debug, Clone)]
enum Cell {
    Tree,
    Snow,
    X,
    O,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Cell::Snow => write!(f, "."),
            Cell::Tree => write!(f, "#"),
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
        }
    }
}

fn transform(p: String) -> Result<Vec<Vec<Cell>>> {
    p.lines()
        .map(|e| {
            let mut v: Vec<Cell> = e
                .chars()
                .map(|c| match c {
                    '.' => Cell::Snow,
                    '#' => Cell::Tree,
                    _ => unreachable!(),
                })
                .collect();

            v.extend(v.iter().cloned().collect::<Vec<Cell>>());
            v.extend(v.iter().cloned().collect::<Vec<Cell>>());
            v.extend(v.iter().cloned().collect::<Vec<Cell>>());
            v.extend(v.iter().cloned().collect::<Vec<Cell>>());
            v.extend(v.iter().cloned().collect::<Vec<Cell>>());
            v.extend(v.iter().cloned().collect::<Vec<Cell>>());
            v.extend(v.iter().cloned().collect::<Vec<Cell>>());
            v.extend(v.iter().cloned().collect::<Vec<Cell>>());
            v.extend(v.iter().cloned().collect::<Vec<Cell>>());

            Ok(v)
        })
        .collect()
}

fn print_map(map: &Vec<Vec<Cell>>) {
    for line in map.iter() {
        for cell in line.iter() {
            print!("{}", cell);
        }
        println!("");
    }

    println!("\n====\n");
}

fn slide(mut map: Vec<Vec<Cell>>, slope: (usize, usize)) -> usize {
    let mut i = 0;
    let mut j = 0;
    let mut treecount = 0;

    while i + slope.0 < map.len() {
        match map[i + slope.0][j + slope.1] {
            Cell::Snow => map[i + slope.0][j + slope.1] = Cell::O,
            Cell::Tree => {
                map[i + slope.0][j + slope.1] = {
                    treecount += 1;
                    Cell::X
                }
            }
            _ => unreachable!(),
        }

        // println!("{} ; {}", i, j);

        i += slope.0;
        j += slope.1;
    }

    println!("treecount : {}", treecount);

    treecount
}

fn main() -> Result<()> {
    let r = input::input(File::open("data/input.txt")?, transform)?;

    print_map(&r);

    let r = slide(r.clone(), (1, 1))
        * slide(r.clone(), (1, 3))
        * slide(r.clone(), (1, 5))
        * slide(r.clone(), (1, 7))
        * slide(r.clone(), (2, 1));

    // print_map(&r);

    println!("trees: {}", r);

    Ok(())
}
