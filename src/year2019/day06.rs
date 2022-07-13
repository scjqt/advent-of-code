use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) -> usize {
    let orbits = orbits(input);
    let mut total = 0;
    for object in orbits.keys() {
        total += total_orbits(&orbits, object);
    }
    total
}

pub fn part2(input: &[String]) -> usize {
    let orbits = orbits(input);
    let you = full_path(&orbits, orbits["YOU"]);
    let san = full_path(&orbits, orbits["SAN"]);
    let min = you.len() + san.len() - you.intersection(&san).collect::<HashSet<_>>().len() * 2;
    min
}

fn total_orbits(orbits: &HashMap<&str, &str>, object: &str) -> usize {
    if object == "COM" {
        0
    } else {
        total_orbits(orbits, orbits[object]) + 1
    }
}

fn full_path<'a>(orbits: &'a HashMap<&str, &str>, mut object: &'a str) -> HashSet<&'a str> {
    let mut path = HashSet::new();
    while object != "COM" {
        path.insert(object);
        object = orbits[object];
    }
    path
}

fn orbits(input: &[String]) -> HashMap<&str, &str> {
    let mut orbits = HashMap::new();
    for orbit in input {
        orbits.insert(&orbit[4..7], &orbit[..3]);
    }
    orbits
}
