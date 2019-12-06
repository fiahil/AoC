use std::collections::HashMap;
use std::fs;

fn walk<'a>(h: &'a HashMap<&str, Vec<&str>>, node: &'a str, count: usize) -> usize {
    println!("Starting walk on {} with {}", node, count);
    if let None = h.get(node) {
        return count;
    }

    let mut c = 0;
    for vs in h[node].clone() {
        let x = walk(h, vs, count + 1);
        c += x;
    }

    println!("Total for {} = {}", node, count + c);
    count + c
}

fn walk1<'a>(
    h: &'a HashMap<&str, Vec<&str>>,
    node: &'a str,
    to: &'a str,
    count: usize,
) -> Option<usize> {
    println!("Starting walk1 on {} with {}", node, count);

    if node == to {
        return Some(count);
    }

    if let None = h.get(node) {
        return None;
    }

    for vs in h[node].clone() {
        if let Some(x) = walk1(h, vs, to, count + 1) {
            return Some(x);
        }
    }
    None
}

fn walk2<'a>(h: &'a HashMap<&str, Vec<&str>>, node: &'a str, t1: &'a str, t2: &'a str) -> usize {
    println!("Starting walk2 on {}", node);
    let opt1 = walk1(h, node, t1, 0);
    let opt2 = walk1(h, node, t2, 0);
    if opt1.is_some() && opt2.is_some() {
        return opt1.unwrap() + opt2.unwrap();
    }

    println!("Not found, going up;");
    return walk2(
        h,
        h.iter().find(|e| e.1.iter().any(|c| *c == node)).unwrap().0,
        t1,
        t2,
    );
}

fn main() {
    let f = fs::read_to_string("input.txt").unwrap();
    //     let f = String::from(
    //         "COM)B
    // B)C
    // C)D
    // D)E
    // E)F
    // B)G
    // G)H
    // D)I
    // E)J
    // J)K
    // K)L
    // K)YOU
    // I)SAN
    // ",
    //     );

    let mut hash: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in f.split_terminator('\n') {
        let ll = l.split(')').collect::<Vec<&str>>();
        let parent = ll[0];
        let child = ll[1];

        hash.entry(parent).or_insert(Vec::new()).push(child);
    }

    let root = "COM";
    println!("{}", walk(&hash, root, 0));
    println!("{}", walk2(&hash, "ZML", "ZML", "939"));
}
