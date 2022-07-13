use array2d::Array2D;
use search::{bft, dijkstra};
use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) -> u16 {
    let (map, start, keys, ids) = parse(input);
    let goal = dijkstra(
        Vault::new(vec![start], &keys),
        |v| v.adjacent(&map, &keys, &ids),
        |v| (v.id, v.robots[0]),
        |v| v.steps,
    )
    .find(|v| v.complete(keys.len()))
    .unwrap();
    goal.steps
}

pub fn part2(input: &[String]) -> u16 {
    const WALL_OFFSETS: [(isize, isize); 5] = [(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)];
    const POS_OFFSETS: [(isize, isize); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];
    let (mut map, start, keys, ids) = parse(input);
    for offset in WALL_OFFSETS {
        let pos = map.offset(start, offset).unwrap();
        map[pos] = true;
    }
    let mut robots = Vec::new();
    for offset in POS_OFFSETS {
        robots.push(map.offset(start, offset).unwrap());
    }
    let goal = dijkstra(
        Vault::new(robots, &keys),
        |v| v.adjacent(&map, &keys, &ids),
        |v| (v.id, v.robots[0], v.robots[1], v.robots[2], v.robots[3]),
        |v| v.steps,
    )
    .find(|v| v.complete(keys.len()))
    .unwrap();
    goal.steps
}

fn parse(
    input: &[String],
) -> (
    Array2D<bool>,
    (usize, usize),
    HashMap<(usize, usize), Option<(usize, usize)>>,
    HashMap<(usize, usize), u8>,
) {
    let mut map = Array2D::new(input[0].len(), input.len(), false);
    let mut start = (0, 0);
    let mut pos_iter = map.positions();
    let mut count = 0;
    let mut keys = HashMap::new();
    let mut ids = HashMap::new();
    let mut chars = HashMap::new();
    for line in input {
        for c in line.chars() {
            let pos = pos_iter.next().unwrap();
            match c {
                '.' => (),
                '#' => map[pos] = true,
                '@' => start = pos,
                _ => {
                    if c.is_ascii_lowercase() {
                        count += 1;
                        ids.insert(pos, count - 1);
                        if let Some(door) = chars.get(&c) {
                            keys.insert(pos, Some(*door));
                        } else {
                            keys.insert(pos, None);
                            chars.insert(c, pos);
                        }
                    } else {
                        let lower = c.to_ascii_lowercase();
                        if let Some(key) = chars.get(&lower) {
                            *keys.get_mut(&key).unwrap() = Some(pos);
                        } else {
                            chars.insert(lower, pos);
                        }
                    }
                }
            }
        }
    }
    (map, start, keys, ids)
}

#[derive(Clone)]
struct Vault {
    robots: Vec<(usize, usize)>,
    keys: HashSet<(usize, usize)>,
    doors: HashSet<(usize, usize)>,
    steps: u16,
    id: u32,
}

impl Vault {
    fn new(
        robots: Vec<(usize, usize)>,
        keys: &HashMap<(usize, usize), Option<(usize, usize)>>,
    ) -> Vault {
        Vault {
            robots,
            keys: keys.keys().copied().collect(),
            doors: keys.values().copied().flatten().collect(),
            steps: 0,
            id: 0,
        }
    }

    fn step(&self, map: &Array2D<bool>, robot: usize) -> Vec<Vault> {
        const OFFSETS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        if self.at_key(robot) {
            return Vec::new();
        }
        let mut vaults = Vec::new();
        for offset in OFFSETS {
            let adj = map.offset(self.robots[robot], offset).unwrap();
            if !map[adj] && !self.doors.contains(&adj) {
                let mut new = self.clone();
                new.robots[robot] = adj;
                new.steps += 1;
                vaults.push(new);
            }
        }
        vaults
    }

    fn adjacent(
        &self,
        map: &Array2D<bool>,
        keys: &HashMap<(usize, usize), Option<(usize, usize)>>,
        ids: &HashMap<(usize, usize), u8>,
    ) -> Vec<Vault> {
        let mut vaults = Vec::new();
        for robot in 0..self.robots.len() {
            for vault in bft(self.clone(), |v| v.step(map, robot), |v| v.robots[robot])
                .filter(|v| v.at_key(robot))
            {
                vaults.push(vault.unlock(keys, ids, robot));
            }
        }
        vaults
    }

    fn unlock(
        mut self,
        keys: &HashMap<(usize, usize), Option<(usize, usize)>>,
        ids: &HashMap<(usize, usize), u8>,
        robot: usize,
    ) -> Vault {
        self.keys.remove(&self.robots[robot]);
        if let Some(door) = keys[&self.robots[robot]] {
            self.doors.remove(&door);
        }
        self.id |= 1 << ids[&self.robots[robot]];
        self
    }

    fn at_key(&self, robot: usize) -> bool {
        self.keys.contains(&self.robots[robot])
    }

    fn complete(&self, keys: usize) -> bool {
        self.id == (1 << keys) - 1
    }
}
