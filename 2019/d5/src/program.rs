use crate::opcode::{exec, read};

#[derive(Debug)]
pub struct Program {
    pub bin: Vec<i32>,
}

impl Program {
    pub fn new(input: &str) -> Self {
        let p = Program {
            bin: input
                .trim_matches('\n')
                .split(',')
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        };

        debug!("Created program {:?}", p);

        p
    }

    pub fn set_input(&mut self, noun: i32, verb: i32) {
        info!("setting input to noun {}\tverb {}", noun, verb);
        self.bin[1] = noun;
        self.bin[2] = verb;
    }

    pub fn run(&mut self) -> &Self {
        let mut ip = 0;

        loop {
            let op = read(ip, &self.bin);
            debug!("{} | {}", ip, op);

            match exec(op, &mut self.bin) {
                Some(c) => ip += c,
                None => break,
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
