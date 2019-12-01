use math::round;
use std::fs;
use std::io::Result;

fn compute_fuel(weight: i32) -> i32 {
    fn sub(w: f64) -> f64 {
        if w <= 0.0 {
            return 0.0;
        }
        w + sub(round::floor(w / 3.0, 0) - 2.0)
    };

    let initial = round::floor(weight as f64 / 3.0, 0) - 2.0;
    sub(initial) as i32
}

fn main() -> Result<()> {
    let sum: i32 = fs::read_to_string("input.txt")?
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .map(compute_fuel)
        .sum();

    println!("> Sum of fuel {}", sum);

    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(compute_fuel(12), 2);
        assert_eq!(compute_fuel(14), 2);
        assert_eq!(compute_fuel(1969), 966);
        assert_eq!(compute_fuel(100756), 50346);
    }
}
