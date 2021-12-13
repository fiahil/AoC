use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use anyhow::Result;

struct Node {
    name: String,
    connections: RefCell<HashSet<Arc<Node>>>,
}

impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<8} -> {:?}",
            self.name,
            self.connections
                .borrow()
                .iter()
                .map(|n| &n.name)
                .collect::<Vec<_>>()
        )
    }
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            connections: RefCell::new(HashSet::new()),
        }
    }
}

struct Graph {
    nodes: HashSet<Arc<Node>>,
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in self.nodes.iter() {
            writeln!(f, "{:?}", node)?;
        }
        Ok(())
    }
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashSet::new(),
        }
    }

    fn add(&mut self, node_name: &str) -> Arc<Node> {
        let node = Arc::new(Node::new(node_name));

        if !self.nodes.contains(&node) {
            self.nodes.insert(node);
        }

        self.get(&node_name).unwrap()
    }

    fn get(&self, node_name: &str) -> Option<Arc<Node>> {
        self.nodes
            .iter()
            .find(|node| node.name == node_name.to_string())
            .cloned()
    }

    #[inline]
    fn link_to(&self, node: &Arc<Node>, other: &Arc<Node>) {
        node.connections.borrow_mut().insert(other.clone());
    }

    fn start(&self) -> Arc<Node> {
        self.get("start").unwrap()
    }
}

fn parse(input: &String) -> Graph {
    let mut graph = Graph::new();

    for line in input.lines() {
        if let Some((n1, n2)) = line.split_once("-") {
            let n1 = graph.add(n1);
            let n2 = graph.add(n2);

            graph.link_to(&n1, &n2);
            graph.link_to(&n2, &n1);
        } else {
            panic!("Invalid line: {}", line);
        }
    }

    graph
}

fn explore(
    graph: &Graph,
    node: &Arc<Node>,
    paths: &mut HashSet<String>,
    visited: &mut Vec<String>,
    with_small_exception: bool,
) {
    visited.push(node.name.clone());

    if node.name == "end" {
        paths.insert(visited.join(","));
        return;
    }

    for connection in node.connections.borrow().iter() {
        if connection.name == "start" {
            continue;
        }

        if with_small_exception
            || (connection.name == connection.name.to_lowercase()
                && !visited.contains(&connection.name))
            || (connection.name == connection.name.to_uppercase())
        {
            if with_small_exception {
                if connection.name == connection.name.to_lowercase()
                    && visited.contains(&connection.name)
                {
                    explore(graph, connection, paths, &mut visited.clone(), false);
                } else if (connection.name == connection.name.to_uppercase())
                    || (connection.name == connection.name.to_lowercase()
                        && !visited.contains(&connection.name))
                {
                    explore(
                        graph,
                        connection,
                        paths,
                        &mut visited.clone(),
                        with_small_exception,
                    );
                }
            } else {
                explore(
                    graph,
                    connection,
                    paths,
                    &mut visited.clone(),
                    with_small_exception,
                );
            }
        }
    }
}

pub fn part1(input: &String) -> Result<i32> {
    let graph = parse(input);

    println!("{:?}", graph);

    let mut visited = Vec::new();
    let mut paths = HashSet::new();
    explore(&graph, &graph.start(), &mut paths, &mut visited, false);

    for path in paths.iter() {
        println!("{}", path);
    }

    println!("explored {} paths", paths.len());

    Ok(paths.len() as i32)
}

pub fn part2(input: &String) -> Result<i32> {
    let graph = parse(input);

    println!("{:?}", graph);

    let mut visited = Vec::new();
    let mut paths = HashSet::new();
    explore(&graph, &graph.start(), &mut paths, &mut visited, true);

    for path in paths.iter() {
        println!("{}", path);
    }

    println!("explored {} paths", paths.len());

    Ok(paths.len() as i32)
}

pub mod test {
    use super::*;

    pub fn part1(input: &String) -> Result<()> {
        assert_eq!(super::part1(input)?, 10);

        Ok(())
    }

    pub fn part2(input: &String) -> Result<()> {
        assert_eq!(super::part2(input)?, 36);

        Ok(())
    }
}
