mod input;

use std::convert::TryFrom;
use std::fs::File;
use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};
use regex::Regex;

#[derive(Debug, Clone)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct RawPassport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl FromStr for RawPassport {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let passport = s.split(" ").fold(RawPassport::default(), |mut pass, e| {
            match e.replace(":", "").split_at(3) {
                ("byr", x) => pass.byr.replace(x.to_string()),
                ("iyr", x) => pass.iyr.replace(x.to_string()),
                ("eyr", x) => pass.eyr.replace(x.to_string()),
                ("hgt", x) => pass.hgt.replace(x.to_string()),
                ("hcl", x) => pass.hcl.replace(x.to_string()),
                ("ecl", x) => pass.ecl.replace(x.to_string()),
                ("pid", x) => pass.pid.replace(x.to_string()),
                ("cid", x) => pass.cid.replace(x.to_string()),
                _ => unreachable!(),
            };
            pass
        });

        Ok(passport)
    }
}

fn byr(x: Option<String>) -> Result<String> {
    let x = x.context("Missing byr")?.parse::<i32>()?;

    if x >= 1920 && x <= 2002 {
        Ok(x.to_string())
    } else {
        Err(anyhow!("byr out of bounds"))
    }
}

fn iyr(x: Option<String>) -> Result<String> {
    let x = x.context("Missing iyr")?.parse::<i32>()?;

    if x >= 2010 && x <= 2020 {
        Ok(x.to_string())
    } else {
        Err(anyhow!("iyr out of bounds"))
    }
}

fn eyr(x: Option<String>) -> Result<String> {
    let x = x.context("Missing eyr")?.parse::<i32>()?;

    if x >= 2020 && x <= 2030 {
        Ok(x.to_string())
    } else {
        Err(anyhow!("eyr out of bounds"))
    }
}

fn hgt(x: Option<String>) -> Result<String> {
    let x = x.context("Missing hgt")?;
    let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();

    let x = re.captures(&x).ok_or_else(|| anyhow!("hgt invalid"))?;

    match (x.get(1).map(|m| m.as_str()), x.get(2).map(|m| m.as_str())) {
        (Some(d), Some("in")) => {
            let d: i32 = d.parse()?;
            if d >= 59 && d <= 76 {
                Ok(d.to_string())
            } else {
                Err(anyhow!("hgt out of bounds"))
            }
        }
        (Some(d), Some("cm")) => {
            let d: i32 = d.parse()?;
            if d >= 150 && d <= 193 {
                Ok(d.to_string())
            } else {
                Err(anyhow!("hgt out of bounds"))
            }
        }
        _ => Err(anyhow!("hgt bad format")),
    }
}

fn hcl(x: Option<String>) -> Result<String> {
    let x = x.context("Missing hcl")?;
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

    if re.is_match(&x) {
        Ok(x.to_string())
    } else {
        Err(anyhow!("hcl bad format"))
    }
}

fn ecl(x: Option<String>) -> Result<String> {
    let x = x.context("Missing ecl")?;
    let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();

    if re.is_match(&x) {
        Ok(x.to_string())
    } else {
        Err(anyhow!("ecl bad format"))
    }
}

fn pid(x: Option<String>) -> Result<String> {
    let x = x.context("Missing pid")?;
    let re = Regex::new(r"^\d{9}$").unwrap();

    if re.is_match(&x) {
        Ok(x.to_string())
    } else {
        Err(anyhow!("pid bad format"))
    }
}

impl TryFrom<RawPassport> for Passport {
    type Error = Error;

    fn try_from(raw: RawPassport) -> Result<Self, Self::Error> {
        let p = Passport {
            byr: byr(raw.byr)?,
            iyr: iyr(raw.iyr)?,
            eyr: eyr(raw.eyr)?,
            hgt: hgt(raw.hgt)?,
            hcl: hcl(raw.hcl)?,
            ecl: ecl(raw.ecl)?,
            pid: pid(raw.pid)?,
            cid: raw.cid,
        };

        Ok(p)
    }
}

fn transform(p: String) -> Result<Vec<RawPassport>> {
    p.split("\n\n")
        .map(|e| e.replace("\n", " ").trim().to_string())
        .inspect(|e| println!("> {}", e))
        .map(|e| RawPassport::from_str(&e))
        .inspect(|e| println!("> {:?}", e))
        .collect()
}

fn main() -> Result<()> {
    let rawpassports = input::input(File::open("data/input.txt")?, transform)?;

    println!("");

    let passports = rawpassports
        .into_iter()
        .inspect(|e| println!("> {:?}", e))
        .map(|r| Passport::try_from(r))
        .inspect(|e| println!("> {:?}", e))
        .filter(Result::is_ok)
        .collect::<Vec<Result<Passport>>>();

    println!("> count: {}", passports.len());

    Ok(())
}
