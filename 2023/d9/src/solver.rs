fn reduce(serie: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut i = 0;
    let mut j = 1;
    while j < serie.len() {
        let sub = serie[j] - serie[i];
        result.push(sub);

        i += 1;
        j += 1;
    }
    dbg!(result)
}

fn recursion_part1(serie: &Vec<i32>) -> i32 {
    if serie.iter().all(|&n| n == 0) {
        return 0;
    }

    let new_serie = reduce(serie);
    let new_element = recursion_part1(&new_serie);
    let a = new_element + serie.last().unwrap();
    println!("X={} {:?} ; A={} {:?}", new_element, new_serie, a, serie);

    a
}
fn recursion_part2(serie: &Vec<i32>) -> i32 {
    if serie.iter().all(|&n| n == 0) {
        return 0;
    }

    let new_serie = reduce(serie);
    let new_element = recursion_part2(&new_serie);
    let a = -new_element + serie.first().unwrap();
    println!("X={} {:?} ; A={} {:?}", new_element, new_serie, a, serie);

    a
}

pub fn part1(input: &String) -> u32 {
    let series = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    series
        .iter()
        .map(|serie| recursion_part1(serie))
        .sum::<i32>() as u32
}

pub fn part2(input: &String) -> u32 {
    let series = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    series
        .iter()
        .map(|serie| recursion_part2(serie))
        .sum::<i32>() as u32
}

pub mod test {
    pub fn part1(input: &String) -> u32 {
        let r = super::part1(input);
        assert_eq!(r, 114);

        r
    }

    pub fn part2(input: &String) -> u32 {
        let r = super::part2(input);
        assert_eq!(r, 2);

        r
    }
}
