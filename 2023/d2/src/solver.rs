use anyhow::Result;

#[derive(Debug)]
struct Games {
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    green_cubes: usize,
    blue_cubes: usize,
    red_cubes: usize,
}

impl From<&str> for Games {
    fn from(input: &str) -> Self {
        let games = input.lines().map(|line| Game::from(line)).collect();

        Games { games }
    }
}

impl From<&str> for Game {
    fn from(input: &str) -> Self {
        let mut draws = Vec::new();

        let id = input
            .chars()
            .skip(5)
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        for draw in input.split_once(":").unwrap().1.split(';') {
            draws.push(Draw::from(draw));
        }

        Game { id, draws }
    }
}

impl From<&str> for Draw {
    fn from(input: &str) -> Self {
        let mut green_cubes = 0;
        let mut blue_cubes = 0;
        let mut red_cubes = 0;

        for cube in input.split(',') {
            let cube = cube.trim();

            if cube.ends_with("green") {
                green_cubes = cube
                    .chars()
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
            } else if cube.ends_with("blue") {
                blue_cubes = cube
                    .chars()
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
            } else if cube.ends_with("red") {
                red_cubes = cube
                    .chars()
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
            }
        }

        Draw {
            green_cubes,
            blue_cubes,
            red_cubes,
        }
    }
}

pub fn part1(input: &String) -> Result<i32> {
    let games = Games::from(input.as_str());

    // Find which games are possible if we only have 12 red cubes, 13 green cubes, and 14 blue cubes

    let result: usize = games
        .games
        .iter()
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| draw.red_cubes <= 12 && draw.green_cubes <= 13 && draw.blue_cubes <= 14)
        })
        .map(|game| dbg!(game.id))
        .sum();

    Ok(result as i32)
}

pub fn part2(input: &String) -> Result<i32> {
    let games = Games::from(input.as_str());

    // Find the minimum number of cubes we need to add to make each games possible

    let result: usize = games
        .games
        .iter()
        .map(|game| {
            let mut red_cubes = 0;
            let mut green_cubes = 0;
            let mut blue_cubes = 0;

            for draw in &game.draws {
                if draw.red_cubes > red_cubes {
                    red_cubes = draw.red_cubes;
                }
                if draw.green_cubes > green_cubes {
                    green_cubes = draw.green_cubes;
                }
                if draw.blue_cubes > blue_cubes {
                    blue_cubes = draw.blue_cubes;
                }
            }

            dbg!(red_cubes * green_cubes * blue_cubes)
        })
        .sum();

    Ok(result as i32)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 8);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 2286);

        Ok(())
    }
}
