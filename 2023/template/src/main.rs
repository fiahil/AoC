use std::env;
use std::fs::read_to_string;

use anyhow::Result;

mod solver;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let test = read_to_string("data/test.txt")?;
    let input = read_to_string("data/input.txt")?;

    if args.contains(&String::from("p1")) {
        println!("# Part 1");
        println!("## Test");
        solver::test::part1(&test)?;
        if !args.contains(&String::from("test")) {
            println!("## Solve");
            println!("{}", solver::part1(&input)?);
        }
    }

    if args.contains(&String::from("p2")) {
        println!("# Part 2");
        println!("## Test");
        solver::test::part2(&read_to_string("data/test2.txt")?)?;
        if !args.contains(&String::from("test")) {
            println!("## Solve");
            println!("{}", solver::part2(&input)?);
        }
    }

    Ok(())
}
