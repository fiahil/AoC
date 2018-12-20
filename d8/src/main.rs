use std::fs::read_to_string;
use std::io::Result;
use std::path::Path;

extern crate regex;

use regex::Regex;

struct Node {
    children: u32,
    metadata: u32,
    entries: Vec<u32>,
}

fn parse(s: String) -> Vec<u32> {
    s.split(' ')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>()
}

fn follow(input: &Vec<u32>) -> Vec<Node> {
    Vec::new()
}

fn main() -> Result<()> {
    let s = read_to_string(Path::new("data/input2"))?
        .trim_end()
        .to_string();
    let input = parse(s);

    println!("{:?}", input);
    println!("sum = {}", input.iter().sum::<u32>());

    Ok(())
}
