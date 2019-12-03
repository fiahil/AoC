#[macro_use]
extern crate log;

use simplelog::{Config, LevelFilter, SimpleLogger};
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    steps: usize,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Eq for Point {}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cross {
    x: i32,
    y: i32,
    distance: usize,
    steps: usize,
}

#[derive(Debug)]
struct Wires {
    board: Vec<HashSet<Point>>,
}

impl Wires {
    fn new() -> Self {
        Wires { board: Vec::new() }
    }

    fn add(&mut self, wire: &str) -> &mut Self {
        fn code_to_points(code: &str) -> Vec<Point> {
            let (d, tail) = code.split_at(1);
            let count: usize = tail.parse().unwrap();
            debug!("Parsed {} | {}", d, count);

            match d {
                "R" => (0..count)
                    .map(|_| Point {
                        x: 1,
                        y: 0,
                        steps: 1,
                    })
                    .collect(),
                "L" => (0..count)
                    .map(|_| Point {
                        x: -1,
                        y: 0,
                        steps: 1,
                    })
                    .collect(),
                "U" => (0..count)
                    .map(|_| Point {
                        x: 0,
                        y: 1,
                        steps: 1,
                    })
                    .collect(),
                "D" => (0..count)
                    .map(|_| Point {
                        x: 0,
                        y: -1,
                        steps: 1,
                    })
                    .collect(),
                x => panic!("Unknown char `{}`", x),
            }
        }

        debug!("Parsing {}", wire);

        let mut set = HashSet::new();
        let codes: Vec<&str> = wire.split(',').collect();
        codes.iter().map(|x| code_to_points(*x)).flatten().fold(
            Point {
                x: 0,
                y: 0,
                steps: 0,
            },
            |last, current| {
                let p = Point {
                    x: last.x + current.x,
                    y: last.y + current.y,
                    steps: last.steps + current.steps,
                };
                set.insert(p.clone());
                p
            },
        );

        debug!("Transformed to {:?}", set);

        self.board.push(set);

        info!("Wires in board: {}", self.board.len());

        self
    }

    fn instersects(&self) -> Vec<Cross> {
        fn points_to_cross(p: &Point, other: &Point) -> Cross {
            assert!(p.x == other.x && p.y == other.y);
            Cross {
                x: p.x,
                y: p.y,
                distance: (p.x.abs() + p.y.abs()) as usize,
                steps: p.steps + other.steps,
            }
        }

        let intersections = self.board[0].intersection(&self.board[1]);
        let mut crosses = Vec::new();

        for e in intersections {
            let p1 = self.board[0].get(e).unwrap();
            let p2 = self.board[1].get(e).unwrap();

            crosses.push(points_to_cross(p1, p2));
        }
        crosses
    }

    fn by_distance(&self, idx: usize) -> Cross {
        let mut intersects = self.instersects();

        intersects.sort_by(|a, b| a.distance.cmp(&b.distance));

        info!("Gathered intersections: {:?}", intersects);

        intersects[idx].clone()
    }

    fn by_steps(&self, idx: usize) -> Cross {
        let mut intersects = self.instersects();

        intersects.sort_by(|a, b| a.steps.cmp(&b.steps));

        info!("Gathered intersections: {:?}", intersects);

        intersects[idx].clone()
    }
}

fn main() {
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
    let f = fs::read_to_string("input.txt").unwrap();

    let mut ws = Wires::new();

    f.split_terminator('\n').for_each(|e| {
        ws.add(e);
    });

    println!("Found best distance cross: {:?}", ws.by_distance(0));
    println!("Found best steps cross: {:?}", ws.by_steps(0));
}

mod test {
    use super::*;

    #[test]
    fn test_step_0() {
        SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
        assert_eq!(
            Wires::new()
                .add("R75,D30,R83,U83,L12,D49,R71,U7,L72")
                .add("U62,R66,U55,R34,D71,R55,D58,R83")
                .by_steps(0)
                .steps,
            610
        )
    }

    #[test]
    fn test_step_1() {
        assert_eq!(
            Wires::new()
                .add("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")
                .add("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
                .by_steps(0)
                .steps,
            410
        )
    }

    #[test]
    fn test_0() {
        SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
        assert_eq!(
            Wires::new()
                .add("R75,D30,R83,U83,L12,D49,R71,U7,L72")
                .add("U62,R66,U55,R34,D71,R55,D58,R83")
                .by_distance(0)
                .distance,
            159
        )
    }

    #[test]
    fn test_1() {
        assert_eq!(
            Wires::new()
                .add("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")
                .add("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
                .by_distance(0)
                .distance,
            135
        )
    }
}
