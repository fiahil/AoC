use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone)]
pub enum Com {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl TryFrom<&str> for Com {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let (first, second) = s.split_once(' ').unwrap();
        let second = second.parse::<i32>()?;

        match first {
            "up" => Ok(Com::Up(second)),
            "down" => Ok(Com::Down(second)),
            "forward" => Ok(Com::Forward(second)),
            _ => Err(anyhow!("invalid command")),
        }
    }
}

pub fn part1(input: &String) -> Result<i32> {
    let mut pos = 0;
    let mut depth = 0;

    println!("Submarine at -> {} x {}m", pos, depth);
    for line in input.lines() {
        let com = Com::try_from(line)?;

        match com {
            Com::Up(n) => {
                depth -= n;
            }
            Com::Down(n) => {
                depth += n;
            }
            Com::Forward(n) => {
                pos += n;
            }
        }
    }
    println!("Submarine at -> {} x {}m (= {})", pos, depth, pos * depth);

    Ok(depth * pos)
}

pub fn part2(input: &String) -> Result<i32> {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    println!("Submarine at -> {} x {}m", pos, depth);
    for line in input.lines() {
        let com = Com::try_from(line)?;

        match com {
            Com::Up(n) => {
                aim -= n;
            }
            Com::Down(n) => {
                aim += n;
            }
            Com::Forward(n) => {
                pos += n;
                depth += n * aim;
            }
        }
    }
    println!("Submarine at -> {} x {}m (= {})", pos, depth, pos * depth);

    Ok(depth * pos)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 150);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 900);

        Ok(())
    }
}
