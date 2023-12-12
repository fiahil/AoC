use std::collections::{HashSet, VecDeque};

use itertools::{chain, repeat_n, Itertools};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

#[derive(Debug)]
struct SpringRow {
    springs: Vec<Spring>,
    manifest: Vec<usize>,
}

impl From<&str> for SpringRow {
    fn from(s: &str) -> Self {
        let (springs_chars, manifest_chars) = s.split_once(' ').unwrap();

        let mut springs = Vec::new();
        for c in springs_chars.chars() {
            match c {
                '?' => springs.push(Spring::Unknown),
                '.' => springs.push(Spring::Operational),
                '#' => springs.push(Spring::Damaged),
                _ => panic!("Invalid spring character: {}", c),
            }
        }

        let manifest = manifest_chars
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        SpringRow { springs, manifest }
    }
}

impl SpringRow {
    fn is_valid(&self, arrangement: &Vec<Spring>) -> bool {
        let counts = arrangement
            .iter()
            .group_by(|&s| s)
            .into_iter()
            .filter(|(s, _)| match s {
                Spring::Unknown => panic!("Unknown spring in arrangement!"),
                Spring::Operational => false,
                Spring::Damaged => true,
            })
            .map(|(_, count)| count.count())
            .collect::<Vec<usize>>();

        counts == self.manifest
    }

    fn count_arrangements(&self) -> u32 {
        let number_of_unknowns = self
            .springs
            .iter()
            .filter(|s| **s == Spring::Unknown)
            .count();
        let number_of_damaged_to_find = self.manifest.iter().sum::<usize>()
            - self
                .springs
                .iter()
                .filter(|s| **s == Spring::Damaged)
                .count();

        // we try each combinnation of arrangements
        let mut arrangements = HashSet::new();

        let must_start_with: VecDeque<Spring> = VecDeque::new();

        let set = chain(
            repeat_n(
                Spring::Operational,
                number_of_unknowns - number_of_damaged_to_find,
            ),
            repeat_n(Spring::Damaged, number_of_damaged_to_find),
        )
        .collect::<Vec<Spring>>();

        println!("set: {:?}", set);
        println!("msw: {:?}", must_start_with);

        let k = set.len();
        for mut set in set.into_iter().permutations(k).unique() {
            let mut msw = must_start_with.clone();
            // println!("setc: {:?}", set);
            let arrangement = self
                .springs
                .iter()
                .cloned()
                .map(|s| match s {
                    Spring::Unknown => msw.pop_front().or_else(|| set.pop()).unwrap(),
                    _ => s,
                })
                .collect::<Vec<Spring>>();

            if self.is_valid(&arrangement) {
                arrangements.insert(arrangement);
            }
        }

        arrangements.len() as u32
    }
}

pub fn part1(input: &String) -> u32 {
    let spring_rows: Vec<SpringRow> = input.lines().map(SpringRow::from).collect();

    spring_rows
        .into_iter()
        .map(|sr| sr.count_arrangements())
        // .enumerate()
        // .inspect(|(i, src)| println!("{} src: {}", i, src))
        // .map(|(_, src)| src)
        .sum()
}

pub fn part2(input: &String) -> u32 {
    0
}

pub mod test {
    #[test]
    pub fn part11() {
        let r = super::part1(&"???.### 1,1,3".to_string());
        assert_eq!(r, 1);
    }

    #[test]
    pub fn part12() {
        let r = super::part1(&".??..??...?##. 1,1,3".to_string());
        assert_eq!(r, 4);
    }

    #[test]
    pub fn part13() {
        let r = super::part1(&"?###???????? 3,2,1".to_string());
        assert_eq!(r, 10);
    }

    #[test]
    pub fn part14() {
        let r = super::part1(&"?#????????????##?? 1,1,10".to_string());
        assert_eq!(r, 0);
    }

    #[test]
    pub fn part15() {
        let r = super::part1(&"?.???.?#???#???.? 3,6,1".to_string());
        assert_eq!(r, 0);
    }

    #[test]
    pub fn part16() {
        let r = super::part1(&"?##?????#.#..??.? 3,3,1,1,1".to_string());
        assert_eq!(r, 7);
    }

    #[test]
    pub fn part2() {
        let r = super::part2(&"".to_string());
        assert_eq!(r, 1);
    }
}
