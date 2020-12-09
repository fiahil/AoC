mod input;

use std::fs::File;

use anyhow::{Context, Result};
use either::Either;

fn transform(p: String) -> Result<Vec<usize>> {
    p.lines()
        .inspect(|e| println!("> {:?}", e))
        .map(|e| e.parse().context(format!("Not a number: {}", e)))
        .collect()
}

fn match_pairs(heap: Vec<&usize>, sum: usize) -> Option<(usize, usize)> {
    for (i, e) in heap.iter().enumerate() {
        for (j, f) in heap.iter().enumerate() {
            if *e + *f == sum && i != j {
                return Some((**e, **f));
            }
        }
    }

    None
}

fn validate_xmas(nums: &Vec<usize>, preamble: usize) -> Either<usize, usize> {
    for (i, num) in nums.iter().skip(preamble).enumerate() {
        let heap: Vec<&usize> = nums.iter().skip(i).take(preamble).collect();

        if let None = match_pairs(heap, *num) {
            return Either::Right(*num);
        }
    }

    Either::Left(0)
}

fn find_weakness(nums: &Vec<usize>, num: usize) -> (usize, usize) {
    println!("{:?}", nums);

    for (i, e) in nums.iter().enumerate() {
        let mut j = 1;
        let mut acc = *e;

        while i + j < nums.len() && acc + nums[i + j] <= num {
            acc += nums[i + j];
            j += 1;
        }

        println!("i = {} ; j = {} ; acc = {} ; num = {}", i, j, acc, num);

        if j >= 2 && acc == num {
            let slice: Vec<&usize> = nums.iter().skip(i).take(j).collect();

            println!(">> {:?}", slice);
            return (**slice.iter().min().unwrap(), **slice.iter().max().unwrap());
        }
    }

    (0, 0)
}

fn main() -> Result<()> {
    let nums = input::input(File::open("data/input.txt")?, transform)?;

    if let Either::Right(num) = validate_xmas(&nums, 25) {
        println!("> Found XMAS number: {}", num);

        let (x, y) = find_weakness(&nums, num);

        println!("> Found vulnerability: {} + {} = {}", x, y, x + y);
    }

    Ok(())
}
