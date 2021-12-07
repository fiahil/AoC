use std::cmp::{max, min};

use anyhow::{Context, Result};

#[derive(Debug)]
struct Solution {
    pub cost: u64,
    pub number: u64,
}

fn parse(input: &String) -> Result<Vec<u64>> {
    input
        .trim()
        .split(",")
        .map(|s| s.parse::<u64>().with_context(|| "could not parse int"))
        .collect::<Result<Vec<u64>>>()
}

pub fn part1(input: &String) -> Result<u64> {
    let crabs = parse(input)?;

    let mut i = 0;
    let mut solutions = Vec::new();
    let should_stop_at = crabs.iter().max().unwrap() * 4;

    println!("should stop at {}", should_stop_at);

    while i < should_stop_at {
        let cost = crabs
            .iter()
            .map(|&c| (c as i64 - i as i64).abs() as u64)
            .fold(0, |acc, x| acc + x);

        let s = Solution { number: i, cost };

        if solutions.iter().all(|ss: &Solution| ss.cost > s.cost) {
            println!("Found a new solution: {:?}", s);
            solutions.push(s);
        }

        i += 1;
    }

    let best = solutions.iter().min_by_key(|s| s.cost).unwrap();
    println!("Best solution: {:?}", best);

    Ok(best.cost)
}

fn fuel_cost(start: u64, end: u64) -> u64 {
    let mut i = 1;
    let mut cost = 0;
    let (mut s, e) = (min(start, end), max(start, end));

    while s < e {
        cost += i;
        s += 1;
        i += 1;
    }

    cost
}

pub fn part2(input: &String) -> Result<u64> {
    let crabs = parse(input)?;

    let mut i = 0;
    let mut solutions = Vec::new();
    let should_stop_at = crabs.iter().max().unwrap() * 2;

    println!("should stop at {}", should_stop_at);

    while i < should_stop_at {
        let cost = crabs
            .iter()
            .map(|&c| fuel_cost(c, i))
            .fold(0, |acc, x| acc + x);

        let s = Solution { number: i, cost };

        if solutions.iter().all(|ss: &Solution| ss.cost > s.cost) {
            println!("Found a new solution: {:?}", s);
            solutions.push(s);
        }

        i += 1;
    }

    let best = solutions.iter().min_by_key(|s| s.cost).unwrap();
    println!("Best solution: {:?}", best);

    Ok(best.cost)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 37);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 168);

        Ok(())
    }
}
