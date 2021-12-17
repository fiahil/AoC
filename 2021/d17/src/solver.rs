use std::{
    cmp::max,
    collections::HashSet,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use rand::prelude::*;

#[derive(Debug)]
struct Target {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Velocity {
    dx: i32,
    dy: i32,
}

#[derive(Debug, Clone)]
struct Probe {
    pos: Point,
    vel: Velocity,
    original_vel: Velocity,
    max_height: i32,
}

#[derive(Debug, Clone, Copy)]
enum Intersection {
    Ok,
    Maybe,
    No,
}

impl TryFrom<&String> for Target {
    type Error = anyhow::Error;

    fn try_from(s: &String) -> Result<Self> {
        let (_, range) = s.trim().split_once(": ").with_context(|| "Bad format 1")?;
        let (x, y) = range.split_once(", ").with_context(|| "Bad format 2")?;
        let (xmin, xmax) = x[2..].split_once("..").with_context(|| "Bad format 3")?;
        let (ymin, ymax) = y[2..].split_once("..").with_context(|| "Bad format 4")?;

        Ok(Target {
            xmin: xmin.parse::<i32>().with_context(|| "Bad number xmin")?,
            xmax: xmax.parse::<i32>().with_context(|| "Bad number xmax")?,
            ymin: ymin.parse::<i32>().with_context(|| "Bad number ymin")?,
            ymax: ymax.parse::<i32>().with_context(|| "Bad number ymax")?,
        })
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Velocity {
    fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }
}

impl Probe {
    fn new(pos: Point, vel: Velocity) -> Self {
        Self {
            original_vel: vel.clone(),
            max_height: 0,
            pos,
            vel,
        }
    }

    fn step(&mut self) {
        self.pos.x += self.vel.dx;
        self.pos.y += self.vel.dy;

        self.max_height = max(self.max_height, self.pos.y);

        self.vel.dx = max(0, self.vel.dx - 1);
        self.vel.dy = self.vel.dy - 1;
    }
}

impl Intersection {
    fn is_ok(&self) -> bool {
        match self {
            Self::Ok => true,
            _ => false,
        }
    }
}

impl Target {
    fn intersect(&self, point: &Point) -> Intersection {
        if point.x > self.xmax || point.y < self.ymin {
            Intersection::No
        } else if point.x >= self.xmin
            && point.y >= self.ymin
            && point.x <= self.xmax
            && point.y <= self.ymax
        {
            Intersection::Ok
        } else {
            Intersection::Maybe
        }
    }
}

fn syscheck(target: &Target, point: &Point) {
    println!(
        "> Checking system      : {:<4?} | {:?}",
        point,
        target.intersect(&point)
    );
}

fn gen(saved_probe: &Option<Probe>, max_range: (i32, i32, i32), pool_size: usize) -> Vec<Velocity> {
    let mut v = Vec::new();

    if saved_probe.is_some() {
        let probe = saved_probe.as_ref().unwrap();

        v.push(Velocity::new(probe.original_vel.dx, probe.original_vel.dy));
        v.push(Velocity::new(
            probe.original_vel.dx + 1,
            probe.original_vel.dy + 1,
        ));
        v.push(Velocity::new(
            probe.original_vel.dx - 1,
            probe.original_vel.dy + 1,
        ));
        v.push(Velocity::new(
            probe.original_vel.dx - 1,
            probe.original_vel.dy - 1,
        ));
        v.push(Velocity::new(
            probe.original_vel.dx + 1,
            probe.original_vel.dy - 1,
        ));
    }

    for _ in 0..pool_size - 5 {
        let mut rng = thread_rng();

        v.push(Velocity::new(
            rng.gen_range(0..max_range.0),
            rng.gen_range(max_range.1..max_range.2),
        ));
    }

    v
}

pub fn part1(input: &String) -> Result<i32> {
    let target = Target::try_from(input)?;

    const MAX_ITERATIONS: usize = 100;

    println!("> Registered target    : {:?}", target);
    syscheck(&target, &Point::new(0, 0));
    syscheck(&target, &Point::new(25, -8));
    syscheck(&target, &Point::new(100, -100));

    let mut max_height = i32::MIN;
    let mut saved_probe = None;
    for i in 0..MAX_ITERATIONS {
        let mut probes = gen(
            &saved_probe,
            (target.xmax, target.ymin, -target.ymin * 100),
            100,
        )
        .into_iter()
        .map(|v| Probe::new(Point::new(0, 0), v))
        .collect::<Vec<_>>();

        println!("> Iteration            : {}", i);
        let mut step = 0;
        loop {
            probes = probes
                .into_iter()
                .filter_map(|mut p| match target.intersect(&p.pos) {
                    Intersection::Ok => Some(p),
                    Intersection::Maybe => {
                        p.step();
                        match target.intersect(&p.pos) {
                            Intersection::Ok => Some(p),
                            Intersection::Maybe => Some(p),
                            Intersection::No => None,
                        }
                    }
                    _ => unreachable!("only keep valid probes after each iteration"),
                })
                .collect::<Vec<_>>();

            if probes.len() == 0 {
                println!("                       : No probes left!");
                break;
            }

            if probes.iter().all(|p| target.intersect(&p.pos).is_ok()) {
                println!("                       : All probes found!");
                break;
            }

            step += 1;
        }
        println!("                       : {} / {} steps", i, step);

        if let Some(new_max) = probes
            .into_iter()
            .max_by(|a, b| a.max_height.cmp(&b.max_height))
        {
            if new_max.max_height > max_height {
                println!("> New max height       : {}m", new_max.max_height);
                println!(">> Probe               : {:?}", new_max);
                max_height = new_max.max_height;
                saved_probe = Some(new_max);
            }
        }

        println!("");
    }

    println!("> Record max height    : {}m", max_height);

    Ok(max_height)
}

fn gen2(max_range: (i32, i32, i32)) -> Vec<Velocity> {
    let mut v = Vec::new();

    for dx in 0..max_range.0 {
        for dy in max_range.1..max_range.2 {
            v.push(Velocity::new(dx, dy));
        }
    }

    v
}

pub fn part2(input: &String) -> Result<usize> {
    let target = Target::try_from(input)?;

    println!("> Registered target    : {:?}", target);
    syscheck(&target, &Point::new(0, 0));
    syscheck(&target, &Point::new(25, -8));
    syscheck(&target, &Point::new(100, -100));

    let mut velocity_set = HashSet::new();
    let mut probes = gen2((target.xmax + 1, target.ymin - 1, -target.ymin + 1))
        .into_iter()
        .map(|v| Probe::new(Point::new(0, 0), v))
        .collect::<Vec<_>>();

    println!("> Registered probes    : {}", probes.len());
    let mut step = 0;
    let delta = Duration::from_secs(5);
    let mut start = Instant::now();
    loop {
        probes = probes
            .into_iter()
            .filter_map(|mut p| match target.intersect(&p.pos) {
                Intersection::Ok => Some(p),
                Intersection::Maybe => {
                    p.step();
                    match target.intersect(&p.pos) {
                        Intersection::Ok => Some(p),
                        Intersection::Maybe => Some(p),
                        Intersection::No => None,
                    }
                }
                _ => unreachable!("only keep valid probes after each iteration"),
            })
            .collect::<Vec<_>>();

        if probes.len() == 0 {
            println!("                       : No probes left!");
            break;
        }

        if probes.iter().all(|p| target.intersect(&p.pos).is_ok()) {
            println!("                       : All probes found!");
            break;
        }

        step += 1;

        if start.elapsed() > delta {
            println!("> Step                 : {}", step);
            start = Instant::now();
        }
    }

    for probe in probes {
        velocity_set.insert(probe.original_vel);
    }

    println!("> Found {} valid velocities", velocity_set.len());

    Ok(velocity_set.len())
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 45);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 112);

        Ok(())
    }
}
