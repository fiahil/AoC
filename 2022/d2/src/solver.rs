use anyhow::Result;

pub fn part1(input: &String) -> Result<i32> {
    let mut score = 0;

    for line in input.lines() {
        match line.split_once(" ").unwrap() {
            ("A", "X") => score += 3 + 1,
            ("A", "Y") => score += 6 + 2,
            ("A", "Z") => score += 0 + 3,
            ("B", "X") => score += 0 + 1,
            ("B", "Y") => score += 3 + 2,
            ("B", "Z") => score += 6 + 3,
            ("C", "X") => score += 6 + 1,
            ("C", "Y") => score += 0 + 2,
            ("C", "Z") => score += 3 + 3,
            _ => unreachable!(),
        };
    }

    Ok(score)
}

pub fn part2(input: &String) -> Result<i32> {
    let mut score = 0;

    for line in input.lines() {
        match line.split_once(" ").unwrap() {
            ("A", "X") => score += 0 + 3,
            ("A", "Y") => score += 3 + 1,
            ("A", "Z") => score += 6 + 2,
            ("B", "X") => score += 0 + 1,
            ("B", "Y") => score += 3 + 2,
            ("B", "Z") => score += 6 + 3,
            ("C", "X") => score += 0 + 2,
            ("C", "Y") => score += 3 + 3,
            ("C", "Z") => score += 6 + 1,
            _ => unreachable!(),
        };
    }

    Ok(score)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 15);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 12);

        Ok(())
    }
}
