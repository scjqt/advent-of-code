use regex::Regex;
use std::collections::HashMap;

pub fn part1(input: &[String]) -> i32 {
    let mut i = 0;
    let mut rules = read_rules(input, &mut i);
    let valid = Regex::new(&format!("^{}$", resolve_rule(&mut rules, 0))).unwrap();
    let mut total = 0;
    for j in i..input.len() {
        total += if valid.is_match(&input[j]) { 1 } else { 0 };
    }
    total
}

pub fn part2(input: &[String]) -> i32 {
    let mut i = 0;
    let mut rules = read_rules(input, &mut i);
    rules.insert(
        8,
        Rule::Unresolved(vec![
            vec![42],
            vec![42, 42],
            vec![42, 42, 42],
            vec![42, 42, 42, 42],
            vec![42, 42, 42, 42, 42],
        ]),
    );
    rules.insert(
        11,
        Rule::Unresolved(vec![
            vec![42, 31],
            vec![42, 42, 31, 31],
            vec![42, 42, 42, 31, 31, 31],
            vec![42, 42, 42, 42, 31, 31, 31, 31],
        ]),
    );
    let valid = Regex::new(&format!("^{}$", resolve_rule(&mut rules, 0))).unwrap();
    let mut total = 0;
    for j in i..input.len() {
        total += if valid.is_match(&input[j]) { 1 } else { 0 };
    }
    total
}

fn read_rules(input: &[String], i: &mut usize) -> HashMap<u8, Rule> {
    let mut rules = HashMap::new();
    while input[*i].len() > 0 {
        let mut parts = input[*i].split(": ");
        rules.insert(
            parts.next().unwrap().parse().unwrap(),
            Rule::from(parts.next().unwrap()),
        );
        *i += 1;
    }
    *i += 1;
    rules
}

fn resolve_rule(rules: &mut HashMap<u8, Rule>, number: u8) -> String {
    match rules.get(&number).unwrap() {
        Rule::Resolved(string) => string.clone(),
        Rule::Unresolved(definition) => {
            let definition = definition.clone();
            let mut string = "(".to_string();
            for (i, set) in definition.iter().enumerate() {
                string += "(";
                for &rule in set {
                    string += &resolve_rule(rules, rule);
                }
                string += ")";
                if i < definition.len() - 1 {
                    string += "|";
                }
            }
            string += ")";
            rules.insert(number, Rule::Resolved(string.clone()));
            string
        }
    }
}

enum Rule {
    Resolved(String),
    Unresolved(Vec<Vec<u8>>),
}

impl Rule {
    fn from(string: &str) -> Rule {
        if &string[0..=0] == "\"" {
            Rule::Resolved(String::from(&string[1..=1]))
        } else {
            let mut definition = Vec::new();
            string
                .split(" | ")
                .for_each(|x| definition.push(x.split(' ').map(|y| y.parse().unwrap()).collect()));
            Rule::Unresolved(definition)
        }
    }
}
