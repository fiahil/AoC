use std::collections::HashMap;

fn contains_double(i: usize) -> bool {
    let mut number: Vec<u32> = i
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    number.as_mut_slice().sort();

    let mut h: HashMap<u32, u32> = HashMap::new();

    for i in number {
        if h.get(&i).is_none() {
            h.insert(i, 1);
        } else {
            let v = *h.get(&i).unwrap() + 1;
            h.insert(i, v);
        }
    }

    let mut c = false;
    for (_, val) in h.iter() {
        if *val == 2 {
            c = true;
        }
    }

    c
}

fn growing_number(i: usize) -> bool {
    let number: Vec<u32> = i
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    for i in 1..number.len() {
        if number[i - 1] > number[i] {
            return false;
        }
    }

    true
}

fn test(i: usize) {
    println!(
        "{} | double: {}, growing: {}",
        i,
        contains_double(i),
        growing_number(i)
    );
}

fn main() {
    let mut count = 0;

    test(112233);
    test(123444);
    test(111122);

    for i in 152085..670283 {
        if contains_double(i) && growing_number(i) {
            count += 1;
        }
    }

    println!("count : {}", count);
}
