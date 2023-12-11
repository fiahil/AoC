use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_start(input: &String) -> (isize, isize) {
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                return (i as isize, j as isize);
            }
        }
    }
    panic!("No start found");
}

fn is_connected(
    map: &HashMap<(isize, isize), char>,
    start: &(isize, isize),
    direction: Direction,
) -> bool {
    match direction {
        Direction::Up => {
            println!(
                "> {:?} {:?}",
                map.get(start),
                map.get(&(start.0 - 1, start.1))
            );
            match (map.get(start), map.get(&(start.0 - 1, start.1))) {
                (
                    Some('S') | Some('|') | Some('J') | Some('L'),
                    Some('|') | Some('F') | Some('7') | Some('S'),
                ) => true,
                (_, _) => false,
            }
        }
        Direction::Down => {
            println!(
                "> {:?} {:?}",
                map.get(start),
                map.get(&(start.0 + 1, start.1))
            );
            match (map.get(start), map.get(&(start.0 + 1, start.1))) {
                (
                    Some('S') | Some('|') | Some('F') | Some('7'),
                    Some('|') | Some('J') | Some('L') | Some('S'),
                ) => true,
                (_, _) => false,
            }
        }
        Direction::Left => {
            println!(
                "> {:?} {:?}",
                map.get(start),
                map.get(&(start.0, start.1 - 1))
            );
            match (map.get(start), map.get(&(start.0, start.1 - 1))) {
                (
                    Some('S') | Some('-') | Some('7') | Some('J'),
                    Some('-') | Some('F') | Some('L') | Some('S'),
                ) => true,
                (_, _) => false,
            }
        }
        Direction::Right => {
            println!(
                "> {:?} {:?}",
                map.get(start),
                map.get(&(start.0, start.1 + 1))
            );
            match (map.get(start), map.get(&(start.0, start.1 + 1))) {
                (
                    Some('S') | Some('-') | Some('F') | Some('L'),
                    Some('-') | Some('7') | Some('J') | Some('S'),
                ) => true,
                (_, _) => false,
            }
        }
    }
}

pub fn part1(input: &String) -> u32 {
    let mut start = find_start(input);
    let mut map = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                map.insert((i as isize, j as isize), c);
            }
        }
    }

    println!("> Start: {:?}", start);
    let original_start = start.clone();
    let mut scored = HashMap::new();
    let mut i = 0;
    scored.insert(start.clone(), i);
    while i == 0 || start != original_start {
        // check the 4 directions
        let mut found = false;
        for direction in vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            println!("> {:?} Checking {:?}...", start, direction);
            if is_connected(&map, &start, direction) {
                let new_start = match direction {
                    Direction::Up => (start.0 - 1, start.1),
                    Direction::Down => (start.0 + 1, start.1),
                    Direction::Left => (start.0, start.1 - 1),
                    Direction::Right => (start.0, start.1 + 1),
                };

                if !scored.contains_key(&new_start) || (new_start == original_start && i > 2) {
                    i += 1;
                    start = new_start;
                    println!("> {:?} = {}", start, i);
                    scored.insert(start.clone(), i);
                    found = true;
                    break;
                }
            }
        }
        if !found {
            panic!("No direction found");
        }
    }

    (scored.into_values().max().unwrap() + 1) / 2
}

pub fn part2(input: &String) -> u32 {
    0
}

pub mod test {
    #[test]
    pub fn part11() {
        let r = super::part1(
            &"
.....
.S-7.
.|.|.
.L-J.
.....
"
            .to_string(),
        );
        assert_eq!(r, 4);
    }

    #[test]
    pub fn part12() {
        let r = super::part1(
            &"
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"
            .to_string(),
        );
        assert_eq!(r, 8);
    }

    #[test]
    pub fn part2() {
        let r = super::part2(
            &"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
        "
            .to_string(),
        );
        assert_eq!(r, 10);
    }
}
