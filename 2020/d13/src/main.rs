mod input;

use std::fs::File;

use anyhow::{Context, Result};
use modinverse::modinverse;

#[derive(Debug)]
enum Bus {
    Unconstrained,
    Constrained(i64),
}

#[derive(Debug)]
struct SpeedBus {
    period: i64,
    to_add: i64,
}

fn transform(p: String) -> Result<(i64, Vec<Bus>)> {
    let mut lines = p.split("\n");
    let timestamp = lines.next().context("Missing timestamp")?.parse()?;
    let buses = lines
        .next()
        .context("Missing buses")?
        .split(",")
        // .inspect(|e| println!("> {:?}", e))
        .map(|e| {
            if e == "x" {
                Bus::Unconstrained
            } else {
                Bus::Constrained(e.parse().unwrap())
            }
        })
        .collect();

    Ok((timestamp, buses))
}

fn run(buses: &Vec<Bus>) -> i64 {
    println!("> Solving for bus list: ({}) {:?}", buses.len(), buses);

    let buses = buses
        .iter()
        .enumerate()
        .filter_map(|(i, b)| match b {
            Bus::Unconstrained => None,
            Bus::Constrained(p) => Some(SpeedBus {
                period: *p,
                to_add: i as i64,
            }),
        })
        .collect::<Vec<SpeedBus>>();

    println!("> Converted bus list to: ({}) {:?}", buses.len(), buses);

    let product = buses
        .iter()
        .map(|schedule| schedule.period)
        .product::<i64>();

    let solution = buses
        .iter()
        .map(|bus| {
            let x = product / bus.period;
            let modular_inverse = modinverse(x, bus.period).unwrap();
            (bus.period - bus.to_add % bus.period) * x * modular_inverse
        })
        .sum::<i64>()
        % product;

    println!("> Found solution ! {}", solution);
    solution
}

fn main() -> Result<()> {
    let (_, buses) = input::input(File::open("data/test.txt")?, transform)?;
    assert_eq!(run(&buses), 1068781);

    let (_, buses) = input::input(File::open("data/input.txt")?, transform)?;
    run(&buses);

    assert_eq!(
        run(&vec![
            Bus::Constrained(17),
            Bus::Unconstrained,
            Bus::Constrained(13),
            Bus::Constrained(19)
        ]),
        3417
    );
    assert_eq!(
        run(&vec![
            Bus::Constrained(67),
            Bus::Constrained(7),
            Bus::Constrained(59),
            Bus::Constrained(61)
        ]),
        754018
    );
    assert_eq!(
        run(&vec![
            Bus::Constrained(67),
            Bus::Unconstrained,
            Bus::Constrained(7),
            Bus::Constrained(59),
            Bus::Constrained(61)
        ]),
        779210
    );
    assert_eq!(
        run(&vec![
            Bus::Constrained(67),
            Bus::Constrained(7),
            Bus::Unconstrained,
            Bus::Constrained(59),
            Bus::Constrained(61)
        ]),
        1261476
    );
    assert_eq!(
        run(&vec![
            Bus::Constrained(1789),
            Bus::Constrained(37),
            Bus::Constrained(47),
            Bus::Constrained(1889)
        ]),
        1202161486
    );

    Ok(())
}
