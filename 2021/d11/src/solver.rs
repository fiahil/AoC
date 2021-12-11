use std::collections::HashSet;

use anyhow::Result;

fn parse(input: &String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|i| i.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn flash(
    grid: &mut Vec<Vec<u32>>,
    y: usize,
    x: usize,
    flash_set: &mut HashSet<(usize, usize)>,
) {
    if flash_set.contains(&(y, x)) {
        return;
    }

    if grid[y][x] <= 9 {
        return;
    }

    flash_set.insert((y, x));

    let neighbors: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dy, dx) in neighbors {
        if y as isize + dy < grid.len() as isize
            && y as isize + dy >= 0
            && x as isize + dx < grid[y].len() as isize
            && x as isize + dx >= 0
        {
            grid[(y as isize + dy) as usize][(x as isize + dx) as usize] += 1;
            flash(
                grid,
                (y as isize + dy) as usize,
                (x as isize + dx) as usize,
                flash_set,
            );
        }
    }
}

fn display_grid(grid: &Vec<Vec<u32>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn part1(input: &String) -> Result<usize> {
    let mut grid = parse(input);
    let mut sum_flashes = 0;

    for steps in 0..100 {
        // Increase all octopuses
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                grid[y][x] += 1;
            }
        }

        // All Octopuses that are larger than 9 flash
        let mut flash_set = HashSet::new();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                flash(&mut grid, y, x, &mut flash_set);
            }
        }

        // All flashing octopuses are set to 0
        sum_flashes += flash_set.len();
        println!(
            "step {:>3}: {} flashes ({} total)",
            steps + 1,
            flash_set.len(),
            sum_flashes
        );
        for (y, x) in flash_set {
            grid[y][x] = 0;
        }

        // display_grid(&grid);
    }

    Ok(sum_flashes)
}

pub fn part2(input: &String) -> Result<usize> {
    let mut grid = parse(input);
    let grid_size = grid.len() * grid[0].len();
    let mut steps = 1;

    loop {
        // Increase all octopuses
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                grid[y][x] += 1;
            }
        }

        // All Octopuses that are larger than 9 flash
        let mut flash_set = HashSet::new();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                flash(&mut grid, y, x, &mut flash_set);
            }
        }

        // All flashing octopuses are set to 0
        let flash_count = flash_set.len();
        println!("step {:>3}: {} flashes", steps, flash_count);
        for (y, x) in flash_set {
            grid[y][x] = 0;
        }

        if flash_count == grid_size {
            break;
        }

        steps += 1;
    }

    println!("steps: {}", steps);
    display_grid(&grid);

    Ok(steps)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 1656);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 195);

        Ok(())
    }
}
