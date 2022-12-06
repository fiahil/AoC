use std::collections::VecDeque;

use anyhow::Result;
use regex::Regex;

pub fn part1(input: &String) -> Result<i32> {
    let mut stacks = vec![
        VecDeque::from(["B", "V", "W", "T", "Q", "N", "H", "D"]),
        VecDeque::from(["B", "W", "D"]),
        VecDeque::from(["C", "J", "W", "Q", "S", "T"]),
        VecDeque::from(["P", "T", "Z", "N", "R", "J", "F"]),
        VecDeque::from(["T", "S", "M", "J", "V", "P", "G"]),
        VecDeque::from(["N", "T", "F", "W", "B"]),
        VecDeque::from(["N", "V", "H", "F", "Q", "D", "L", "B"]),
        VecDeque::from(["R", "F", "P", "H"]),
        VecDeque::from(["H", "P", "N", "L", "B", "M", "S", "Z"]),
    ];

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in input.lines().skip(10) {
        let captures = re.captures(line).unwrap();

        let mut i = captures.get(1).unwrap().as_str().parse::<usize>()?;
        let from = captures.get(2).unwrap().as_str().parse::<usize>()? - 1;
        let to = captures.get(3).unwrap().as_str().parse::<usize>()? - 1;

        while i > 0 {
            let e = stacks[from].pop_front().unwrap();

            stacks[to].push_front(e);

            i -= 1;
        }
    }

    for e in stacks {
        print!("{}", e.front().unwrap_or(&""));
    }
    println!("");

    Ok(1)
}

pub fn part2(input: &String) -> Result<i32> {
    let mut stacks = vec![
        VecDeque::from(["B", "V", "W", "T", "Q", "N", "H", "D"]),
        VecDeque::from(["B", "W", "D"]),
        VecDeque::from(["C", "J", "W", "Q", "S", "T"]),
        VecDeque::from(["P", "T", "Z", "N", "R", "J", "F"]),
        VecDeque::from(["T", "S", "M", "J", "V", "P", "G"]),
        VecDeque::from(["N", "T", "F", "W", "B"]),
        VecDeque::from(["N", "V", "H", "F", "Q", "D", "L", "B"]),
        VecDeque::from(["R", "F", "P", "H"]),
        VecDeque::from(["H", "P", "N", "L", "B", "M", "S", "Z"]),
    ];

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in input.lines().skip(10) {
        let captures = re.captures(line).unwrap();

        let mut i = captures.get(1).unwrap().as_str().parse::<usize>()?;
        let from = captures.get(2).unwrap().as_str().parse::<usize>()? - 1;
        let to = captures.get(3).unwrap().as_str().parse::<usize>()? - 1;

        let mut stack = VecDeque::new();
        while i > 0 {
            let e = stacks[from].pop_front().unwrap();
            stack.push_front(e);
            i -= 1;
        }
        for e in stack {
            stacks[to].push_front(e);
        }
    }

    for e in stacks {
        print!("{}", e.front().unwrap_or(&""));
    }
    println!("");

    Ok(1)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 1);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 1);

        Ok(())
    }
}
