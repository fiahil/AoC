mod input;

use std::{collections::VecDeque, fs::File};

use anyhow::Result;

#[derive(Debug, Clone)]
enum Expr {
    Val(u64),
    Plus,
    Mult,
    Open,
    Close,
}

#[derive(Debug, Clone, Default)]
struct Root {
    root: Vec<Expr>,
}

fn transform(p: String) -> Result<Vec<Root>> {
    p.lines().map(|l| parse(l)).collect()
}

fn parse(l: &str) -> Result<Root> {
    let ops = l
        .chars()
        .filter(|e| *e != ' ')
        .map(|op| match op {
            '+' => Expr::Plus,
            '*' => Expr::Mult,
            '(' => Expr::Open,
            ')' => Expr::Close,
            v => Expr::Val(v.to_string().parse().unwrap()),
        })
        .collect::<Vec<_>>();

    Ok(Root { root: ops })
}

fn execute(r: Root) -> u64 {
    let mut acc = VecDeque::new();
    let mut stack = VecDeque::new();

    for op in r.root {
        println!("> {:?}  \t  {:>3?}  \t  {:?} ", op, acc, stack);

        match op {
            Expr::Plus => stack.push_back(Expr::Plus),
            Expr::Mult => stack.push_back(Expr::Mult),
            Expr::Open => stack.push_back(Expr::Open),
            Expr::Close => {
                let x = stack.pop_back();
                let y = acc.pop_back().unwrap();
                let last = if acc.len() > 0 { acc.len() - 1 } else { 0 };

                match x {
                    None => acc.push_back(y),
                    Some(Expr::Plus) => acc[last] += y,
                    Some(Expr::Mult) => acc[last] *= y,
                    Some(Expr::Open) => {
                        acc.push_back(y);
                    }
                    _ => unreachable!(),
                }
            }
            Expr::Val(v) => {
                let x = stack.pop_back();
                let last = if acc.len() > 0 { acc.len() - 1 } else { 0 };

                match x {
                    None => acc.push_back(v),
                    Some(Expr::Plus) => acc[last] += v,
                    Some(Expr::Mult) => acc[last] *= v,
                    Some(Expr::Open) => acc.push_back(v),
                    _ => unreachable!(),
                }
            }
        };
        println!("= {:?}  \t  {:>3?}  \t  {:?} ", op, acc, stack);
    }

    assert_eq!(acc.len(), 1);

    let solution = acc.pop_back().unwrap();
    println!("> solution = {}", solution);

    solution
}

fn main() -> Result<()> {
    let expr = input::input(File::open("data/test2.txt")?, transform)?;

    let mut acc = 0;
    for r in expr {
        println!("> {:?}", r);
        acc += execute(r);

        println!("> acc = {}", acc);
    }

    Ok(())
}
