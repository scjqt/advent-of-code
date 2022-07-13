use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) -> impl ToString {
    let mut bags: HashMap<String, Vec<String>> = HashMap::new();
    for line in input {
        let (parent, children) = parse(line);
        for bag in children {
            bags.entry(bag.colour)
                .or_insert(Vec::new())
                .push(parent.clone());
        }
    }
    let mut contained = HashSet::new();
    traverse1(&bags, &mut contained, "shiny gold");
    contained.len() - 1
}

pub fn part2(input: &[String]) -> impl ToString {
    let mut bags: HashMap<String, Vec<FreqPair>> = HashMap::new();
    for line in input {
        let (parent, children) = parse(line);
        bags.insert(parent, children);
    }
    let mut contains = HashMap::new();
    traverse2(&bags, &mut contains, "shiny gold");
    contains.get("shiny gold").unwrap().clone()
}

fn traverse1(graph: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>, node: &str) {
    if visited.insert(node.to_string()) {
        if let Some(others) = graph.get(node) {
            others.iter().for_each(|n| traverse1(graph, visited, n));
        }
    }
}

fn traverse2(
    graph: &HashMap<String, Vec<FreqPair>>,
    visited: &mut HashMap<String, u32>,
    node: &str,
) {
    if !visited.contains_key(node) {
        if let Some(others) = graph.get(node) {
            let count = others
                .iter()
                .map(|n| {
                    traverse2(graph, visited, &n.colour);
                    n.freq * (1 + visited.get(&n.colour).unwrap())
                })
                .sum();
            visited.insert(node.to_string(), count);
        } else {
            visited.insert(node.to_string(), 0);
        }
    }
}

struct FreqPair {
    colour: String,
    freq: u32,
}

fn parse(line: &str) -> (String, Vec<FreqPair>) {
    let mut parts = line.split(" bags contain ");
    let parent = parts.next().unwrap().to_string();
    let parts = parts.next().unwrap();
    let parts = parts[..parts.len() - 1].split(", ");
    let mut children = Vec::new();
    for part in parts {
        let mut child = part.split(' ');
        if let Ok(freq) = child.next().unwrap().parse() {
            let colour = format!("{} {}", child.next().unwrap(), child.next().unwrap());
            children.push(FreqPair { colour, freq })
        }
    }
    (parent, children)
}
