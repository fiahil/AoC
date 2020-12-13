mod input;

use std::fs::File;
use std::str::FromStr;

use anyhow::{Error, Result};

fn transform(p: String) -> Result<Vec<Nav>> {
    p.lines()
        .inspect(|e| println!("> {:?}", e))
        .map(|e| Nav::from_str(e))
        .collect()
}

#[derive(Debug, Clone)]
enum Nav {
    N(u32),
    S(u32),
    E(u32),
    W(u32),
    L(u32),
    R(u32),
    F(u32),
}

impl FromStr for Nav {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nav = match s.split_at(1) {
            ("N", v) => Nav::N(v.parse()?),
            ("S", v) => Nav::S(v.parse()?),
            ("E", v) => Nav::E(v.parse()?),
            ("W", v) => Nav::W(v.parse()?),
            ("L", v) => Nav::L(v.parse()?),
            ("R", v) => Nav::R(v.parse()?),
            ("F", v) => Nav::F(v.parse()?),
            _ => unreachable!(),
        };

        Ok(nav)
    }
}

#[derive(Debug)]
struct Waypoint {
    we_pos: i32,
    ns_pos: i32,
}

impl Waypoint {
    fn rotate_left(mut self) -> Self {
        let ns = self.ns_pos;
        let we = self.we_pos;

        self.ns_pos = -we;
        self.we_pos = ns;

        self
    }
    fn rotate_right(mut self) -> Self {
        let ns = self.ns_pos;
        let we = self.we_pos;

        self.ns_pos = we;
        self.we_pos = -ns;

        self
    }
}

#[derive(Debug)]
struct Boat {
    waypoint: Waypoint,
    we_pos: i32,
    ns_pos: i32,
}

impl Boat {
    fn new() -> Boat {
        Boat {
            waypoint: Waypoint {
                we_pos: -10,
                ns_pos: 1,
            },
            we_pos: 0,
            ns_pos: 0,
        }
    }

    fn manhattan(&self) -> u32 {
        self.ns_pos.abs() as u32 + self.we_pos.abs() as u32
    }

    fn forward(&mut self, v: i32) {
        self.ns_pos += self.waypoint.ns_pos * v;
        self.we_pos += self.waypoint.we_pos * v;
    }

    fn next(mut self, nav: &Nav) -> Self {
        match nav {
            Nav::N(v) => self.waypoint.ns_pos += *v as i32,
            Nav::S(v) => self.waypoint.ns_pos -= *v as i32,
            Nav::E(v) => self.waypoint.we_pos -= *v as i32,
            Nav::W(v) => self.waypoint.we_pos += *v as i32,
            Nav::L(v) => {
                self.waypoint = match v {
                    90 => self.waypoint.rotate_left(),
                    180 => self.waypoint.rotate_left().rotate_left(),
                    270 => self.waypoint.rotate_left().rotate_left().rotate_left(),
                    _ => unreachable!(),
                }
            }
            Nav::R(v) => {
                self.waypoint = match v {
                    90 => self.waypoint.rotate_right(),
                    180 => self.waypoint.rotate_right().rotate_right(),
                    270 => self.waypoint.rotate_right().rotate_right().rotate_right(),
                    _ => unreachable!(),
                }
            }
            Nav::F(v) => self.forward(*v as i32),
        };

        println!("# > {:?}", self);

        self
    }
}

fn main() -> Result<()> {
    let boat = input::input(File::open("data/input.txt")?, transform)?
        .iter()
        .inspect(|e| println!("> {:?}", e))
        .fold(Boat::new(), |boat, nav| boat.next(nav));

    println!("> {:?}", boat);
    println!("> distance: {:?}", boat.manhattan());

    Ok(())
}
