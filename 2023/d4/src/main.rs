use std::env;
use std::fs::read_to_string;
use std::io;

mod solver;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    let test = read_to_string("data/test.txt")?;
    let input = read_to_string("data/input.txt")?;

    if args.contains(&String::from("p1")) || args.len() == 1 {
        println!("# Part 1");
        println!("## Test");
        solver::test::part1(&test);
        if !args.contains(&String::from("test")) {
            println!("## Solve");
            println!("{}", solver::part1(&input));
        }
    }

    if args.contains(&String::from("p2")) || args.len() == 1 {
        println!("# Part 2");
        println!("## Test");
        solver::test::part2(&test);
        if !args.contains(&String::from("test")) {
            println!("## Solve");
            println!("{}", solver::part2(&input));
        }
    }

    Ok(())
}
