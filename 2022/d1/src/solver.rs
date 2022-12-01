use anyhow::Result;

pub fn part1(input: &String) -> Result<i32> {
    let mut elfs = Vec::new();
    let mut sum = 0;

    for line in input.lines() {
        if line.is_empty() {
            elfs.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>()?;
        }
    }
    elfs.push(sum);

    Ok(*elfs.iter().max().unwrap())
}

pub fn part2(input: &String) -> Result<i32> {
    let mut elfs = Vec::new();
    let mut sum = 0;

    for line in input.lines() {
        if line.is_empty() {
            elfs.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>()?;
        }
    }
    elfs.push(sum);

    elfs.sort_by(|a, b| b.cmp(a));

    Ok(elfs.iter().take(3).sum::<i32>())
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 24000);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 45000);

        Ok(())
    }
}
