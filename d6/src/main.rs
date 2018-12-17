use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::io::Result;
use std::path::Path;

extern crate regex;

use regex::Regex;

fn parse(s: String) -> Vec<(u32, u32)> {
    let re = Regex::new(r"^(\d+), (\d+)$").unwrap();
    let mut acc = Vec::<(u32, u32)>::new();

    for e in s.split_terminator('\n') {
        if re.is_match(&e) {
            let captures = re.captures(&e);

            match captures {
                Some(x) => acc.push((x[1].parse().unwrap(), x[2].parse().unwrap())),
                None => (),
            }
        }
    }

    acc
}

fn max(seq: &Vec<&u32>) -> u32 {
    let mut i = 0;

    for e in seq.iter() {
        if **e >= i {
            i = **e;
        }
    }

    i
}

fn build_matrice(input: &Vec<(u32, u32)>) -> BTreeMap<(u32, u32), char> {
    let mut hm = BTreeMap::new();
    let mut current_char = 'A';

    for y in 0..=(max(&input.iter().map(|(x, y)| y).collect())) {
        for x in 0..=(max(&input.iter().map(|(x, y)| x).collect())) {
            hm.insert((y, x), '.');
        }
    }

    for (x, y) in input.iter() {
        hm.insert((*y, *x), current_char);
        current_char = (current_char as u8 + 1) as char;
    }

    hm
}

fn distance(x: i32, xx: i32, y: i32, yy: i32) -> u32 {
    let r = (x - xx).abs() + (y - yy).abs();

    r as u32
}

fn compute_distances(
    input: &Vec<(u32, u32)>,
    matrice: &BTreeMap<(u32, u32), char>,
) -> BTreeMap<(u32, u32), char> {
    let mut bmap = BTreeMap::new();

    for ((y, x), v) in matrice.iter() {
        let mut closest_char = BTreeMap::new();

        for (xx, yy) in input.iter() {
            closest_char
                .entry(distance(*x as i32, *xx as i32, *y as i32, *yy as i32))
                .or_insert(Vec::<char>::new())
                .push(*matrice.get(&(*yy, *xx)).unwrap());

            // println!(
            //     "{}, {} | {}, {} ({}) => {}",
            //     x,
            //     y,
            //     xx,
            //     yy,
            //     matrice.get(&(*yy, *xx)).unwrap(),
            //     distance(*x as i32, *xx as i32, *y as i32, *yy as i32)
            // );
        }

        match closest_char.iter().next() {
            Some((k, v)) if v.len() == 1 => bmap.insert((*y, *x), *v.first().unwrap()),
            Some(_) => bmap.insert((*y, *x), '.'),
            None => None,
        };
    }

    bmap
}

fn compute_distances2(
    input: &Vec<(u32, u32)>,
    matrice: &BTreeMap<(u32, u32), char>,
) -> BTreeMap<(u32, u32), char> {
    let mut bmap = BTreeMap::new();

    for ((y, x), v) in matrice.iter() {
        let mut sum = 0;

        for (xx, yy) in input.iter() {
            sum += distance(*x as i32, *xx as i32, *y as i32, *yy as i32)
        }

        if sum < 10000 {
            bmap.insert((*y, *x), '#');
        } else {
            bmap.insert((*y, *x), '.');
        }
    }

    bmap
}

fn print_matrice(matrice: &BTreeMap<(u32, u32), char>) {
    let mut prev = 0;

    for ((y, x), v) in matrice.iter() {
        // println!("{}, {} => {}", x, y, v);

        if *y > prev {
            print!("\n");
            prev = *y;
        }
        print!("{}", v);
    }

    print!("\n====\n");
}

fn main() -> Result<()> {
    let s = read_to_string(Path::new("data/input"))?;
    let input = parse(s);

    println!("{:?}", &input);

    let matrice = build_matrice(&input);

    print_matrice(&matrice);

    let new_matrice = compute_distances2(&input, &matrice);

    print_matrice(&new_matrice);

    let counter = new_matrice.values().fold(BTreeMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += if *c == '#' { 1 } else { 0 };
        acc
    });

    println!("{:?}", counter);

    // for (k, v) in counter.iter() {
    //     println!("{}: {}", *v, *k);
    // }

    Ok(())
}
