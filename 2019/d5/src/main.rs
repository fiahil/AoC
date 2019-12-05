use d5::computer::Computer;
use simple_logger;
use std::fs;

fn main() {
    simple_logger::init().unwrap();
    let initial = fs::read_to_string("input.txt").unwrap();
    let mut p = Computer::new(&initial);
    p.run();
}
