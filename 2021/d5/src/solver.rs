use anyhow::Result;
use std::{collections::HashSet, fmt};

#[derive(Debug)]
pub struct Segment {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}) -> ({}, {})",
            self.x0, self.y0, self.x1, self.y1
        )
    }
}

impl TryFrom<&str> for Segment {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self> {
        let mut iter = s.split(" -> ");
        let mut first = iter
            .next()
            .ok_or(anyhow::anyhow!("Missing segment 1"))?
            .split(",");
        let mut second = iter
            .next()
            .ok_or(anyhow::anyhow!("Missing segment 2"))?
            .split(",");

        let x0 = first.next().ok_or(anyhow::anyhow!("Missing x0"))?.parse()?;
        let y0 = first.next().ok_or(anyhow::anyhow!("Missing y0"))?.parse()?;

        let x1 = second
            .next()
            .ok_or(anyhow::anyhow!("Missing x1"))?
            .parse()?;
        let y1 = second
            .next()
            .ok_or(anyhow::anyhow!("Missing y1"))?
            .parse()?;

        Ok(Segment { x0, y0, x1, y1 })
    }
}

impl Segment {
    pub fn points(&self) -> Vec<(i32, i32)> {
        let mut points = Vec::new();
        let mut x = self.x0;
        let mut y = self.y0;

        loop {
            points.push((x, y));

            if x == self.x1 && y == self.y1 {
                break;
            }

            if x != self.x1 && y != self.y1 {
                if x < self.x1 {
                    x += 1;
                } else {
                    x -= 1;
                }

                if y < self.y1 {
                    y += 1;
                } else {
                    y -= 1;
                }
            } else {
                if x != self.x1 {
                    if self.x0 < self.x1 {
                        x += 1;
                    } else {
                        x -= 1;
                    }
                } else {
                    if self.y0 < self.y1 {
                        y += 1;
                    } else {
                        y -= 1;
                    }
                }
            }
        }

        // println!("Segment: {}", self);
        // println!("segment cover these points: {:?}", points);

        points
    }

    pub fn intersect(&self, other: &Segment) -> HashSet<(i32, i32)> {
        if self.x0 == other.x0 && self.y0 == other.y0 && self.x1 == other.x1 && self.y1 == other.y1
        {
            return HashSet::new();
        }

        // println!("calculating intersection for {} | {}", self, other);

        let self_points = self.points();
        let other_points = other.points();

        let mut count = HashSet::new();
        for point in self_points.iter() {
            for other_point in other_points.iter() {
                if point == other_point {
                    count.insert(*point);
                }
            }
        }

        count
    }

    pub fn intersect_no_diagonal(&self, other: &Segment) -> HashSet<(i32, i32)> {
        if !(self.x0 == self.x1 || self.y0 == self.y1)
            || !(other.x0 == other.x1 || other.y0 == other.y1)
        {
            // println!("One of the segments is diagonal");
            return HashSet::new();
        }

        self.intersect(other)
    }
}

pub fn part1(input: &String) -> Result<i32> {
    let segments = input
        .lines()
        .map(|s| Segment::try_from(s))
        .collect::<Result<Vec<_>>>()?;

    let mut intersections = HashSet::new();
    for (i, segment) in segments.iter().enumerate() {
        for other in segments.iter() {
            let points = segment.intersect_no_diagonal(other);

            if points.len() > 0 {
                println!("found theses intersecting points: {:?}", points,);
                intersections.extend(points);
                println!("intersections count so far: {}", intersections.len());
                println!(
                    "progress: {:.2}%",
                    (i + 1) as f32 / segments.len() as f32 * 100.0
                );
            }
        }
    }

    println!("Solution: counted {} intersections", intersections.len());

    Ok(intersections.len() as i32)
}

pub fn part2(input: &String) -> Result<i32> {
    let segments = input
        .lines()
        .map(|s| Segment::try_from(s))
        .collect::<Result<Vec<_>>>()?;

    let mut intersections = HashSet::new();
    for (i, segment) in segments.iter().enumerate() {
        for other in segments.iter() {
            let points = segment.intersect(other);

            if points.len() > 0 {
                println!("found theses intersecting points: {:?}", points,);
                intersections.extend(points);
                println!("intersections count so far: {}", intersections.len());
                println!(
                    "progress: {:.2}%",
                    (i + 1) as f32 / segments.len() as f32 * 100.0
                );
            }
        }
    }

    println!("Solution: counted {} intersections", intersections.len());

    Ok(intersections.len() as i32)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 5);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 12);

        Ok(())
    }
}
