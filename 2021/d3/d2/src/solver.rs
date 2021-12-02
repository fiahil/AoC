use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone)]
pub enum Ops {}

impl TryFrom<&str> for Ops {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        todo!()
    }
}

pub fn part1(input: &String) -> Result<i32> {
    Ok(0)
}

pub fn part2(input: &String) -> Result<i32> {
    Ok(0)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 1);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 1);

        Ok(())
    }
}
