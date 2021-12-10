use std::collections::HashSet;

use anyhow::Result;

fn parse(input: &String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|i| i.to_digit(10).unwrap()).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: u32,
}

fn find_low_points(map: &Vec<Vec<u32>>, y: usize, x: usize, low_points: &mut HashSet<Point>) {
    let z = map[y][x];

    let neighbors = vec![
        map.get(y).and_then(|row| row.get(x + 1)).cloned(),
        map.get(y)
            .and_then(|row| if x > 0 { row.get(x - 1) } else { None })
            .cloned(),
        if y > 0 {
            map.get(y - 1).and_then(|row| row.get(x)).cloned()
        } else {
            None
        },
        map.get(y + 1).and_then(|row| row.get(x)).cloned(),
    ];

    if neighbors.iter().all(|n| n.is_none() || n.unwrap() > z) {
        low_points.insert(Point { x, y, z });
    }
}

pub fn part1(input: &String) -> Result<u32> {
    let map = parse(input);

    let mut low_points = HashSet::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            find_low_points(&map, y, x, &mut low_points);
        }
    }

    println!("found these low points: {:?}", low_points);

    let sum = low_points.iter().map(|p| p.z + 1).sum();

    println!("sum: {}", sum);

    Ok(sum)
}

fn find_basins(map: &Vec<Vec<u32>>, y: usize, x: usize, basin: &mut HashSet<Point>) {
    let z = map.get(y).and_then(|r| r.get(x)).cloned();

    if let Some(z) = z {
        let point = Point { x, y, z };

        if z < 9 && basin.get(&point).is_none() {
            basin.insert(point);

            if x > 0 {
                find_basins(map, y, x - 1, basin);
            }

            if y > 0 {
                find_basins(map, y - 1, x, basin);
            }

            find_basins(map, y, x + 1, basin);
            find_basins(map, y + 1, x, basin);
        }
    }
}

pub fn part2(input: &String) -> Result<u32> {
    let map = parse(input);

    let mut basins: Vec<HashSet<Point>> = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let mut basin = HashSet::new();

            find_basins(&map, y, x, &mut basin);

            if !basins.contains(&basin) {
                basins.push(basin);
            }
        }
    }

    println!("found these basins: {:?}", basins);

    let mut product = basins.iter().map(|b| b.len() as u32).collect::<Vec<_>>();
    product.sort();
    product.reverse();
    let product = product.iter().take(3).product();

    println!("product: {}", product);

    Ok(product)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 15);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 1134);

        Ok(())
    }
}
