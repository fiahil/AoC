mod input;

use std::fs::File;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone)]
enum Opcode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Opcode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(" ").collect::<Vec<&str>>();

        match (v[0], v[1]) {
            ("nop", x) => Ok(Opcode::Nop(x.parse()?)),
            ("acc", x) => Ok(Opcode::Acc(x.parse()?)),
            ("jmp", x) => Ok(Opcode::Jmp(x.parse()?)),
            _ => unreachable!(),
        }
    }
}

fn transform(p: String) -> Result<Vec<Opcode>> {
    p.lines()
        .inspect(|e| println!("> {:?}", e))
        .map(|e| Opcode::from_str(e))
        .inspect(|e| println!("> {:?}", e))
        .collect()
}

fn run(ops: Vec<Opcode>) -> Result<i32> {
    let mut visited = Vec::new();
    let mut acc = 0;
    let mut i: i32 = 0;

    loop {
        if visited.contains(&i) {
            // println!("i: {} ; visited: {:?}", i, visited);
            return Err(anyhow!("Inifinite Loop! acc = {}", acc));
        }

        if i as usize >= ops.len() {
            return Ok(acc);
        }

        visited.push(i);

        // println!("- {:?}\t(i={}; acc={})", ops[i as usize], i, acc);
        match ops[i as usize] {
            Opcode::Nop(_) => i += 1,
            Opcode::Acc(x) => {
                acc += x;
                i += 1
            }
            Opcode::Jmp(x) => i += x,
        };
    }
}

fn main() -> Result<()> {
    let ops = input::input(File::open("data/input.txt")?, transform)?;

    for i in 0..ops.len() {
        let mut new = ops.clone();

        match new[i] {
            Opcode::Acc(_) => {}
            Opcode::Jmp(x) => new[i] = Opcode::Nop(x),
            Opcode::Nop(x) => new[i] = Opcode::Jmp(x),
        };

        let outcome = run(new);

        println!("outcome: {:?}", outcome);

        if outcome.is_ok() {
            break;
        }
    }

    Ok(())
}
