use std::collections::{HashMap, HashSet};
const OFFSETS: [(i8, i8); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

pub fn part1(input: &[String]) -> u32 {
    let mut bugs = parse(input);
    let mut rating = biodiversity(&bugs);
    let mut history = HashSet::new();
    while !history.contains(&rating) {
        history.insert(rating);
        bugs = update(bugs, adj);
        rating = biodiversity(&bugs);
    }
    rating
}

pub fn part2(input: &[String]) -> usize {
    let mut bugs = parse(input);
    for _ in 0..200 {
        bugs = update(bugs, level_adj);
    }
    bugs.len()
}

fn parse(input: &[String]) -> HashSet<Bug> {
    let mut bugs = HashSet::new();
    let mut y = -2;
    for line in input {
        let mut x = -2;
        for c in line.chars() {
            if c == '#' {
                bugs.insert(Bug::new(x, y, 0));
            }
            x += 1;
        }
        y += 1;
    }
    bugs
}

fn update<F, I>(bugs: HashSet<Bug>, mut adjacent: F) -> HashSet<Bug>
where
    F: FnMut(&Bug) -> I,
    I: IntoIterator<Item = Bug>,
{
    let mut counts = HashMap::new();
    for bug in &bugs {
        for adj in adjacent(bug) {
            *counts.entry(adj).or_insert(0) += 1;
        }
    }
    let mut new = HashSet::new();
    for (bug, count) in counts.into_iter() {
        if count == 1 || (count == 2 && !bugs.contains(&bug)) {
            new.insert(bug);
        }
    }
    new
}

fn adj(bug: &Bug) -> Vec<Bug> {
    let mut bugs = Vec::new();
    for (x, y) in OFFSETS {
        let (nx, ny) = (bug.x + x, bug.y + y);
        if nx.abs() != 3 && ny.abs() != 3 {
            bugs.push(Bug::new(nx, ny, 0));
        }
    }
    bugs
}

fn level_adj(bug: &Bug) -> Vec<Bug> {
    let mut bugs = Vec::new();
    for (x, y) in OFFSETS {
        let (nx, ny) = (bug.x + x, bug.y + y);
        if nx.abs() == 3 {
            bugs.push(Bug::new(nx / nx.abs(), 0, bug.level - 1));
        } else if ny.abs() == 3 {
            bugs.push(Bug::new(0, ny / ny.abs(), bug.level - 1));
        } else if (nx, ny) == (0, 0) {
            for z in -2..=2 {
                if bug.x == 0 {
                    bugs.push(Bug::new(z, bug.y * 2, bug.level + 1));
                } else {
                    bugs.push(Bug::new(bug.x * 2, z, bug.level + 1));
                }
            }
        } else {
            bugs.push(Bug::new(nx, ny, bug.level));
        }
    }
    bugs
}

fn biodiversity(bugs: &HashSet<Bug>) -> u32 {
    let mut rating = 0;
    for bug in bugs {
        rating |= bug.rating();
    }
    rating
}

#[derive(PartialEq, Eq, Hash)]
struct Bug {
    x: i8,
    y: i8,
    level: i8,
}

impl Bug {
    fn new(x: i8, y: i8, level: i8) -> Bug {
        Bug { x, y, level }
    }

    fn rating(&self) -> u32 {
        1 << (self.x + 2 + 5 * (self.y + 2))
    }
}
