use std::collections::VecDeque;

use anyhow::Result;

pub fn part1(input: &String) -> Result<i32> {
    let mut score = 0;
    for line in input.lines() {
        let mut stack = VecDeque::new();
        for char in line.chars() {
            match (char, stack.front()) {
                (_, None) => {
                    stack.push_front(char);
                }
                ('(' | '[' | '{' | '<', _) => {
                    stack.push_front(char);
                }
                (')', Some('(')) => {
                    stack.pop_front();
                }
                (']', Some('[')) => {
                    stack.pop_front();
                }
                ('}', Some('{')) => {
                    stack.pop_front();
                }
                ('>', Some('<')) => {
                    stack.pop_front();
                }
                (char, Some(stacked)) => {
                    match char {
                        ')' => score += 3,
                        ']' => score += 57,
                        '}' => score += 1197,
                        '>' => score += 25137,
                        _ => unreachable!(),
                    }

                    println!("Expected: {}  got: {}  score: {}", stacked, char, score);

                    break;
                }
            }
        }
    }

    println!("score: {}", score);
    Ok(score)
}

pub fn part2(input: &String) -> Result<u64> {
    let mut scores = Vec::new();
    let mut skipped;

    for line in input.lines() {
        let mut stack = VecDeque::new();
        skipped = false;

        for char in line.chars() {
            match (char, stack.front()) {
                (_, None) => {
                    stack.push_front(char);
                }
                ('(' | '[' | '{' | '<', _) => {
                    stack.push_front(char);
                }
                (')', Some('(')) => {
                    stack.pop_front();
                }
                (']', Some('[')) => {
                    stack.pop_front();
                }
                ('}', Some('{')) => {
                    stack.pop_front();
                }
                ('>', Some('<')) => {
                    stack.pop_front();
                }
                (_, Some(_)) => {
                    println!("corrupted line, skipping");
                    skipped = true;
                    break;
                }
            }
        }

        if !skipped {
            println!("stack of {}", stack.len());

            let mut score = 0;
            for stacked in stack {
                score *= 5;
                match stacked {
                    '(' => score += 1,
                    '[' => score += 2,
                    '{' => score += 3,
                    '<' => score += 4,
                    _ => unreachable!(),
                }
            }

            scores.push(score);
        }
    }

    scores.sort();

    let score = scores[scores.len() / 2];

    println!("score: {}", score);
    Ok(score)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 26397);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 288957);

        Ok(())
    }
}
