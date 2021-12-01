use anyhow::Result;

pub fn test_part1(input: &String, solution: i32) -> Result<()> {
    assert_eq!(run_part1(input)?, solution);

    Ok(())
}

pub fn test_part2(input: &String, solution: i32) -> Result<()> {
    assert_eq!(run_part2(input)?, solution);

    Ok(())
}

pub fn run_part1(input: &String) -> Result<i32> {
    let mut depth_measurement_increases = 0;

    input.lines().fold(-1, |acc, line| {
        let current = line.parse().unwrap();

        println!("{} | previous measurement: {}", current, acc);

        if current > acc && acc != -1 {
            depth_measurement_increases += 1;
        }

        current
    });

    println!(
        "> total depth measurement increases: {}",
        depth_measurement_increases
    );

    Ok(depth_measurement_increases)
}

pub fn run_part2(input: &String) -> Result<i32> {
    let mut depth_measurement_increases = 0;

    input
        .lines()
        .enumerate()
        .fold(&mut [-1, -1, 1], |acc, (i, line)| {
            let current: i32 = line.parse().unwrap();

            let previous_sum = acc.iter().sum::<i32>();

            acc[0] = acc[1];
            acc[1] = acc[2];
            acc[2] = current;

            let current_sum = acc.iter().sum::<i32>();

            if i < 2 {
                return acc;
            }

            println!("{} | previous measurement: {}", current_sum, previous_sum);

            if current_sum > previous_sum && i > 2 {
                depth_measurement_increases += 1;
            }

            acc
        });

    println!(
        "> total depth measurement increases: {}",
        depth_measurement_increases
    );

    Ok(depth_measurement_increases)
}
