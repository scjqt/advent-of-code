use std::collections::HashSet;

pub fn part1(input: &[String]) {
    let mut total = 0;
    let mut group = HashSet::new();
    for line in input {
        if line.len() == 0 {
            total += group.len();
            group.clear();
        } else {
            group = group.union(&to_set(line)).copied().collect();
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
        if line.len() == 0 {
            total += group.len();
            group.clear();
            new = true;
        } else {
            group = if new {
                to_set(line)
            } else {
                group.intersection(&to_set(line)).copied().collect()
            };
            new = false;
        }
    }
    total += group.len();
    println!("{}", total);
}

fn to_set(line: &str) -> HashSet<char> {
    line.chars().collect::<HashSet<_>>()
}
