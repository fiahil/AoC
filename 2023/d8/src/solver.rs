use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    id: String,
    left: Weak<RefCell<Node>>,
    right: Weak<RefCell<Node>>,
}

impl Node {
    fn new(id: String) -> Self {
        Self {
            id,
            left: Weak::new(),
            right: Weak::new(),
        }
    }
}

pub fn part1(input: &String) -> u32 {
    let (instructions, raw_nodes) = input.split_once("\n\n").unwrap();
    let ids = raw_nodes
        .lines()
        .map(|n| {
            let (id, rest) = n.split_once(" = ").unwrap();
            let (left, right) = rest.split_once(", ").unwrap();
            (
                id.to_string(),
                (left.replace("(", ""), right.replace(")", "")),
            )
        })
        .collect::<Vec<(String, (String, String))>>();

    let mut nodes = HashMap::new();
    for (id, (_, _)) in ids.iter() {
        nodes.insert(id.clone(), Rc::new(RefCell::new(Node::new(id.clone()))));
    }
    for (id, (left, right)) in ids.iter() {
        let node = nodes.get(id).unwrap();
        let left_node = nodes.get(left).unwrap();
        let right_node = nodes.get(right).unwrap();
        node.borrow_mut().left = Rc::downgrade(left_node);
        node.borrow_mut().right = Rc::downgrade(right_node);
    }

    let mut steps = 0;
    let mut current_node = nodes.get("AAA").unwrap().clone();
    for instruction in instructions.chars().cycle() {
        match instruction {
            'L' => {
                let left = current_node.borrow().left.upgrade().unwrap();
                steps += 1;
                current_node = left;
            }
            'R' => {
                let right = current_node.borrow().right.upgrade().unwrap();
                steps += 1;
                current_node = right;
            }
            _ => {}
        }

        if current_node.borrow().id == "ZZZ" {
            return steps;
        }
    }

    0
}

pub fn part2(input: &String) -> usize {
    let (instructions, raw_nodes) = input.split_once("\n\n").unwrap();
    let ids = raw_nodes
        .lines()
        .map(|n| {
            let (id, rest) = n.split_once(" = ").unwrap();
            let (left, right) = rest.split_once(", ").unwrap();
            (
                id.to_string(),
                (left.replace("(", ""), right.replace(")", "")),
            )
        })
        .collect::<Vec<(String, (String, String))>>();

    let mut nodes = HashMap::new();
    for (id, (_, _)) in ids.iter() {
        nodes.insert(id.clone(), Rc::new(RefCell::new(Node::new(id.clone()))));
    }
    for (id, (left, right)) in ids.iter() {
        let node = nodes.get(id).unwrap();
        let left_node = nodes.get(left).unwrap();
        let right_node = nodes.get(right).unwrap();
        node.borrow_mut().left = Rc::downgrade(left_node);
        node.borrow_mut().right = Rc::downgrade(right_node);
    }

    let starting_nodes = nodes
        .iter()
        .filter(|(k, _)| k.ends_with("A"))
        .map(|(_, n)| n.clone())
        .collect::<Vec<Rc<RefCell<Node>>>>();
    let node_steps = starting_nodes
        .iter()
        .map(|n| {
            let mut steps = 0;
            let mut current_node = n.clone();
            for instruction in instructions.chars().cycle() {
                match instruction {
                    'L' => {
                        let left = current_node.borrow().left.upgrade().unwrap();
                        steps += 1;
                        current_node = left;
                    }
                    'R' => {
                        let right = current_node.borrow().right.upgrade().unwrap();
                        steps += 1;
                        current_node = right;
                    }
                    _ => {}
                }

                if current_node.borrow().id.ends_with("Z") {
                    return steps;
                }
            }

            0
        })
        .collect::<Vec<usize>>();

    dbg!(&node_steps);

    lcm(node_steps.as_slice())
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub mod test {
    pub fn part1(input: &String) -> u32 {
        let r = super::part1(input);
        assert_eq!(r, 2);

        r
    }

    pub fn part2(input: &String) -> u32 {
        let r = super::part2(input);
        assert_eq!(r, 18);

        r as u32
    }
}
