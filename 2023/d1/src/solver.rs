use std::collections::VecDeque;

use anyhow::Result;

pub fn part1(input: &String) -> Result<i32> {
    let mut store = Vec::new();
    for line in input.lines() {
        let filtered = line.chars().filter(|c| c.is_numeric()).collect::<String>();
        let filtered = filtered
            .chars()
            .take(1)
            .chain(filtered.chars().rev().take(1))
            .collect::<String>();
        let num = filtered.parse::<i32>()?;
        println!("{}", num);
        store.push(num);
    }
    Ok(store.iter().sum())
}

fn auto_match_3(s: &str) -> Option<char> {
    match s {
        "one" => Some('1'),
        "two" => Some('2'),
        "six" => Some('6'),
        _ => None,
    }
}

fn auto_match_4(s: &str) -> Option<char> {
    match s {
        "four" => Some('4'),
        "five" => Some('5'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn auto_match_5(s: &str) -> Option<char> {
    match s {
        "three" => Some('3'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        _ => None,
    }
}

fn auto_match(s: &str) -> Option<char> {
    if let Some(num) = auto_match_3(s) {
        return Some(num);
    }
    if let Some(num) = auto_match_4(s) {
        return Some(num);
    }
    if let Some(num) = auto_match_5(s) {
        return Some(num);
    }
    None
}

pub fn part2(input: &String) -> Result<i32> {
    let mut store = Vec::new();
    for line in input.lines() {
        let mut filtered = Vec::new();
        let mut stack = VecDeque::new();
        println!("-> {}", line);
        for c in line.chars() {
            if c.is_numeric() {
                while stack.len() > 0 {
                    println!("{:?}", stack);
                    if let Some(num) = auto_match(&stack.iter().take(3).collect::<String>()) {
                        filtered.push(num);
                    }
                    if let Some(num) = auto_match(&stack.iter().take(4).collect::<String>()) {
                        filtered.push(num);
                    }
                    if let Some(num) = auto_match(&stack.iter().take(5).collect::<String>()) {
                        filtered.push(num);
                    }
                    if stack.len() > 0 {
                        stack.pop_front();
                    }
                }
                filtered.push(c);
                continue;
            }
            stack.push_back(c);
            if let Some(num) = auto_match(&stack.iter().collect::<String>()) {
                filtered.push(num);
                stack.clear();
            }
        }
        while stack.len() > 0 {
            println!("{:?}", stack);
            if let Some(num) = auto_match(&stack.iter().take(3).collect::<String>()) {
                filtered.push(num);
            }
            if let Some(num) = auto_match(&stack.iter().take(4).collect::<String>()) {
                filtered.push(num);
            }
            if let Some(num) = auto_match(&stack.iter().take(5).collect::<String>()) {
                filtered.push(num);
            }
            if stack.len() > 0 {
                stack.pop_front();
            }
        }

        println!("= {:?}", filtered);
        if filtered.len() == 1 {
            println!("stop");
        }
        let filtered = filtered
            .iter()
            .take(1)
            .chain(filtered.iter().rev().take(1))
            .collect::<String>();
        let num = filtered.parse::<i32>()?;
        println!("{}", num);
        store.push(num);
    }
    Ok(store.iter().sum())
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 142);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 281);

        Ok(())
    }
}
