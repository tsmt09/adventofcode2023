use std::collections::{HashMap, VecDeque};

fn main() {
    let start = std::time::Instant::now();
    let bytes = include_bytes!("../../inputs/a8.txt");
    let (assignments, nodes) = assignments_and_nodes(bytes);
    let r1 = nodes.steps(&assignments, "AAA", "ZZZ");
    println!(
        "a1: {} - took {}µs",
        r1,
        std::time::Instant::now().duration_since(start).as_micros()
    );
    let results: Vec<usize> = nodes
        .map
        .iter()
        .filter(|(k, _v)| k.ends_with('A'))
        .map(|(k, _v)| nodes.steps(&assignments, k, "Z"))
        .collect();
    let r2: usize = results
        .into_iter()
        .reduce(num::integer::lcm)
        .unwrap();
    println!(
        "a2: {} - took {}µs",
        r2,
        std::time::Instant::now().duration_since(start).as_micros()
    );
}

#[derive(Clone, Debug)]
enum Direction {
    Left,
    Right,
}

impl From<&u8> for Direction {
    fn from(value: &u8) -> Self {
        match value {
            82 => Self::Right,
            76 => Self::Left,
            _ => panic!("Unknown Direction {}", value),
        }
    }
}

#[derive(Debug, Clone)]
struct Nodes<'a> {
    pub map: HashMap<&'a str, Node<'a>>,
}

#[derive(Debug, Clone)]
struct Node<'a> {
    l: &'a str,
    r: &'a str,
}

impl<'a> Node<'a> {
    fn from_line(line: &'a [u8]) -> (&'a str, Node<'a>) {
        let position: &'a str = std::str::from_utf8(&line[0..3]).unwrap();
        let l: &'a str = std::str::from_utf8(&line[7..10]).unwrap();
        let r: &'a str = std::str::from_utf8(&line[12..15]).unwrap();
        (position, Node { l, r })
    }
}

impl<'a> Nodes<'a> {
    fn steps(&self, assignments: &Vec<Direction>, start: &str, ends_with: &str) -> usize {
        // start at AAA
        let mut current_node = start;
        let mut next_nodes = &self.map[current_node];
        let asslen = assignments.len();
        let mut ass_index = 0;
        let mut steps = 0;
        while !current_node.ends_with(ends_with) {
            let ass = &assignments[ass_index];
            (current_node, next_nodes) = match ass {
                Direction::Left => (next_nodes.l, &self.map[&next_nodes.l]),
                Direction::Right => (next_nodes.r, &self.map[&next_nodes.r]),
            };
            ass_index = (ass_index + 1) % asslen;
            steps += 1;
        }
        steps
    }
}

fn assignments_and_nodes(bytes: &[u8]) -> (Vec<Direction>, Nodes) {
    let mut lines: VecDeque<&[u8]> = bytes.split(|x| x == &b'\n').collect();
    let assignments: Vec<Direction> = lines
        .pop_front()
        .unwrap()
        .iter()
        .map(Direction::from)
        .collect();
    let mut nodes = Nodes {
        map: HashMap::new(),
    };
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (pos, node) = Node::from_line(line);
        nodes.map.insert(pos, node);
    }
    (assignments, nodes)
}

#[cfg(test)]
mod tests {
    use crate::assignments_and_nodes;

    #[test]
    fn test_input_1() {
        let bytes = include_bytes!("../../inputs/a8_test.txt");
        let (assignments, nodes) = assignments_and_nodes(bytes);
        let r = nodes.steps(&assignments, "AAA", "ZZZ");
        assert_eq!(r, 2);
    }

    #[test]
    fn test_input_2() {
        let bytes = include_bytes!("../../inputs/a8_test2.txt");
        let (assignments, nodes) = assignments_and_nodes(bytes);
        let results: Vec<usize> = nodes
            .map
            .iter()
            .filter(|(k, _v)| k.ends_with("A"))
            .map(|(k, _v)| nodes.steps(&assignments, k, "Z"))
            .collect();
        let r: usize = results
            .into_iter()
            .reduce(|acc, x| num::integer::lcm(acc, x))
            .unwrap();
        assert_eq!(r, 6);
    }
}
