use std::collections::HashSet;
use std::collections::VecDeque;

use anyhow::Result;

pub fn part1(input: &String) -> Result<i32> {
    let mut count = 0;
    let mut win = VecDeque::new();

    for char in input.chars() {
        win.push_front(char);
        count += 1;

        if win.len() > 4 {
            win.pop_back();
        }

        if HashSet::<&char>::from_iter(win.iter()).len() == 4 {
            return Ok(count);
        }
    }

    Ok(0)
}

pub fn part2(input: &String) -> Result<i32> {
    let mut count = 0;
    let mut win = VecDeque::new();

    for char in input.chars() {
        win.push_front(char);
        count += 1;

        if win.len() > 14 {
            win.pop_back();
        }

        if HashSet::<&char>::from_iter(win.iter()).len() == 14 {
            return Ok(count);
        }
    }

    Ok(0)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 11);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 26);

        Ok(())
    }
}
