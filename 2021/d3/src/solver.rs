use std::str;

use anyhow::Result;

pub fn part1(input: &String) -> Result<i64> {
    let mut data: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for num in line.chars() {
            match num {
                '0' => row.push(0),
                '1' => row.push(1),
                _ => unreachable!(),
            }
        }
        data.push(row);
    }

    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();

    for i in 0..data[0].len() {
        let count_zero = data.iter().map(|row| row[i]).filter(|&x| x == 0).count();
        let count_one = data.iter().map(|row| row[i]).filter(|&x| x == 1).count();

        if count_zero > count_one {
            gamma.push(0);
            epsilon.push(1);
        } else {
            gamma.push(1);
            epsilon.push(0);
        }
    }

    let gamma_rate = i64::from_str_radix(
        str::from_utf8(&gamma.iter().map(|s| s + '0' as u8).collect::<Vec<_>>())?,
        2,
    )?;
    let epsilon_rate = i64::from_str_radix(
        str::from_utf8(&epsilon.iter().map(|s| s + '0' as u8).collect::<Vec<_>>())?,
        2,
    )?;

    println!("gamma rate: {:?} ({})", gamma, gamma_rate);
    println!("epsilon rate: {:?} ({})", epsilon, epsilon_rate);
    println!("= {}", gamma_rate * epsilon_rate);

    Ok(gamma_rate * epsilon_rate)
}

pub fn part2(input: &String) -> Result<i64> {
    let mut data: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();
        for num in line.chars() {
            match num {
                '0' => row.push(0),
                '1' => row.push(1),
                _ => unreachable!(),
            }
        }
        data.push(row);
    }

    let mut oxy_filters = Vec::new();
    let mut co2_filters = Vec::new();
    for i in 0..data[0].len() {
        let oxy_count_zero = data
            .iter()
            .filter(|&x| {
                for j in 0..oxy_filters.len() {
                    if oxy_filters[j] != x[j] {
                        return false;
                    }
                }
                return true;
            })
            .map(|row| row[i])
            .filter(|&x| x == 0)
            .count();
        let oxy_count_one = data
            .iter()
            .filter(|&x| {
                for j in 0..oxy_filters.len() {
                    if oxy_filters[j] != x[j] {
                        return false;
                    }
                }
                return true;
            })
            .map(|row| row[i])
            .filter(|&x| x == 1)
            .count();

        if oxy_count_zero > oxy_count_one {
            oxy_filters.push(0);
        } else {
            oxy_filters.push(1);
        }

        let co2_count_zero = data
            .iter()
            .filter(|&x| {
                for j in 0..co2_filters.len() {
                    if co2_filters[j] != x[j] {
                        return false;
                    }
                }
                return true;
            })
            .map(|row| row[i])
            .filter(|&x| x == 0)
            .count();
        let co2_count_one = data
            .iter()
            .filter(|&x| {
                for j in 0..co2_filters.len() {
                    if co2_filters[j] != x[j] {
                        return false;
                    }
                }
                return true;
            })
            .map(|row| row[i])
            .filter(|&x| x == 1)
            .count();

        if co2_count_zero == 0 {
            co2_filters.push(1);
        } else if co2_count_one == 0 {
            co2_filters.push(0);
        } else if co2_count_zero <= co2_count_one {
            co2_filters.push(0);
        } else {
            co2_filters.push(1);
        }
    }

    let oxy = data
        .iter()
        .filter(|&x| {
            for j in 0..oxy_filters.len() {
                if oxy_filters[j] != x[j] {
                    return false;
                }
            }
            return true;
        })
        .collect::<Vec<_>>();

    println!("filtered oxy: {:?}", oxy);

    let co2 = data
        .iter()
        .filter(|&x| {
            for j in 0..co2_filters.len() {
                if co2_filters[j] != x[j] {
                    return false;
                }
            }
            return true;
        })
        .collect::<Vec<_>>();

    println!("filtered co2: {:?}", co2);

    let oxy = i64::from_str_radix(
        str::from_utf8(&oxy[0].iter().map(|s| s + '0' as u8).collect::<Vec<_>>())?,
        2,
    )?;

    let co2 = i64::from_str_radix(
        str::from_utf8(&co2[0].iter().map(|s| s + '0' as u8).collect::<Vec<_>>())?,
        2,
    )?;

    println!("oxy: {}", oxy);
    println!("co2: {}", co2);
    println!("= {}", oxy * co2);

    Ok(oxy * co2)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 198);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 230);

        Ok(())
    }
}
