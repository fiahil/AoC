use std::fmt;

pub enum Opcode {
    Add { op1: i32, op2: i32, result: usize },
    Mul { op1: i32, op2: i32, result: usize },
    Halt,
}

impl fmt::Display for Opcode {
    fn fmt(&self, fm: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Opcode::Add { op1, op2, result } => {
                write!(fm, "Add | [{}] <- {} + {}", result, op1, op2)
            }
            Opcode::Mul { op1, op2, result } => {
                write!(fm, "Mul | [{}] <- {} + {}", result, op1, op2)
            }
            Opcode::Halt => write!(fm, "Halt | <END>"),
        }
    }
}

pub fn read(ip: usize, bin: &Vec<i32>) -> Opcode {
    match bin[ip] {
        1 => Opcode::Add {
            op1: bin[bin[ip + 1] as usize],
            op2: bin[bin[ip + 2] as usize],
            result: bin[ip + 3] as usize,
        },
        2 => Opcode::Mul {
            op1: bin[bin[ip + 1] as usize],
            op2: bin[bin[ip + 2] as usize],
            result: bin[ip + 3] as usize,
        },
        99 => Opcode::Halt,
        _ => panic!("Unknown opcode!"),
    }
}

pub fn exec(opcode: Opcode, bin: &mut Vec<i32>) -> Option<usize> {
    match opcode {
        Opcode::Add { op1, op2, result } => {
            debug!(" {} <- {}", bin[result], op1 + op2);
            bin[result] = op1 + op2;
            Some(4)
        }
        Opcode::Mul { op1, op2, result } => {
            debug!(" {} <- {}", bin[result], op1 * op2);
            bin[result] = op1 * op2;
            Some(4)
        }
        Opcode::Halt => None,
    }
}
