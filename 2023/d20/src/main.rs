use std::env;
use std::fs::read_to_string;
use std::io;

mod solver;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    let input = read_to_string("data/input.txt")?;

    if args.contains(&String::from("p1")) || args.len() == 1 {
        println!("{}", solver::part1(&input));
    }

    if args.contains(&String::from("p2")) || args.len() == 1 {
        println!("{}", solver::part2(&input));
    }

    Ok(())
}
