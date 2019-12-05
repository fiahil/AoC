use crate::opcode::{exec, read, PtrMove};

#[derive(Debug)]
pub struct Program {
    bin: Vec<i32>,
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

    pub fn get_output(&self) -> i32 {
        self.bin[0]
    }

    pub fn run(&mut self) -> &Self {
        let mut ip = 0;

        loop {
            let op = read(ip, &self.bin);
            debug!("{} | {}", ip, op);

            match exec(op, &mut self.bin) {
                PtrMove::Relative(c) => ip += c,
                PtrMove::Absolute(c) => ip = c,
                PtrMove::Halt => break,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_jump() {
        Program::new("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99").run();
    }

    #[test]
    fn test_mode() {
        assert_eq!(Program::new("1002,4,3,4,33").run(), &Program::new("1002,4,3,4,99"));
    }

    #[test]
    fn test_neg() {
        assert_eq!(Program::new("1101,100,-1,4,0").run(), &Program::new("1101,100,-1,4,99"));
    }

    #[test]
    fn test_0_0() {
        let mut p = Program::new("1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,2,19,13,23,1,23,10,27,1,13,27,31,2,31,10,35,1,35,9,39,1,39,13,43,1,13,43,47,1,47,13,51,1,13,51,55,1,5,55,59,2,10,59,63,1,9,63,67,1,6,67,71,2,71,13,75,2,75,13,79,1,79,9,83,2,83,10,87,1,9,87,91,1,6,91,95,1,95,10,99,1,99,13,103,1,13,103,107,2,13,107,111,1,111,9,115,2,115,10,119,1,119,5,123,1,123,2,127,1,127,5,0,99,2,14,0,0");
        p.set_input(12, 2);
        p.run();
        assert_eq!(p.get_output(), 4330636);
    }

    #[test]
    fn test_0() {
        assert_eq!(Program::new("1,0,0,0,99").run(), &Program::new("2,0,0,0,99"));
    }

    #[test]
    fn test_1() {
        assert_eq!(Program::new("2,3,0,3,99").run(), &Program::new("2,3,0,6,99"));
    }

    #[test]
    fn test_2() {
        assert_eq!(Program::new("2,4,4,5,99,0").run(), &Program::new("2,4,4,5,99,9801"));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Program::new("1,1,1,4,99,5,6,0,99").run(),
            &Program::new("30,1,1,4,2,5,6,0,99")
        );
    }
}
