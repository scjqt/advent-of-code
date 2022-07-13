use super::intcode;
use std::collections::{HashMap, HashSet, VecDeque};
use vector::Vector;

pub fn part1(input: &[String]) -> usize {
    let (oxygen, distances) = map_area(&input[0]);
    distances[&oxygen]
}

pub fn part2(input: &[String]) -> usize {
    let (oxygen, distances) = map_area(&input[0]);
    let area: HashSet<Vector> = distances.keys().cloned().collect();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(oxygen);
    visited.insert(oxygen);
    let mut time = 0;

    while queue.len() > 0 {
        let length = queue.len();
        for _ in 0..length {
            let current = queue.pop_front().unwrap();
            for &offset in Vector::all_directions().iter() {
                let new = current + offset;
                if !visited.contains(&new) && area.contains(&new) {
                    visited.insert(new);
                    queue.push_back(new);
                }
            }
        }
        if queue.len() > 0 {
            time += 1;
        }
    }

    time
}

fn map_area(input: &str) -> (Vector, HashMap<Vector, usize>) {
    let mut computer = intcode::Computer::new(input).unwrap();
    computer.run();
    let mut distances = HashMap::new();
    let mut unchecked = HashMap::new();
    let mut walls = HashSet::new();
    let start = Vector::new(0, 0);
    let mut stack = vec![start];
    let mut oxygen = start;
    distances.insert(start, 0);
    unchecked.insert(start, Vector::all_directions());

    'outer: while stack.len() > 0 {
        let current_pos = *stack.last().unwrap();
        let current_dist = distances[&current_pos];
        while let Some(movement) = unchecked.get_mut(&current_pos).unwrap().pop() {
            let new_pos = current_pos + movement;
            if let Some(distance) = distances.get_mut(&new_pos) {
                if *distance > current_dist + 1 {
                    unchecked.insert(new_pos, Vector::all_directions());
                    *distance = current_dist + 1;
                    stack.push(new_pos);
                    computer.input(movement.as_input().unwrap());
                    computer.output().unwrap();
                    continue 'outer;
                }
            } else if !walls.contains(&new_pos) {
                computer.input(movement.as_input().unwrap());
                let status = computer.output().unwrap();
                if status == 2 {
                    oxygen = new_pos;
                }
                if status > 0 {
                    unchecked.insert(new_pos, Vector::all_directions());
                    distances.insert(new_pos, current_dist + 1);
                    stack.push(new_pos);
                    continue 'outer;
                } else {
                    walls.insert(new_pos);
                }
            }
        }
        let current_pos = stack.pop().unwrap();
        if let Some(&new_pos) = stack.last() {
            computer.input((new_pos - current_pos).as_input().unwrap());
            computer.output().unwrap();
        }
    }

    (oxygen, distances)
}

mod vector {
    use std::ops::{Add, Sub};

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct Vector {
        x: i8,
        y: i8,
    }

    impl Vector {
        pub fn new(x: i8, y: i8) -> Vector {
            Vector { x, y }
        }

        pub fn as_input(&self) -> Option<i64> {
            Some(match (self.x, self.y) {
                (0, 1) => 1,
                (0, -1) => 2,
                (-1, 0) => 3,
                (1, 0) => 4,
                _ => return None,
            })
        }

        pub fn all_directions() -> Vec<Vector> {
            vec![
                Vector::new(0, 1),
                Vector::new(0, -1),
                Vector::new(-1, 0),
                Vector::new(1, 0),
            ]
        }
    }

    impl Add for Vector {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Vector {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Sub for Vector {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            Vector {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }
}
