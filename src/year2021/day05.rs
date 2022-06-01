use std::{collections::HashMap, str::Split};

pub fn part1(input: &[String]) {
    solve(input, false);
}

pub fn part2(input: &[String]) {
    solve(input, true);
}

fn parse(line: &str) -> ((i16, i16), (i16, i16)) {
    let mut ends = line.split(" -> ");
    let one = parse_coords(ends.next().unwrap().split(','));
    let two = parse_coords(ends.next().unwrap().split(','));
    (one, two)
}

fn parse_coords(mut coords: Split<char>) -> (i16, i16) {
    (
        coords.next().unwrap().parse().unwrap(),
        coords.next().unwrap().parse().unwrap(),
    )
}

fn solve(input: &[String], part: bool) {
    let mut vents = HashMap::new();
    for vent in input {
        let ((x1, y1), (x2, y2)) = parse(vent);
        let dx = direction(x1, x2);
        let dy = direction(y1, y2);
        if part || dx == 0 || dy == 0 {
            let (mut x, mut y) = (x1 - dx, y1 - dy);
            while (x, y) != (x2, y2) {
                x += dx;
                y += dy;
                let overlaps = vents.entry((x, y)).or_insert(0);
                *overlaps += 1;
            }
        }
    }
    println!(
        "{}",
        vents.values().filter(|overlaps| **overlaps > 1).count()
    );
}

fn direction(v1: i16, v2: i16) -> i16 {
    match v2 - v1 {
        0 => 0,
        d => d / d.abs(),
    }
}
