use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;

mod solver;

fn read_file(path: PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() -> Result<()> {
    let test = read_file(PathBuf::from("data/test.txt"))?;
    solver::test_part1(&test, 7)?;
    solver::test_part2(&test, 5)?;

    let input = read_file(PathBuf::from("data/input.txt"))?;
    solver::run_part1(&input)?;
    solver::run_part2(&input)?;

    Ok(())
}
