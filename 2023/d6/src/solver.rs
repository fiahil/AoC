fn parse_input(input: &String) -> (Vec<u32>, Vec<u32>) {
    input
        .split_once("\n")
        .map(|(time, distance)| {
            (
                time.split_once(":")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .map(|n| n.trim().parse::<u32>().unwrap())
                    .collect(),
                distance
                    .split_once(":")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .map(|n| n.trim().parse::<u32>().unwrap())
                    .collect(),
            )
        })
        .unwrap()
}

fn parse_input2(input: &String) -> (usize, usize) {
    input
        .split_once("\n")
        .map(|(time, distance)| {
            (
                dbg!(time.split_once(":").unwrap().1.replace(" ", "").trim())
                    .parse::<usize>()
                    .unwrap(),
                dbg!(distance.split_once(":").unwrap().1.replace(" ", "").trim())
                    .parse::<usize>()
                    .unwrap(),
            )
        })
        .unwrap()
}

pub fn part1(input: &String) -> u32 {
    let (time, distance) = parse_input(input);

    let mut total = 1;
    for (t, d) in time.into_iter().zip(distance.into_iter()) {
        let mut count = 0;
        for v in 0..t {
            if v * (t - v) > d {
                count += 1;
            }
        }
        total *= count;
    }

    total
}

pub fn part2(input: &String) -> u32 {
    let (time, distance) = parse_input2(input);

    let mut total = 0;
    for v in 0..time {
        if v * (time - v) > distance {
            total += 1;
        }
    }

    total
}

pub mod test {
    pub fn part1(input: &String) -> u32 {
        let r = super::part1(input);
        assert_eq!(r, 288);

        r
    }

    pub fn part2(input: &String) -> u32 {
        let r = super::part2(input);
        assert_eq!(r, 71503);

        r
    }
}
