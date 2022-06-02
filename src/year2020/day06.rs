use std::collections::HashSet;

pub fn part1(input: &[String]) {
    let mut total = 0;
    let mut group = HashSet::new();
    for line in input {
        if line.is_empty() {
            total += group.len();
            group.clear();
        } else {
            group = group.union(&parse(line)).copied().collect();
        }
    }
    total += group.len();
    println!("{}", total);
}

pub fn part2(input: &[String]) {
    let mut total = 0;
    let mut group = HashSet::new();
    let mut new = true;
    for line in input {
        if line.is_empty() {
            total += group.len();
            group.clear();
            new = true;
        } else {
            group = if new {
                parse(line)
            } else {
                group.intersection(&parse(line)).copied().collect()
            };
            new = false;
        }
    }
    total += group.len();
    println!("{}", total);
}

fn parse(line: &str) -> HashSet<char> {
    line.chars().collect::<HashSet<_>>()
}
