use std::fmt;
use std::io;

pub enum Opcode {
    Add { op1: i32, op2: i32, result: usize },
    Mul { op1: i32, op2: i32, result: usize },
    In { to: usize },
    Out { from: i32 },
    Halt,
}

impl fmt::Display for Opcode {
    fn fmt(&self, fm: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        #[rustfmt::skip]
        let m = match *self {
            Opcode::Add { op1, op2, result } => write!(fm, "Add  | [{}] <- {} + {}", result, op1, op2),
            Opcode::Mul { op1, op2, result } => write!(fm, "Mul  | [{}] <- {} + {}", result, op1, op2),
            Opcode::In { to }                => write!(fm, "In   | [{}] <- ()", to),
            Opcode::Out { from }             => write!(fm, "Out  | [{}] -> ()", from),
            Opcode::Halt                     => write!(fm, "Halt | <END>"),
        };

        m
    }
}

pub fn read(ip: usize, bin: &Vec<i32>) -> Opcode {
    fn pad(p: String, len: usize) -> String {
        let mut r = String::new();

        if p.len() < len {
            for _ in 0..(len - p.len()) {
                r.push('0');
            }
        }
        r.push_str(p.as_str());

        r
    }

    fn dref(mode: char, value: i32, bin: &Vec<i32>) -> i32 {
        match mode {
            '0' => bin[value as usize],
            '1' => value,
            x => panic!("invalid parameter mode {}", x),
        }
    }

    let original = bin[ip].to_string();
    let mut parameters = String::from("");
    let code: String;

    if original.len() > 2 {
        let x = original.split_at(original.len() - 2);
        code = x.1.to_string();
        parameters = x.0.to_string();
    } else {
        code = format!("{:0>2}", original);
    }

    match code.as_str() {
        "01" => {
            parameters = pad(parameters, 4);
            Opcode::Add {
                op1: dref(parameters.pop().unwrap(), bin[ip + 1], bin),
                op2: dref(parameters.pop().unwrap(), bin[ip + 2], bin),
                result: bin[ip + 3] as usize,
            }
        }
        "02" => {
            parameters = pad(parameters, 4);
            Opcode::Mul {
                op1: dref(parameters.pop().unwrap(), bin[ip + 1], bin),
                op2: dref(parameters.pop().unwrap(), bin[ip + 2], bin),
                result: bin[ip + 3] as usize,
            }
        }
        "03" => Opcode::In {
            to: bin[ip + 1] as usize,
        },
        "04" => {
            parameters = pad(parameters, 1);
            Opcode::Out {
                from: dref(parameters.pop().unwrap(), bin[ip + 1], bin),
            }
        }
        "99" => Opcode::Halt,
        o => panic!("Unknown opcode! {}", o),
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
        Opcode::In { to } => {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let i: i32 = input.trim().parse().unwrap();
            debug!(" {} <- {} ", bin[to], input);
            bin[to] = i;
            Some(2)
        }
        Opcode::Out { from } => {
            debug!(" {} -> () ", from);
            println!("> {}", from);
            Some(2)
        }
        Opcode::Halt => None,
    }
}
