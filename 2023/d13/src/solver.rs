/// Find a vertical symetry in the given pattern
fn find_vsym(pattern: &str) -> (usize, usize) {
    let lines = pattern.trim().split("\n").collect::<Vec<&str>>();

    let mut columns = Vec::new();
    for i in 0..lines[0].len() {
        let col = lines
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .collect::<String>();
        columns.push(col);
    }

    // find the 2 identical columns in the list
    for i in 1..(columns.len() / 2 + 1) {
        let mut icols = columns
            .iter()
            .enumerate()
            .take_while(|(x, _)| *x < i)
            .map(|(_, s)| s)
            .collect::<Vec<&String>>();

        icols.reverse();

        let jcols = columns
            .iter()
            .enumerate()
            .skip_while(|(x, _)| *x < i)
            .map(|(_, s)| s)
            .collect::<Vec<&String>>();

        // println!("> icols {:?}", icols);
        // println!("> jcols {:?}", jcols);
        // println!("---");

        if icols.iter().zip(jcols.iter()).all(|(a, b)| a == b) {
            println!("> sym line {} {}", i, i + 1);
            return (i, i + 1);
        }
    }

    (0, 0)
}

/// Find an horizontal symetry in the given pattern
fn find_hsym(pattern: &str) -> (usize, usize) {
    let lines = pattern.trim().split("\n").collect::<Vec<&str>>();

    for i in 1..(lines.len() / 2 + 1) {
        let mut icols = lines
            .iter()
            .enumerate()
            .take_while(|(x, _)| *x < i)
            .map(|(_, s)| s)
            .collect::<Vec<&&str>>();

        icols.reverse();

        let jcols = lines
            .iter()
            .enumerate()
            .skip_while(|(x, _)| *x < i)
            .map(|(_, s)| s)
            .collect::<Vec<&&str>>();

        // println!("> icols {:?}", icols);
        // println!("> jcols {:?}", jcols);
        // println!("---");

        if icols.iter().zip(jcols.iter()).all(|(a, b)| a == b) {
            println!("> sym line {} {}", i, i + 1);
            return (i, i + 1);
        }
    }

    (0, 0)
}

pub fn part1(input: &String) -> u32 {
    let patterns = input.split("\n\n").collect::<Vec<&str>>();

    let mut left_of = 0;
    let mut above_of = 0;
    for pattern in patterns {
        println!("{}", pattern);
        let (a, _) = find_vsym(pattern);
        let (c, _) = find_hsym(pattern);
        left_of += a;
        above_of += c;
    }

    left_of as u32 + (above_of as u32 * 100)
}

pub fn part2(input: &String) -> u32 {
    let patterns = input.split("\n\n").collect::<Vec<&str>>();

    let mut left_of = 0;
    let mut above_of = 0;
    for pattern in patterns {
        println!("original\n{}", pattern);
        for idx in 0..pattern.len() {
            let mut new_pattern = String::from(pattern).chars().collect::<Vec<char>>();
            match new_pattern[idx] {
                '#' => new_pattern[idx] = '.',
                '.' => new_pattern[idx] = '#',
                _ => continue,
            }

            println!(
                "\n---\nnew pattern\n{}",
                new_pattern.iter().collect::<String>()
            );

            let (a, _) = find_vsym(new_pattern.iter().collect::<String>().as_str());
            let (b, _) = find_vsym(pattern);
            println!("v original : {} | new : {}", b, a);
            if a > 0 && idx < a {
                left_of += a;
            }
            let (c, _) = find_hsym(new_pattern.iter().collect::<String>().as_str());
            let (d, _) = find_hsym(pattern);
            println!("h original : {} | new : {}", d, c);
            if c > 0 && idx < c {
                above_of += c;
            }
        }
    }

    left_of as u32 + (above_of as u32 * 100)
}

pub mod test {
    #[test]
    pub fn part1() {
        let r = super::part1(
            &"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"
            .to_string(),
        );
        assert_eq!(r, 405);
    }

    #[test]
    pub fn part2() {
        let r = super::part2(
            &"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"
            .to_string(),
        );
        assert_eq!(r, 400);
    }
}
