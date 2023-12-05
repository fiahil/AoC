use rayon::prelude::*;
use std::ops::Range;

use indicatif::{ParallelProgressIterator, ProgressIterator, ProgressStyle};

#[derive(Debug)]
enum Selector {
    Unset,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug)]
struct Maps {
    seed_to_soil: Vec<(Range<u64>, u64)>,
    soil_to_fertilizer: Vec<(Range<u64>, u64)>,
    fertilizer_to_water: Vec<(Range<u64>, u64)>,
    water_to_light: Vec<(Range<u64>, u64)>,
    light_to_temperature: Vec<(Range<u64>, u64)>,
    temperature_to_humidity: Vec<(Range<u64>, u64)>,
    humidity_to_location: Vec<(Range<u64>, u64)>,
}

impl Maps {
    fn new() -> Self {
        Self {
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),
        }
    }

    fn add_range(&mut self, selector: &Selector, source: u64, dest: u64, length: u64) {
        match selector {
            Selector::Unset => unreachable!(),
            Selector::SeedToSoil => {
                self.seed_to_soil.push((source..(source + length), dest));
            }
            Selector::SoilToFertilizer => {
                self.soil_to_fertilizer
                    .push((source..(source + length), dest));
            }
            Selector::FertilizerToWater => {
                self.fertilizer_to_water
                    .push((source..(source + length), dest));
            }
            Selector::WaterToLight => {
                self.water_to_light.push((source..(source + length), dest));
            }
            Selector::LightToTemperature => {
                self.light_to_temperature
                    .push((source..(source + length), dest));
            }
            Selector::TemperatureToHumidity => {
                self.temperature_to_humidity
                    .push((source..(source + length), dest));
            }
            Selector::HumidityToLocation => {
                self.humidity_to_location
                    .push((source..(source + length), dest));
            }
        }
    }

    // Get the location for a given seed
    fn resolve(&self, seed: &u64) -> u64 {
        fn find(ranges: &Vec<(Range<u64>, u64)>, item: u64) -> u64 {
            for (range, dest) in ranges {
                if range.contains(&item) {
                    return dest + (item - range.start);
                }
            }

            item
        }

        let soil = find(&self.seed_to_soil, *seed);
        let fertilizer = find(&self.soil_to_fertilizer, soil);
        let water = find(&self.fertilizer_to_water, fertilizer);
        let light = find(&self.water_to_light, water);
        let temperature = find(&self.light_to_temperature, light);
        let humidity = find(&self.temperature_to_humidity, temperature);
        let location = find(&self.humidity_to_location, humidity);

        location
    }
}

pub fn part1(input: &String) -> u64 {
    let mut maps = Maps::new();
    let mut selector = Selector::Unset;
    let mut seeds = Vec::new();
    for line in input.lines() {
        println!("{}", line);

        if line.starts_with("seeds:") {
            line[6..]
                .split_whitespace()
                .for_each(|s| seeds.push(s.trim().parse::<u64>().unwrap()));
            continue;
        }

        match line {
            "seed-to-soil map:" => selector = Selector::SeedToSoil,
            "soil-to-fertilizer map:" => selector = Selector::SoilToFertilizer,
            "fertilizer-to-water map:" => selector = Selector::FertilizerToWater,
            "water-to-light map:" => selector = Selector::WaterToLight,
            "light-to-temperature map:" => selector = Selector::LightToTemperature,
            "temperature-to-humidity map:" => selector = Selector::TemperatureToHumidity,
            "humidity-to-location map:" => selector = Selector::HumidityToLocation,
            l if l.len() > 1 => {
                let mut parts = l.split_whitespace();
                let to_start = parts.next().unwrap().parse::<u64>().unwrap();
                let from_start = parts.next().unwrap().parse::<u64>().unwrap();
                let length = parts.next().unwrap().parse::<u64>().unwrap();

                maps.add_range(&selector, from_start, to_start, length);
            }
            _ => continue,
        }
    }

    dbg!(&maps);

    seeds.iter().map(|s| maps.resolve(s)).min().unwrap()
}

pub fn part2(input: &String) -> u64 {
    let mut maps = Maps::new();
    let mut selector = Selector::Unset;
    let mut seeds = Vec::new();

    for line in input.lines() {
        println!("{}", line);

        if line.starts_with("seeds:") {
            let numbers: Vec<u64> = line[6..]
                .split_whitespace()
                .map(|n| n.trim().parse::<u64>().unwrap())
                .collect();
            for i in (0..numbers.len()).step_by(2).progress_with_style(
                ProgressStyle::with_template(
                    "[{elapsed_precise}] [{bar:40.cyan/blue}] ({pos:>3}/{len:3}, ETA {eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
            ) {
                for number in numbers[i]..(numbers[i] + numbers[i + 1]) {
                    seeds.push(number);
                }
            }
            continue;
        }

        match line {
            "seed-to-soil map:" => selector = Selector::SeedToSoil,
            "soil-to-fertilizer map:" => selector = Selector::SoilToFertilizer,
            "fertilizer-to-water map:" => selector = Selector::FertilizerToWater,
            "water-to-light map:" => selector = Selector::WaterToLight,
            "light-to-temperature map:" => selector = Selector::LightToTemperature,
            "temperature-to-humidity map:" => selector = Selector::TemperatureToHumidity,
            "humidity-to-location map:" => selector = Selector::HumidityToLocation,
            l if l.len() > 1 => {
                let mut parts = l.split_whitespace();
                let to_start = parts.next().unwrap().parse::<u64>().unwrap();
                let from_start = parts.next().unwrap().parse::<u64>().unwrap();
                let length = parts.next().unwrap().parse::<u64>().unwrap();

                maps.add_range(&selector, from_start, to_start, length);
            }
            _ => continue,
        }
    }

    seeds
        .into_par_iter()
        .fold_chunks_with(100_000, u64::MAX , |acc, e| {
            let location = maps.resolve(&e);

            acc.min(location)
        })
        .progress_with_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{bar:40.cyan/blue}] ({pos:>11}/{len:11} ({percent:>3}%), ETA {eta_precise})",
            )
            .unwrap()
            .progress_chars("#>-"),
        )
        .min()
        .unwrap()
}

pub mod test {
    pub fn part1(input: &String) -> u64 {
        let r = super::part1(input);
        assert_eq!(r, 35);

        r
    }

    pub fn part2(input: &String) -> u64 {
        let r = super::part2(input);
        assert_eq!(r, 46);

        r
    }
}
