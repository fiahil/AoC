use std::collections::{HashMap, HashSet};

use anyhow::Result;

#[derive(Debug)]
struct Digit {
    segments: HashSet<char>,
}

impl Digit {
    fn new(mut segments: Vec<char>) -> Self {
        segments.sort();

        Self {
            segments: segments.into_iter().collect(),
        }
    }
}

#[derive(Debug)]
struct Display {
    signals: Vec<Digit>,
    output: Vec<Digit>,
}

fn parse(input: &String) -> Vec<Display> {
    input
        .lines()
        .map(|line| {
            let (signals, output) = line.split_once(" | ").unwrap();

            let signals = signals
                .split_whitespace()
                .map(|s| Digit {
                    segments: s.chars().collect(),
                })
                .collect();

            let output = output
                .split_whitespace()
                .map(|s| Digit::new(s.chars().collect()))
                .collect();

            Display { signals, output }
        })
        .collect()
}

fn identify_digit(digit: &Digit) -> Option<u8> {
    match digit.segments.iter().count() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}

pub fn part1(input: &String) -> Result<i32> {
    let displays = parse(input);

    let easy_digit_count = displays
        .iter()
        .map(|display| {
            display
                .output
                .iter()
                .filter(|digit| {
                    identify_digit(digit)
                        .map(|d| match d {
                            1 | 7 | 4 | 8 => true,
                            _ => false,
                        })
                        .unwrap_or(false)
                })
                .count()
        })
        .sum::<usize>();

    println!("counting {} easy digits", easy_digit_count);

    Ok(easy_digit_count as i32)
}

fn identify_digit_with_mapping(digit: &Digit, mapping: &HashMap<u8, HashSet<char>>) -> Option<u8> {
    mapping
        .iter()
        .find(|&(_, v)| v == &digit.segments)
        .map(|(k, _)| *k)
}

fn find_digit(digit: &Digit, mapping: &HashMap<u8, HashSet<char>>) -> Option<(u8, HashSet<char>)> {
    let easy = match digit.segments.iter().count() {
        2 => Some((1, digit.segments.clone())),
        4 => Some((4, digit.segments.clone())),
        3 => Some((7, digit.segments.clone())),
        7 => Some((8, digit.segments.clone())),
        _ => None,
    };

    if easy.is_some() {
        return easy;
    }

    fn ok(key: u8, mapping: &HashMap<u8, HashSet<char>>) -> bool {
        mapping.get(&key).unwrap().len() > 0
    }

    fn get(key: u8, mapping: &HashMap<u8, HashSet<char>>) -> &HashSet<char> {
        mapping.get(&key).unwrap()
    }

    if ok(7, mapping)
        && ok(4, mapping)
        && digit.segments.is_superset(get(4, mapping))
        && digit.segments.is_superset(get(7, mapping))
    {
        return Some((9, digit.segments.clone()));
    }

    if ok(1, mapping)
        && ok(7, mapping)
        && digit.segments.is_superset(get(1, mapping))
        && digit.segments.is_superset(get(7, mapping))
        && digit.segments.len() == 6
    {
        return Some((0, digit.segments.clone()));
    }

    if ok(0, mapping) && !digit.segments.eq(get(0, mapping)) && digit.segments.len() == 6 {
        return Some((6, digit.segments.clone()));
    }

    if ok(6, mapping) && digit.segments.is_subset(get(6, mapping)) && digit.segments.len() == 5 {
        return Some((5, digit.segments.clone()));
    }

    if ok(1, mapping) && digit.segments.is_superset(get(1, mapping)) && digit.segments.len() == 5 {
        return Some((3, digit.segments.clone()));
    }

    if ok(3, mapping) && ok(5, mapping) && digit.segments.len() == 5 {
        return Some((2, digit.segments.clone()));
    }

    None
}

pub fn part2(input: &String) -> Result<i32> {
    let displays = parse(input);

    let mut sum = 0;

    for display in displays.iter() {
        let mut mapping = HashMap::new();
        mapping.insert(0, HashSet::new());
        mapping.insert(1, HashSet::new());
        mapping.insert(2, HashSet::new());
        mapping.insert(3, HashSet::new());
        mapping.insert(4, HashSet::new());
        mapping.insert(5, HashSet::new());
        mapping.insert(6, HashSet::new());
        mapping.insert(7, HashSet::new());
        mapping.insert(8, HashSet::new());
        mapping.insert(9, HashSet::new());

        while mapping.values().any(|v| v.len() == 0) {
            for digit in display.signals.iter() {
                if let Some((d, segments)) = find_digit(digit, &mapping) {
                    println!("found digit {} with pattern {:?}", d, segments);
                    mapping.insert(d, segments);
                }
            }

            println!(
                "mapped {}/10",
                mapping.values().filter(|v| v.len() > 0).count()
            );
        }

        let output = display.output.iter().fold(0usize, |acc, digit| {
            let o = identify_digit_with_mapping(digit, &mapping).unwrap() as usize;
            (acc + o) * 10
        }) / 10;

        println!("display = {:?}", display.output);
        println!("output  = {}", output);
        println!("---");

        sum += output;
    }

    println!("sum = {}", sum);

    Ok(sum as i32)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 26);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 61229);

        Ok(())
    }
}
