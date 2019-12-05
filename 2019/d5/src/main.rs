use d5::program::Program;
use simple_logger;
use std::fs;

fn main() {
    simple_logger::init().unwrap();
    let initial = fs::read_to_string("input.txt").unwrap();
    let mut p = Program::new(&initial);
    p.run();
}
