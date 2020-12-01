use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate anyhow;

use anyhow::Result;

fn solve(f: File) -> Result<(i32, i32)> {
    let entries = BufReader::new(f)
        .lines()
        .map(|e| e.unwrap().parse().unwrap())
        .collect::<Vec<i32>>();

    for i in entries.iter() {
        for j in entries.iter() {
            if i + j == 2020 {
                println!("{} + {} = {}\n{} * {} = {}", i, j, i + j, i, j, i * j);
                return Ok((i.clone(), j.clone()));
            }
        }
    }

    Err(anyhow!("Could not find two item that sum to 2020 ;("))
}

fn solve2(f: File) -> Result<(i32, i32, i32)> {
    let entries = BufReader::new(f)
        .lines()
        .map(|e| e.unwrap().parse().unwrap())
        .collect::<Vec<i32>>();

    for i in entries.iter() {
        for j in entries.iter() {
            for k in entries.iter() {
                if i + j + k == 2020 {
                    println!(
                        "{} + {} + {} = {}\n{} * {} * {} = {}",
                        i,
                        j,
                        k,
                        i + j + k,
                        i,
                        j,
                        k,
                        i * j * k
                    );
                    return Ok((i.clone(), j.clone(), k.clone()));
                }
            }
        }
    }

    Err(anyhow!("Could not find two item that sum to 2020 ;("))
}

fn main() -> Result<()> {
    solve(File::open("data/input.txt")?)?;
    solve2(File::open("data/input.txt")?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<()> {
        let (i, j) = solve(File::open("data/test.txt")?)?;

        assert_eq!(i, 1721);
        assert_eq!(j, 299);

        Ok(())
    }

    #[test]
    fn test_2() -> Result<()> {
        let (i, j, k) = solve2(File::open("data/test.txt")?)?;

        assert_eq!(i, 979);
        assert_eq!(j, 366);
        assert_eq!(k, 675);

        Ok(())
    }
}
