use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::io::Result;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

extern crate regex;
extern crate threadpool;

use regex::Regex;
use threadpool::ThreadPool;

fn parse(s: String) -> BTreeMap<char, VecDeque<char>> {
    let re = Regex::new(r"^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$").unwrap();
    let mut acc = BTreeMap::<char, VecDeque<char>>::new();

    for e in s.split_terminator('\n') {
        let captures = re.captures(&e).unwrap();

        acc.entry(captures[2].chars().next().unwrap())
            .or_default()
            .push_back(captures[1].chars().next().unwrap());

        if !acc.contains_key(&captures[1].chars().next().unwrap()) {
            acc.entry(captures[1].chars().next().unwrap()).or_default();
        }
    }

    acc
}

fn remove_task(input: &BTreeMap<char, VecDeque<char>>, c: char) -> BTreeMap<char, VecDeque<char>> {
    let mut newinput = BTreeMap::<char, VecDeque<char>>::new();

    for (k, v) in input.iter() {
        if c != *k {
            newinput.insert(
                *k,
                v.iter()
                    .filter_map(|&x| if x != c { Some(x) } else { None })
                    .collect(),
            );
        }
    }

    newinput
}

fn dagify(input: &BTreeMap<char, VecDeque<char>>) {
    for (k, v) in input.iter() {
        // println!("=> {}", k);

        if v.len() == 0 {
            println!("print {}", k);

            let z = *k;

            let new_input = remove_task(input, *k);

            // println!("new input {:?}", new_input);

            dagify(&new_input);
            break;
        }
    }
}

fn passes(input: &BTreeMap<char, VecDeque<char>>) {
    let mut progress = input.clone();
    let mut execute = progress
        .iter()
        .filter_map(|(k, v)| if v.len() == 0 { Some((*k, 0)) } else { None })
        .collect::<VecDeque<(char, u32)>>();

    let mut final_time = 0;

    for tick in 0..1000 {
        println!("--- {} ({} executing)", tick, execute.len());

        if execute.len() == 0 {
            final_time = tick;
            break;
        }

        for e in execute.iter() {
            println!(" {} \t => {} [{}]", tick, e.0, e.1);
        }

        let finished = execute
            .iter()
            .filter_map(|(x, counter)| {
                if (*x as u32) - ('A' as u32) + 60 == tick - *counter {
                    Some(*x)
                } else {
                    None
                }
            })
            .collect::<VecDeque<char>>();

        for e in finished.iter() {
            println!(" {} \t => {} Done!", tick, e);
            progress = remove_task(&progress, *e);
            println!("{:?}", progress);
            let to_be_added = progress
                .iter()
                .filter_map(|(k, v)| {
                    if v.len() == 0 {
                        Some((*k, tick + 1))
                    } else {
                        None
                    }
                })
                .collect::<VecDeque<(char, u32)>>();

            execute.remove(
                *execute
                    .iter()
                    .enumerate()
                    .filter_map(|(i, (k, _))| if k == e { Some(i) } else { None })
                    .collect::<Vec<usize>>()
                    .first()
                    .unwrap(),
            );

            for tba in to_be_added.iter() {
                if execute.len() < 5 && execute.iter().find(|x| (**x).0 == (*tba).0).is_none() {
                    execute.push_back(*tba);
                }
            }
        }
    }

    println!("Final time      = {}", final_time);
}

fn main() -> Result<()> {
    let s = read_to_string(Path::new("data/input"))?;
    let input = parse(s);

    println!("Starting with {:?}", &input);

    let schedule = "MNQKRSFWGXPZJCOTVYEBLAHIUD";

    passes(&input);

    Ok(())
}
