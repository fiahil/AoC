use std::fmt;
use std::io;

pub enum PtrMove {
    Halt,
    Relative(usize),
    Absolute(usize),
}

pub enum Opcode {
    Add { op1: i32, op2: i32, result: usize },
    Mul { op1: i32, op2: i32, result: usize },
    In { to: usize },
    Out { from: i32 },
    Jmp { cond: i32, jmp: i32 },
    Njmp { cond: i32, jmp: i32 },
    Lt { op1: i32, op2: i32, result: usize },
    Eql { op1: i32, op2: i32, result: usize },
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
            Opcode::Jmp {cond, jmp }         => write!(fm, "Jmp  | #{}  <- {}", jmp, cond),
            Opcode::Njmp {cond, jmp }        => write!(fm, "Njmp | #{}  <- {}", jmp, cond),
            Opcode::Lt { op1, op2, result }  => write!(fm, "Lt   | [{}] <- {} + {}", result, op1, op2),
            Opcode::Eql { op1, op2, result } => write!(fm, "Eql  | [{}] <- {} + {}", result, op1, op2),
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
        "05" => {
            parameters = pad(parameters, 2);
            Opcode::Jmp {
                cond: dref(parameters.pop().unwrap(), bin[ip + 1], bin),
                jmp: dref(parameters.pop().unwrap(), bin[ip + 2], bin),
            }
        }
        "06" => {
            parameters = pad(parameters, 2);
            Opcode::Njmp {
                cond: dref(parameters.pop().unwrap(), bin[ip + 1], bin),
                jmp: dref(parameters.pop().unwrap(), bin[ip + 2], bin),
            }
        }
        "07" => {
            parameters = pad(parameters, 3);
            Opcode::Lt {
                op1: dref(parameters.pop().unwrap(), bin[ip + 1], bin),
                op2: dref(parameters.pop().unwrap(), bin[ip + 2], bin),
                result: bin[ip + 3] as usize,
            }
        }
        "08" => {
            parameters = pad(parameters, 3);
            Opcode::Eql {
                op1: dref(parameters.pop().unwrap(), bin[ip + 1], bin),
                op2: dref(parameters.pop().unwrap(), bin[ip + 2], bin),
                result: bin[ip + 3] as usize,
            }
        }
        "99" => Opcode::Halt,
        o => panic!("Unknown opcode! {}", o),
    }
}

pub fn exec(opcode: Opcode, bin: &mut Vec<i32>) -> PtrMove {
    match opcode {
        Opcode::Add { op1, op2, result } => {
            debug!(" {} <- {}", bin[result], op1 + op2);
            bin[result] = op1 + op2;
            PtrMove::Relative(4)
        }
        Opcode::Mul { op1, op2, result } => {
            debug!(" {} <- {}", bin[result], op1 * op2);
            bin[result] = op1 * op2;
            PtrMove::Relative(4)
        }
        Opcode::In { to } => {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let i: i32 = input.trim().parse().unwrap();
            debug!(" {} <- {} ", bin[to], input);
            bin[to] = i;
            PtrMove::Relative(2)
        }
        Opcode::Out { from } => {
            debug!(" {} -> () ", from);
            println!("> {}", from);
            PtrMove::Relative(2)
        }
        Opcode::Jmp { cond, jmp } => {
            if cond != 0 {
                debug!(" {} ? -> #{}", cond, jmp);
                PtrMove::Absolute(jmp as usize)
            } else {
                debug!(" {} ? x", cond);
                PtrMove::Relative(3)
            }
        }
        Opcode::Njmp { cond, jmp } => {
            if cond == 0 {
                debug!(" {} ? -> #{}", cond, jmp);
                PtrMove::Absolute(jmp as usize)
            } else {
                debug!(" {} ? x", cond);
                PtrMove::Relative(3)
            }
        }
        Opcode::Lt { op1, op2, result } => {
            if op1 < op2 {
                debug!(" {} <- 1", bin[result]);
                bin[result] = 1;
            } else {
                debug!(" {} <- 0", bin[result]);
                bin[result] = 0;
            }
            PtrMove::Relative(4)
        }
        Opcode::Eql { op1, op2, result } => {
            if op1 == op2 {
                debug!(" {} <- 1", bin[result]);
                bin[result] = 1;
            } else {
                debug!(" {} <- 0", bin[result]);
                bin[result] = 0;
            }
            PtrMove::Relative(4)
        }
        Opcode::Halt => PtrMove::Halt,
    }
}
