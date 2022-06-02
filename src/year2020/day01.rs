use std::collections::HashSet;

pub fn part1(input: &[String]) {
    if let Some(product) = find_pair(&parse(input), 2020) {
        println!("{}", product);
    }
}

pub fn part2(input: &[String]) {
    let values = parse(input);
    for (i, &value) in values.iter().enumerate() {
        if let Some(product) = find_pair(&values[i + 1..], 2020 - value) {
            println!("{}", product * value);
            break;
        }
    }
}

fn parse(input: &[String]) -> Vec<i32> {
    input.iter().map(|x| x.parse().unwrap()).collect()
}

fn find_pair(input: &[i32], sum: i32) -> Option<i32> {
    let mut values = HashSet::new();
    for value in input {
        if values.contains(value) {
            return Some(value * (sum - value));
        } else {
            values.insert(sum - value);
        }
    }
    None
}
