use anyhow::Result;
use std::collections::HashSet;

pub fn part1(input: &String) -> Result<i32> {
    let c = input.lines().into_iter().fold(0, |acc, s| {
        let (h1, h2) = s.split_at(s.len() / 2);

        let mut s1 = HashSet::new();
        s1.extend(h1.chars());

        let mut s2 = HashSet::new();
        s2.extend(h2.chars());

        let c = s1.intersection(&s2).last().unwrap();

        acc + "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .enumerate()
            .find_map(|(idx, cc)| if c == &cc { Some(idx as i32 + 1) } else { None })
            .unwrap()
    });

    Ok(c)
}

pub fn part2(input: &String) -> Result<i32> {
    let cs = input
        .lines()
        .into_iter()
        .fold((Vec::new(), Vec::new()), |mut acc, s| {
            let mut h = HashSet::new();

            h.extend(s.chars());

            acc.0.push(h);

            if acc.0.len() == 3 {
                let inter = acc
                    .0
                    .into_iter()
                    .reduce(|a, b| {
                        let mut x = HashSet::new();
                        x.extend(a.intersection(&b).collect::<Vec<_>>());
                        x
                    })
                    .unwrap();
                let inter = inter.iter().take(1).last().unwrap();

                acc.0 = Vec::new();
                acc.1.push(inter.clone());
            }

            acc
        });

    let c = cs.1.iter().fold(0, |acc, c| {
        acc + "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .enumerate()
            .find_map(|(idx, cc)| if c == &cc { Some(idx as i32 + 1) } else { None })
            .unwrap()
    });

    Ok(c)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 157);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 70);

        Ok(())
    }
}
