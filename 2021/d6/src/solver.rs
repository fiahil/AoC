use anyhow::Result;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::time::Instant;

#[derive(Eq, PartialEq, Hash)]
struct Lanternfish {
    timer: u16,
}

impl fmt::Debug for Lanternfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.timer)
    }
}

impl PartialOrd for Lanternfish {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Lanternfish {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timer.cmp(&other.timer)
    }
}

impl Lanternfish {
    pub fn with_timer(timer: u16) -> Self {
        Lanternfish { timer }
    }
}

pub fn part1(input: &String) -> Result<usize> {
    let mut fishes = input
        .split(',')
        .map(|s| Lanternfish::with_timer(s.trim().parse::<u16>().unwrap()))
        .collect::<Vec<Lanternfish>>();

    println!("Day {:>2}: {:?}", 0, fishes);
    for day in 0..80 {
        let mut new_fishes = Vec::new();

        for fish in fishes.iter_mut() {
            match fish.timer {
                0 => {
                    fish.timer = 6;
                    new_fishes.push(Lanternfish::with_timer(8));
                }
                _ => {
                    fish.timer -= 1;
                }
            }
        }

        fishes.extend(new_fishes);

        if day < 20 {
            println!("Day {:>2}: {:?}", day + 1, fishes);
        } else {
            println!("Day {:>2}: ...", day + 1);
        }
    }

    println!("Counting fishes after {} days: {}", 80, fishes.len());

    Ok(fishes.len())
}

pub fn part2(input: &String, n_days: u16) -> Result<usize> {
    let mut parsed_fishes = input
        .split(',')
        .map(|s| Lanternfish::with_timer(s.trim().parse::<u16>().unwrap()))
        .collect::<Vec<Lanternfish>>();

    parsed_fishes.sort();

    let mut fishes = HashMap::new();
    for (key, group) in &parsed_fishes.into_iter().group_by(|elt| elt.timer) {
        fishes.insert(key, group.count());
    }

    println!("Day {:>2}: {:?}", 0, fishes);
    let start = Instant::now();
    for day in 0..n_days {
        let mut new_fishes = HashMap::new();
        for (k, v) in fishes.into_iter() {
            match k {
                0 => {
                    new_fishes.entry(6).and_modify(|fs| *fs += v).or_insert(v);
                    new_fishes.entry(8).and_modify(|fs| *fs += v).or_insert(v);
                }
                _ => {
                    new_fishes
                        .entry(k - 1)
                        .and_modify(|fs| *fs += v)
                        .or_insert(v);
                }
            }
        }

        fishes = new_fishes;

        if day < 20 {
            println!(
                "Day {:>2}: {:?} | {} fishes",
                day + 1,
                fishes,
                fishes.values().sum::<usize>()
            );
        } else {
            println!(
                "Day {:>2}: ... | {:<12} fishes | {:?}",
                day + 1,
                fishes.values().sum::<usize>(),
                start.elapsed()
            );
        }
    }

    println!(
        "Counting fishes after {} days: {}",
        n_days,
        fishes.values().sum::<usize>()
    );

    Ok(fishes.values().sum())
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 5934);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input, 18)?, 26);
        assert_eq!(super::part2(input, 80)?, 5934);
        assert_eq!(super::part2(input, 256)?, 26_984_457_539);

        Ok(())
    }
}
