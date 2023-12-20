use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

use indicatif::{ProgressIterator, ProgressStyle};

#[derive(Debug, Clone)]
pub enum Node {
    Broadcaster {
        name: String,
        to: Vec<String>,
        pending: VecDeque<Signal>,
    },
    FlipFlop {
        name: String,
        to: Vec<String>,
        pending: VecDeque<Signal>,
        state: bool,
    },
    Conjunction {
        name: String,
        to: Vec<String>,
        pending: VecDeque<(String, Signal)>,
        state: HashMap<String, Signal>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Signal {
    Low,
    High,
}

impl Node {
    pub fn process(&mut self, nodes: RefCell<HashMap<String, Node>>) {
        let mut to_process = Vec::new();
        match self {
            Node::Broadcaster { name, to, pending } => {
                for recv in to {
                    let mut nodes = nodes.borrow_mut();
                    let node = nodes.get_mut(recv).unwrap();
                    node.post(name.clone(), pending.pop_front().unwrap());
                    to_process.push(recv);
                }
            }
            Node::FlipFlop {
                name,
                to,
                pending,
                state,
            } => {
                let signal = pending.pop_front().unwrap();
                match (signal, &state) {
                    (Signal::Low, false) => {
                        *state = true;
                        for recv in to {
                            let mut nodes = nodes.borrow_mut();
                            let node = nodes.get_mut(recv).unwrap();
                            node.post(name.clone(), Signal::High);
                            to_process.push(recv);
                        }
                    }
                    (Signal::Low, true) => {
                        *state = false;
                        for recv in to {
                            let mut nodes = nodes.borrow_mut();
                            let node = nodes.get_mut(recv).unwrap();
                            node.post(name.clone(), Signal::Low);
                            to_process.push(recv);
                        }
                    }
                    (Signal::High, _) => {}
                };
            }
            Node::Conjunction {
                name,
                to,
                pending,
                state,
            } => {
                let (sender, signal) = pending.pop_front().unwrap();
                let entry = state.entry(sender).or_insert(Signal::Low);
                *entry = signal;

                if state.values().all(|s| s == &Signal::High) {
                    for recv in to {
                        let mut nodes = nodes.borrow_mut();
                        let node = nodes.get_mut(recv).unwrap();
                        node.post(name.clone(), Signal::Low);
                        to_process.push(recv);
                    }
                } else {
                    for recv in to {
                        let mut nodes = nodes.borrow_mut();
                        let node = nodes.get_mut(recv).unwrap();
                        node.post(name.clone(), Signal::High);
                        to_process.push(recv);
                    }
                }
            }
        }
        let cloned = nodes.clone();
        for name in to_process {
            let mut nnodes = nodes.borrow_mut();
            let node = nnodes.get_mut(name).unwrap();

            node.process(cloned.clone());
        }
    }

    pub fn post(&mut self, sender: String, signal: Signal) {
        dbg!(&self, &signal);
        match self {
            Node::Broadcaster { pending, .. } => {
                pending.push_back(signal);
            }
            Node::FlipFlop { pending, .. } => {
                pending.push_back(signal);
            }
            Node::Conjunction { pending, .. } => {
                pending.push_back((sender, signal));
            }
        }
    }
}

pub fn part1(input: &String) -> u64 {
    let nodes = RefCell::new(
        input
            .trim()
            .lines()
            .map(|line| {
                let (name, out) = line.split_once(" -> ").unwrap();
                match name {
                    "broadcaster" => {
                        let to = out.split(", ").map(|s| s.to_string()).collect();
                        (
                            String::from("broadcaster"),
                            Node::Broadcaster {
                                name: String::from("broadcaster"),
                                to,
                                pending: VecDeque::new(),
                            },
                        )
                    }
                    s if s.starts_with("%") => {
                        let name = s[1..].to_string();
                        let to = out.split(", ").map(|s| s.to_string()).collect();
                        (
                            name.clone(),
                            Node::FlipFlop {
                                name,
                                to,
                                pending: VecDeque::new(),
                                state: false,
                            },
                        )
                    }
                    s if s.starts_with("&") => {
                        let name = s[1..].to_string();
                        let to = out.split(", ").map(|s| s.to_string()).collect();
                        (
                            name.clone(),
                            Node::Conjunction {
                                name,
                                to,
                                pending: VecDeque::new(),
                                state: HashMap::new(),
                            },
                        )
                    }
                    _ => panic!("Unknown node type"),
                }
            })
            .collect::<HashMap<_, _>>(),
    );

    let mut low_signal_count = 1_u64;
    let mut high_signal_count = 0_u64;

    for _ in (0..1).progress_with_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len} ({percent:>3}%), ETA {eta_precise})",
        )
        .unwrap()
        .progress_chars("#>-"),
    ) {
        let mut nnodes = nodes.borrow_mut();
        let broadcaster = nnodes.get_mut("broadcaster").unwrap();
        broadcaster.post("broadcaster".to_string(), Signal::Low);
        broadcaster.process(nodes.clone());
    }

    low_signal_count * high_signal_count
}

pub fn part2(input: &String) -> u32 {
    0
}

pub mod test {
    #[test]
    pub fn part1() {
        let r = super::part1(
            &"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
                .to_string(),
        );
        assert_eq!(r, 11687500);
    }

    #[test]
    pub fn part2() {
        let r = super::part2(&"".to_string());
        assert_eq!(r, 1);
    }
}
