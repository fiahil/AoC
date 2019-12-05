use crate::opcode::{exec, read, PtrMove};

#[derive(Debug)]
pub struct Computer {
    mem: Vec<i32>,
}

impl Computer {
    pub fn new(input: &str) -> Self {
        let p = Computer {
            mem: input
                .trim_matches('\n')
                .split(',')
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        };

        debug!("Created Computer {:?}", p);

        p
    }

    pub fn raw_mem(&self) -> String {
        self.mem
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn set_input(&mut self, noun: i32, verb: i32) {
        info!("setting input to noun {}\tverb {}", noun, verb);
        self.mem[1] = noun;
        self.mem[2] = verb;
    }

    pub fn get_output(&self) -> i32 {
        self.mem[0]
    }

    pub fn run(&mut self) -> &Self {
        let mut ip = 0;

        loop {
            let op = read(ip, &self.mem);
            debug!("{} | {}", ip, op);

            match exec(op, &mut self.mem) {
                PtrMove::Relative(c) => ip += c,
                PtrMove::Absolute(c) => ip = c,
                PtrMove::Halt => break,
            }
        }

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_jump() {
    //     Computer::new("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99").run();
    // }

    #[test]
    fn test_0_0() {
        let mut p = Computer::new("1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,2,19,13,23,1,23,10,27,1,13,27,31,2,31,10,35,1,35,9,39,1,39,13,43,1,13,43,47,1,47,13,51,1,13,51,55,1,5,55,59,2,10,59,63,1,9,63,67,1,6,67,71,2,71,13,75,2,75,13,79,1,79,9,83,2,83,10,87,1,9,87,91,1,6,91,95,1,95,10,99,1,99,13,103,1,13,103,107,2,13,107,111,1,111,9,115,2,115,10,119,1,119,5,123,1,123,2,127,1,127,5,0,99,2,14,0,0");
        p.set_input(12, 2);
        p.run();
        assert_eq!(p.get_output(), 4330636);
    }
}
