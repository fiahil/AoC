use indicatif::{ProgressIterator, ProgressStyle};

pub fn part1(input: &String) -> u32 {
    let line_length = input.trim().find("\n").unwrap() + 1;
    let height = input.trim().matches("\n").count() + 1;
    let mut input = input.trim().chars().collect::<Vec<char>>();

    println!("{}\n", input.iter().collect::<String>());

    // slide rocks up
    loop {
        let mut moved = false;
        for i in 0..input.len() {
            if input[i] == 'O' && i >= line_length && input[i - line_length] == '.' {
                input[i] = '.';
                input[i - line_length] = 'O';
                moved = true;
            }
        }

        if !moved {
            break;
        }
    }
    println!("{}\n", input.iter().collect::<String>());

    // count load
    input.into_iter().enumerate().fold(0, |acc, (i, c)| {
        if c == 'O' {
            acc + (height - (i / line_length)) as u32
        } else {
            acc
        }
    })
}

fn slide_up(input: &mut Vec<char>, line_length: usize) {
    // slide rocks up
    loop {
        let mut moved = false;
        for i in 0..input.len() {
            if input[i] == 'O' && i >= line_length && input[i - line_length] == '.' {
                input[i] = '.';
                input[i - line_length] = 'O';
                moved = true;
            }
        }

        if !moved {
            break;
        }
    }
}
fn slide_left(input: &mut Vec<char>, line_length: usize) {
    // slide rocks left
    loop {
        let mut moved = false;
        for i in 0..input.len() {
            if input[i] == 'O' && i % line_length != 0 && input[i - 1] == '.' {
                input[i] = '.';
                input[i - 1] = 'O';
                moved = true;
            }
        }

        if !moved {
            break;
        }
    }
}
fn slide_right(input: &mut Vec<char>, line_length: usize) {
    // slide rocks right
    loop {
        let mut moved = false;
        for i in 0..input.len() {
            if input[i] == 'O' && i % line_length != line_length - 2 && input[i + 1] == '.' {
                input[i] = '.';
                input[i + 1] = 'O';
                moved = true;
            }
        }

        if !moved {
            break;
        }
    }
}
fn slide_down(input: &mut Vec<char>, line_length: usize) {
    // slide rocks down
    loop {
        let mut moved = false;
        for i in 0..input.len() {
            if input[i] == 'O' && i < input.len() - line_length && input[i + line_length] == '.' {
                input[i] = '.';
                input[i + line_length] = 'O';
                moved = true;
            }
        }

        if !moved {
            break;
        }
    }
}

pub fn part2(input: &String) -> u32 {
    let line_length = input.trim().find("\n").unwrap() + 1;
    let height = input.trim().matches("\n").count() + 1;
    let mut input = input.trim().chars().collect::<Vec<char>>();

    println!("{}\n", input.iter().collect::<String>());

    for _ in (0..1_000).progress_with_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] ({pos:>11}/{len:11} ({percent:>3}%), ETA {eta_precise})",
        )
        .unwrap()
        .progress_chars("#>-"),
    ) {
        slide_up(&mut input, line_length);
        slide_left(&mut input, line_length);
        slide_down(&mut input, line_length);
        slide_right(&mut input, line_length);
    }
    println!("{}\n", input.iter().collect::<String>());

    // count load
    input.into_iter().enumerate().fold(0, |acc, (i, c)| {
        if c == 'O' {
            acc + (height - (i / line_length)) as u32
        } else {
            acc
        }
    })
}

pub mod test {
    #[test]
    pub fn part1() {
        let r = super::part1(
            &"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"
            .to_string(),
        );
        assert_eq!(r, 136);
    }

    #[test]
    pub fn test_slide_right() {
        let mut input = "..O..#O..O\n.......O..".chars().collect::<Vec<char>>();
        super::slide_right(&mut input, 11);
        super::slide_right(&mut input, 11);

        assert_eq!(input.iter().collect::<String>(), "....O#..OO\n.........O");
    }

    #[test]
    pub fn test_slide_down() {
        let mut input = "O.#..O.#.#\n..O..#O..O".chars().collect::<Vec<char>>();
        super::slide_down(&mut input, 11);
        super::slide_down(&mut input, 11);

        assert_eq!(input.iter().collect::<String>(), "..#..O.#.#\nO.O..#O..O");
    }

    #[test]
    pub fn part2() {
        let r = super::part2(
            &"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"
            .to_string(),
        );
        assert_eq!(r, 64);
    }
}
