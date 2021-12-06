use std::env;
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
    let args: Vec<String> = env::args().collect();

    let test = read_file(PathBuf::from("data/test.txt"))?;
    let input = read_file(PathBuf::from("data/input.txt"))?;

    match (
        args.contains(&String::from("p1")),
        args.contains(&String::from("p2")),
    ) {
        (true, false) => {
            println!("==== Part 1 ====");
            println!("====  Test  ====");
            solver::test::part1(&test)?;
            println!("====  Run   ====");
            solver::part1(&input)?;
        }
        (false, true) => {
            println!("==== Part 2 ====");
            println!("====  Test  ====");
            solver::test::part2(&test)?;
            println!("====  Run   ====");
            solver::part2(&input)?;
        }
        _ => {
            println!("==== Part 1 ====");
            println!("====  Test  ====");
            solver::test::part1(&test)?;
            println!("====  Run   ====");
            solver::part1(&input)?;

            println!("==== Part 2 ====");
            println!("====  Test  ====");
            solver::test::part2(&test)?;
            println!("====  Run   ====");
            solver::part2(&input)?;
        }
    }

    Ok(())
}
