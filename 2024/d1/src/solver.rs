use anyhow::Result;

pub fn part1(input: &String) -> Result<i32> {
    let mut lists = input
        .split(&['\n', ' '])
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().expect("a number"))
        .enumerate()
        .fold((Vec::new(), Vec::new()), |mut acc, (i, x)| {
            if i % 2 == 0 {
                acc.0.push(x);
            } else {
                acc.1.push(x);
            }

            acc
        });

    lists.0.sort();
    lists.1.sort();

    // iterate through the lists and compare the values
    let r = lists
        .0
        .iter()
        .zip(lists.1.iter())
        .fold(0, |acc, (a, b)| (a - b).abs() + acc);

    Ok(r)
}

pub fn part2(input: &String) -> Result<i32> {
    let lists = input
        .split(&['\n', ' '])
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().expect("a number"))
        .enumerate()
        .fold((Vec::new(), Vec::new()), |mut acc, (i, x)| {
            if i % 2 == 0 {
                acc.0.push(x);
            } else {
                acc.1.push(x);
            }

            acc
        });

    let mut r = 0;
    for x in lists.0.iter() {
        // find the count of the number in the second list
        let count = lists.1.iter().filter(|&y| y == x).count();

        r += x * count as i32;
    }

    Ok(r)
}

pub mod test {
    use super::*;

    pub fn test_part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 11);

        Ok(())
    }

    pub fn test_part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 31);

        Ok(())
    }
}
