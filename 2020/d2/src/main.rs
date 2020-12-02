use std::fs::File;

use anyhow::{Context, Result};
use regex::{Captures, Regex};

mod input;
use input::input;

#[derive(Debug)]
struct Pwd {
    pub rule: (usize, usize),
    pub letter: char,
    pub password: String,
}

fn transform(p: String) -> Result<Vec<Pwd>> {
    fn get<'a>(
        captures: &Captures<'a>,
        group: usize,
        line: usize,
        context: &str,
    ) -> Result<&'a str> {
        let s = captures
            .get(group)
            .context(format!("Line {}: {}", line, context))?
            .as_str();

        Ok(s)
    }

    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)")?;

    p.lines()
        .enumerate()
        .map(|(i, e)| {
            let captures = re.captures(e).context(format!("Line {}: captures", i))?;

            let pwd = Pwd {
                rule: (
                    get(&captures, 1, i, "rules lhs")?.parse()?,
                    get(&captures, 2, i, "rules rhs")?.parse()?,
                ),
                letter: get(&captures, 3, i, "letter")?
                    .chars()
                    .next()
                    .context(format!("Line {}: char", i))?,
                password: get(&captures, 4, i, "password")?.to_string(),
            };

            Ok(pwd)
        })
        .collect()
}

impl Pwd {
    fn is_ok(&self) -> bool {
        let count = self.password.chars().filter(|e| e == &self.letter).count();

        count >= self.rule.0 && count <= self.rule.1
    }

    fn is_ok2(&self) -> bool {
        match (
            self.password.chars().nth(self.rule.0 - 1).unwrap(),
            self.password.chars().nth(self.rule.1 - 1).unwrap(),
        ) {
            (x, y) if x == self.letter && y != self.letter => true,
            (x, y) if x != self.letter && y == self.letter => true,
            (x, y) if x == self.letter && y == self.letter => false,
            (x, y) if x != self.letter && y != self.letter => false,
            _ => false,
        }
    }
}

fn main() -> Result<()> {
    let r = input(File::open("data/input.txt")?, transform)?;

    let l = r.iter().filter(|e| e.is_ok()).count();

    println!("{}", l);

    let l = r.iter().filter(|e| e.is_ok2()).count();

    println!("{}", l);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() -> Result<()> {
        let r = input(File::open("data/test.txt")?, transform)?;

        println!("{:?}", r);

        assert_eq!(r.iter().filter(|e| e.is_ok()).count(), 2);

        Ok(())
    }
}
