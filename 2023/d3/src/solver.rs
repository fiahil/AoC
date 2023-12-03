use std::collections::{BTreeMap, HashMap, HashSet};

use anyhow::Result;

pub fn part1(input: &String) -> Result<i32> {
    let mut index: BTreeMap<(isize, isize), u32> = BTreeMap::new();
    let mut symbols: HashMap<(isize, isize), char> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        dbg!(line);
        let mut j = 0;
        while j < line.len() {
            match line.chars().nth(j) {
                Some('.') => {}
                Some(c) if c.is_digit(10) => {
                    index.insert((i as isize, j as isize), c.to_digit(10).unwrap());
                }
                Some(c) => {
                    symbols.insert((i as isize, j as isize), c);
                }
                None => {}
            }
            j += 1;
        }
    }

    let mut parts_numbers = Vec::new();
    let mut other_numbers = Vec::new();

    let mut can_skip = Vec::new();

    // For each digit in the index, find the closest symbol in each direction
    for ((i, j), _) in index.iter() {
        if can_skip.contains(&(*i, *j)) {
            continue;
        }

        can_skip.push((*i, *j));

        let mut close_coordinates: HashSet<(isize, isize)> = HashSet::new();
        fn add_to_close_coordinates(cc: &mut HashSet<(isize, isize)>, ij: (isize, isize)) {
            let (i, j) = ij;
            cc.insert((i + 1, j)); // Down
            cc.insert((i - 1, j)); // Up
            cc.insert((i, j + 1)); // Right
            cc.insert((i, j - 1)); // Left
            cc.insert((i + 1, j + 1)); // Down Right
            cc.insert((i + 1, j - 1)); // Down Left
            cc.insert((i - 1, j + 1)); // Up Right
            cc.insert((i - 1, j - 1)); // Up Left
        }

        // Add the coordinates of the 8 closest cells
        add_to_close_coordinates(&mut close_coordinates, (*i, *j));

        // Investigate the left and right direction, if there is another digit, add its close coordinates to the list
        let num = match (
            index.get(&(*i, *j - 2)),
            index.get(&(*i, *j - 1)),
            index.get(&(*i, *j)),
            index.get(&(*i, *j + 1)),
            index.get(&(*i, *j + 2)),
        ) {
            (Some(a), Some(b), Some(c), None, _) => {
                add_to_close_coordinates(&mut close_coordinates, (*i, *j - 2));
                add_to_close_coordinates(&mut close_coordinates, (*i, *j - 1));
                can_skip.push((*i, *j - 2));
                can_skip.push((*i, *j - 1));
                a * 100 + b * 10 + c
            }
            (None, Some(b), Some(c), Some(d), None) => {
                add_to_close_coordinates(&mut close_coordinates, (*i, *j - 1));
                add_to_close_coordinates(&mut close_coordinates, (*i, *j + 1));
                can_skip.push((*i, *j - 1));
                can_skip.push((*i, *j + 1));
                b * 100 + c * 10 + d
            }
            (_, None, Some(c), Some(d), Some(e)) => {
                add_to_close_coordinates(&mut close_coordinates, (*i, *j + 1));
                add_to_close_coordinates(&mut close_coordinates, (*i, *j + 2));
                can_skip.push((*i, *j + 1));
                can_skip.push((*i, *j + 2));
                c * 100 + d * 10 + e
            }
            (None, Some(b), Some(c), None, _) => {
                add_to_close_coordinates(&mut close_coordinates, (*i, *j - 1));
                can_skip.push((*i, *j - 1));
                b * 10 + c
            }
            (_, None, Some(c), Some(d), None) => {
                add_to_close_coordinates(&mut close_coordinates, (*i, *j + 1));
                can_skip.push((*i, *j + 1));
                c * 10 + d
            }
            (_, None, Some(c), None, _) => *c,
            _ => {
                unreachable!()
            }
        };

        dbg!(num);

        // for each close coordinate, check if it is a symbol
        let mut added = false;
        for (i, j) in close_coordinates.into_iter() {
            if let Some(symbol) = symbols.get(&(i, j)) {
                dbg!(symbol);
                parts_numbers.push(num);
                added = true;
                break;
            }
        }
        if !added {
            other_numbers.push(num);
        }
    }

    let parts_numbers = dbg!(parts_numbers);
    dbg!(other_numbers);

    Ok(parts_numbers.into_iter().sum::<u32>() as i32)
}

pub fn part2(input: &String) -> Result<i32> {
    let mut index: BTreeMap<(isize, isize), u32> = BTreeMap::new();
    let mut symbols: HashMap<(isize, isize), char> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        dbg!(line);
        let mut j = 0;
        while j < line.len() {
            match line.chars().nth(j) {
                Some('.') => {}
                Some(c) if c.is_digit(10) => {
                    index.insert((i as isize, j as isize), c.to_digit(10).unwrap());
                }
                Some(c) => {
                    symbols.insert((i as isize, j as isize), c);
                }
                None => {}
            }
            j += 1;
        }
    }

    let mut gear_ratios = Vec::new();

    for ((i, j), v) in symbols.iter() {
        if v != &'*' {
            continue;
        }

        let mut close_coordinates: HashSet<(isize, isize)> = HashSet::new();
        fn add_to_close_coordinates(cc: &mut HashSet<(isize, isize)>, ij: (isize, isize)) {
            let (i, j) = ij;
            cc.insert((i + 1, j)); // Down
            cc.insert((i - 1, j)); // Up
            cc.insert((i, j + 1)); // Right
            cc.insert((i, j - 1)); // Left
            cc.insert((i + 1, j + 1)); // Down Right
            cc.insert((i + 1, j - 1)); // Down Left
            cc.insert((i - 1, j + 1)); // Up Right
            cc.insert((i - 1, j - 1)); // Up Left
        }

        // Add the coordinates of the 8 closest cells
        add_to_close_coordinates(&mut close_coordinates, (*i, *j));

        let mut digits = Vec::new();
        // for each close coordinate, check if it is a symbol
        for (i, j) in close_coordinates.into_iter() {
            if let Some(_) = index.get(&(i, j)) {
                digits.push((i, j));
            }
        }

        fn get_num(index: &BTreeMap<(isize, isize), u32>, ij: (isize, isize)) -> u32 {
            // Investigate the left and right direction, if there is another digit, add its close coordinates to the list
            let (i, j) = ij;
            match (
                index.get(&(i, j - 2)),
                index.get(&(i, j - 1)),
                index.get(&(i, j)),
                index.get(&(i, j + 1)),
                index.get(&(i, j + 2)),
            ) {
                (Some(a), Some(b), Some(c), None, _) => a * 100 + b * 10 + c,
                (None, Some(b), Some(c), Some(d), None) => b * 100 + c * 10 + d,
                (_, None, Some(c), Some(d), Some(e)) => c * 100 + d * 10 + e,
                (None, Some(b), Some(c), None, _) => b * 10 + c,
                (_, None, Some(c), Some(d), None) => c * 10 + d,
                (_, None, Some(c), None, _) => *c,
                _ => {
                    unreachable!()
                }
            }
        }

        if digits.len() >= 2 {
            let collected = digits
                .into_iter()
                .map(|d| get_num(&index, d))
                .collect::<HashSet<u32>>()
                .into_iter()
                .collect::<Vec<u32>>();

            if collected.len() == 2 {
                gear_ratios.push(collected[0] * collected[1]);
            }

            if collected.len() > 2 {
                unreachable!();
            }
        }
    }

    let gear_ratios = dbg!(gear_ratios);

    Ok(gear_ratios.into_iter().sum::<u32>() as i32)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 4361);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 467835);

        Ok(())
    }
}
