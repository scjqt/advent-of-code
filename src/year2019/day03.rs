use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) {
    let mut closest = i32::MAX;
    for intersection in set(&wire(&input[0]))
        .intersection(&set(&wire(&input[1])))
        .map(|(x, y)| x.abs() + y.abs())
    {
        if intersection < closest {
            closest = intersection;
        }
    }
    println!("{}", closest);
}

pub fn part2(input: &[String]) {
    let wires = (wire(&input[0]), wire(&input[1]));
    let mut closest = usize::MAX;
    for intersection in set(&wires.0)
        .intersection(&set(&wires.1))
        .map(|pos| wires.0.get(pos).unwrap() + wires.1.get(pos).unwrap())
    {
        if intersection < closest {
            closest = intersection;
        }
    }
    println!("{}", closest);
}

fn wire(path: &str) -> HashMap<(i32, i32), usize> {
    let (mut x, mut y) = (0, 0);
    let mut distance = 0;
    let mut map = HashMap::new();
    for instruction in path.split(',') {
        let (offx, offy) = match &instruction[0..1] {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!(),
        };
        for _ in 0..instruction[1..].parse::<usize>().unwrap() {
            x += offx;
            y += offy;
            distance += 1;
            map.insert((x, y), distance);
        }
    }
    map
}

fn set(map: &HashMap<(i32, i32), usize>) -> HashSet<(i32, i32)> {
    map.keys().cloned().collect()
}
