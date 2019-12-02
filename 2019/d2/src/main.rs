#[macro_use]
extern crate log;

use simplelog::{Config, LevelFilter, SimpleLogger};

use std::collections::VecDeque;

#[derive(Debug)]
struct Program {
    bin: VecDeque<usize>,
}

impl Program {
    fn new(input: &str) -> Self {
        let p = Program {
            bin: input
                .trim_matches('\n')
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<VecDeque<usize>>(),
        };

        debug!("Created program {:?}", p);

        p
    }

    fn set_input(&mut self, noun: usize, verb: usize) {
        info!("setting input to noun {}\tverb {}", noun, verb);
        self.bin[1] = noun;
        self.bin[2] = verb;
    }

    fn run(&mut self) -> &Self {
        fn add(op1: usize, op2: usize, store: usize, bin: &mut VecDeque<usize>) {
            debug!(
                "Reading {} + {} = {} (original: {})",
                bin[op1],
                bin[op2],
                bin[op1] + bin[op2],
                bin[store]
            );
            bin[store] = bin[op1] + bin[op2]
        }

        fn mul(op1: usize, op2: usize, store: usize, bin: &mut VecDeque<usize>) {
            debug!(
                "Reading {} * {} = {} (original: {})",
                bin[op1],
                bin[op2],
                bin[op1] * bin[op2],
                bin[store]
            );
            bin[store] = bin[op1] * bin[op2]
        }

        for i in (0..self.bin.len()).step_by(4) {
            let instruction = self.bin[i];

            debug!("i: {}\t| instruction {}", i, instruction);

            match instruction {
                1 => add(
                    self.bin[i + 1].into(),
                    self.bin[i + 2].into(),
                    self.bin[i + 3].into(),
                    &mut self.bin,
                ),
                2 => mul(
                    self.bin[i + 1].into(),
                    self.bin[i + 2].into(),
                    self.bin[i + 3].into(),
                    &mut self.bin,
                ),
                99 => {
                    debug!("Halt! State: {:?}", self);
                    return self;
                }
                _ => {
                    error!("Unknown opcode! Panic! State: {:?}", self);
                    return self;
                }
            }
        }

        self
    }
}

impl PartialEq for Program {
    fn eq(&self, other: &Program) -> bool {
        self.bin == other.bin
    }
}

use std::fs;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
    let initial = fs::read_to_string("input.txt").unwrap();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut p = Program::new(&initial);
            p.set_input(noun, verb);
            p.run();

            if p.bin[0] == 19690720 {
                info!("Program output matches ! input is: {}", 100 * noun + verb);
                return ();
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_0_0() {
        let mut p = Program::new(&fs::read_to_string("input.txt").unwrap());
        p.set_input(12, 2);
        p.run();
        assert_eq!(p.bin[0], 4330636);
    }

    #[test]
    fn test_0() {
        assert_eq!(
            Program::new("1,0,0,0,99").run(),
            &Program::new("2,0,0,0,99")
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(
            Program::new("2,3,0,3,99").run(),
            &Program::new("2,3,0,6,99")
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Program::new("2,4,4,5,99,0").run(),
            &Program::new("2,4,4,5,99,9801")
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Program::new("1,1,1,4,99,5,6,0,99").run(),
            &Program::new("30,1,1,4,2,5,6,0,99")
        );
    }
}
