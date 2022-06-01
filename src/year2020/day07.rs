use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) {
    let mut bags: HashMap<String, Vec<String>> = HashMap::new();
    for line in input {
        let contain = from_str(line);
        for bag in &contain.1 {
            let contained_by = bags.entry(bag.colour.clone()).or_insert(Vec::new());
            contained_by.push(contain.0.clone());
        }
    }
    let mut contained = HashSet::new();
    traverse_one(&bags, &mut contained, "shiny gold");
    println!("{}", contained.len() - 1);
}

pub fn part2(input: &[String]) {
    let mut bags: HashMap<String, Vec<FreqPair>> = HashMap::new();
    for line in input {
        let contain = from_str(line);
        bags.insert(contain.0, contain.1);
    }
    let mut contains = HashMap::new();
    traverse_two(&bags, &mut contains, "shiny gold");
    println!("{}", contains.get("shiny gold").unwrap());
}

fn traverse_one(graph: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>, node: &str) {
    if visited.insert(node.to_string()) {
        if let Some(others) = graph.get(node) {
            others.iter().for_each(|n| traverse_one(graph, visited, n));
        }
    }
}

fn traverse_two(
    graph: &HashMap<String, Vec<FreqPair>>,
    visited: &mut HashMap<String, u32>,
    node: &str,
) {
    if !visited.contains_key(node) {
        if let Some(others) = graph.get(node) {
            let count = others
                .iter()
                .map(|n| {
                    traverse_two(graph, visited, &n.colour);
                    n.freq * (1 + visited.get(&n.colour).unwrap())
                })
                .sum::<u32>();
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

fn from_str(line: &str) -> (String, Vec<FreqPair>) {
    let mut parts = line.split(" bags contain ");
    let parent = parts.next().unwrap().to_string();
    let parts = parts.next().unwrap();
    let parts = parts[..parts.len() - 1].split(", ");
    let mut children = Vec::new();
    for part in parts {
        let mut child = part.split(' ');
        if let Ok(freq) = child.next().unwrap().parse::<u32>() {
            let colour: String = format!("{} {}", child.next().unwrap(), child.next().unwrap());
            children.push(FreqPair { colour, freq })
        }
    }

    (String::from(parent), children)
}
