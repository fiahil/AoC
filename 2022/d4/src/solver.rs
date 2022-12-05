use std::ops::RangeInclusive;

use anyhow::Result;

fn into_range(s: &str) -> RangeInclusive<usize> {
    let (low, up) = s.split_once("-").unwrap();

    RangeInclusive::new(low.parse().unwrap(), up.parse().unwrap())
}

pub fn part1(input: &String) -> Result<i32> {
    let c = input.lines().fold(0, |acc, line| {
        let (r1, r2) = line.split_once(",").unwrap();
        let r1 = into_range(r1);
        let r2 = into_range(r2);

        if r1.clone().all(|e| r2.contains(&e)) || r2.clone().all(|e| r1.contains(&e)) {
            acc + 1
        } else {
            acc
        }
    });

    Ok(c)
}

pub fn part2(input: &String) -> Result<i32> {
    let c = input.lines().fold(0, |acc, line| {
        let (r1, r2) = line.split_once(",").unwrap();
        let r1 = into_range(r1);
        let r2 = into_range(r2);

        if r1.clone().any(|e| r2.contains(&e)) || r2.clone().any(|e| r1.contains(&e)) {
            acc + 1
        } else {
            acc
        }
    });

    Ok(c)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 2);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 4);

        Ok(())
    }
}
