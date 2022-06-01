use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) {
    println!("{}", flip(input).len());
}

pub fn part2(input: &[String]) {
    let mut tiles = flip(input);
    let offsets = &[(1, 0), (-1, 0), (0, 1), (0, -1), (1, -1), (-1, 1)];
    for _ in 0..100 {
        let mut adjacent = HashMap::new();
        for &(x, y) in &tiles {
            for &(offx, offy) in offsets {
                let count = adjacent.entry((x + offx, y + offy)).or_insert(0);
                *count += 1;
            }
        }
        let mut new = HashSet::new();
        for tile in &tiles {
            if let Some(&count) = adjacent.get(tile) {
                if count == 1 || count == 2 {
                    new.insert(*tile);
                }
            }
        }
        for (&tile, &count) in adjacent.iter() {
            if count == 2 {
                new.insert(tile);
            }
        }
        tiles = new;
    }
    println!("{}", tiles.len());
}

fn flip(input: &[String]) -> HashSet<(i32, i32)> {
    let mut flipped = HashSet::new();
    for line in input {
        let (mut x, mut y) = (0, 0);
        for (offx, offy) in parse(line) {
            x += offx;
            y += offy;
        }
        if flipped.contains(&(x, y)) {
            flipped.remove(&(x, y));
        } else {
            flipped.insert((x, y));
        }
    }
    flipped
}

fn parse(line: &str) -> Vec<(i32, i32)> {
    let mut i = 0;
    let mut parsed = Vec::new();
    while i < line.len() {
        parsed.push(match &line[i..=i] {
            "n" | "s" => {
                i += 1;
                match &line[i - 1..=i] {
                    "ne" => (0, 1),
                    "nw" => (-1, 1),
                    "se" => (1, -1),
                    "sw" => (0, -1),
                    _ => panic!(),
                }
            }
            "e" => (1, 0),
            "w" => (-1, 0),
            _ => panic!(),
        });
        i += 1;
    }
    parsed
}
