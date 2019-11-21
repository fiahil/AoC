use std::fs::read_to_string;
use std::io::Result;
use std::path::Path;

#[derive(Debug)]
struct Node {
    children: usize,
    metadata: usize,
    family: Vec<Node>,
    entries: Vec<usize>,
}

fn parse(s: String) -> Vec<usize> {
    s.split(' ')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>()
}

fn count_entries(input: &Node) -> usize {
    2 + input.metadata + input.family.iter().map(|n| count_entries(n)).sum::<usize>()
}

fn sum_entries(input: &Node) -> usize {
    input.entries.iter().sum::<usize>() + input.family.iter().map(|n| sum_entries(n)).sum::<usize>()
}

fn root_value(input: &Node) -> usize {
    if input.children == 0 {
        input.entries.iter().sum::<usize>()
    } else {
        input
            .entries
            .iter()
            .map(|n| match input.family.iter().nth(*n - 1) {
                None => 0,
                Some(x) => root_value(x),
            })
            .sum::<usize>()
    }
}

fn follow(input: &Vec<usize>) -> Node {
    let (children, metadata) = (input[0], input[1]);

    // println!("follow: {:?}", input);

    let mut n = Node {
        children,
        metadata,
        family: Vec::new(),
        entries: Vec::new(),
    };

    let mut i = 2;
    for _ in 0..children {
        let tail = follow(&input.iter().skip(i).cloned().collect());

        i += count_entries(&tail);

        // println!("pushing tail {:?}", tail);

        n.family.push(tail);
    }

    n.entries.extend_from_slice(&input[i..(i + metadata)]);

    n
}

fn main() -> Result<()> {
    let s = read_to_string(Path::new("data/input"))?
        .trim_end()
        .to_string();
    let input = parse(s);
    let nodes = follow(&input);

    println!("{:?}", input);
    println!("{:?}", nodes);
    println!("sum = {}", sum_entries(&nodes));
    println!("Root Value = {}", root_value(&nodes));

    Ok(())
}
