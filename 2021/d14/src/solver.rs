use anyhow::Result;
use flume;
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

type Rules = HashMap<[char; 2], char>;

fn parse(input: &String) -> (Vec<char>, Rules) {
    let (polymer_str, rules_str) = input.split_once("\n\n").unwrap();

    let polymer = polymer_str.chars().collect();
    let rules = rules_str
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let mut left = left.chars();
            (
                [left.next().unwrap(), left.next().unwrap()],
                right.chars().next().unwrap(),
            )
        })
        .collect();

    (polymer, rules)
}

struct Explorer {
    start_pair: [char; 2],
    rules: Rules,
    steps: usize,
    sender: flume::Sender<[char; 2]>,
}

impl Explorer {
    fn new(
        pair: [char; 2],
        rules: &Rules,
        steps: usize,
        sender: &flume::Sender<[char; 2]>,
    ) -> Explorer {
        Explorer {
            start_pair: pair,
            rules: rules.clone(),
            sender: sender.clone(),
            steps,
        }
    }

    fn start(self) {
        let id = self.start_pair.clone();

        thread::spawn(move || {
            println!("Explorer {:?}: starting", self.start_pair);
            self.explore(&self.start_pair, 0);
            println!("Explorer {:?}: exploration completed", id);
        });
    }

    fn explore<'a>(&self, pair: &'a [char; 2], step: usize) {
        self.sender.send([pair[0], pair[1]]).unwrap();

        if step + 1 == self.steps {
            return;
        }

        self.explore(&[pair[0], self.rules[&[pair[0], pair[1]]]], step + 1);
        self.explore(&[self.rules[&[pair[0], pair[1]]], pair[1]], step + 1);
    }
}

pub fn part1(input: &String, steps: usize) -> Result<usize> {
    let (polymer, rules) = parse(input);

    let mut elements = HashMap::new();

    let (tx, rx) = flume::bounded(10_000_000);

    for pair in polymer.windows(2) {
        let explorer = Explorer::new([pair[0], pair[1]], &rules, steps, &tx);

        explorer.start();
    }

    drop(tx);
    println!("Counting elements");

    let timer = Duration::from_secs(10);
    let mut start = Instant::now();
    let mut counter = 0;
    while let Ok(msg) = rx.recv() {
        elements.entry(msg).and_modify(|ee| *ee += 1).or_insert(1);
        counter += 1;

        if start.elapsed() > timer {
            println!(
                "processed {} messages in {} seconds (queue: {})",
                counter,
                timer.as_secs(),
                rx.len()
            );
            println!("> {:?}", elements);

            start = Instant::now();
            counter = 0;
        }
    }

    println!("> {:?}", elements);

    let mut flattened = HashMap::new();
    for elt in polymer {
        flattened.entry(elt).and_modify(|e| *e += 1).or_insert(1);
    }

    for ([p1, p2], count) in elements.into_iter() {
        flattened
            .entry(rules[&[p1, p2]])
            .and_modify(|e| *e += count)
            .or_insert(count);
    }

    println!("> {:?}", flattened);

    let least_common = flattened.iter().min_by_key(|(_, count)| *count).unwrap();
    let most_common = flattened.iter().max_by_key(|(_, count)| *count).unwrap();

    println!("Most common : {:?}", most_common);
    println!("Least common: {:?}", least_common);
    println!("Diff        : {}", most_common.1 - least_common.1);

    Ok(most_common.1 - least_common.1)
}

pub fn part2(input: &String) -> Result<usize> {
    let (polymer, rules) = parse(input);

    let mut elements = HashMap::new();
    let mut flattened = HashMap::new();

    for pair in polymer.windows(2) {
        elements
            .entry([pair[0], pair[1]])
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for elt in polymer {
        flattened.entry(elt).and_modify(|e| *e += 1).or_insert(1);
    }

    for _ in 0..40 {
        let mut new_elements = HashMap::new();

        for (pair, count) in elements.into_iter() {
            flattened
                .entry(rules[&[pair[0], pair[1]]])
                .and_modify(|e| *e += count)
                .or_insert(count);

            new_elements
                .entry([pair[0], rules[&[pair[0], pair[1]]]])
                .and_modify(|e| *e += count)
                .or_insert(count);

            new_elements
                .entry([rules[&[pair[0], pair[1]]], pair[1]])
                .and_modify(|e| *e += count)
                .or_insert(count);
        }

        println!("> {:?}", flattened);
        println!("");

        elements = new_elements;
    }

    println!("> {:?}", flattened);

    let least_common = flattened.iter().min_by_key(|(_, count)| *count).unwrap();
    let most_common = flattened.iter().max_by_key(|(_, count)| *count).unwrap();

    println!("Most common : {:?}", most_common);
    println!("Least common: {:?}", least_common);
    println!("Diff        : {}", most_common.1 - least_common.1);

    Ok(most_common.1 - least_common.1)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input, 10)?, 1588);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 2188189693529);

        Ok(())
    }
}
