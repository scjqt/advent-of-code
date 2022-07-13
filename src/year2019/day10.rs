use asteroids::{Asteroid, AsteroidOrd};
use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) -> usize {
    let map = parse_input(input);
    best(&map).0
}

pub fn part2(input: &[String]) -> i32 {
    let mut map = parse_input(input);
    let station = best(&map).1;
    map.remove(map.iter().position(|&x| x == station).unwrap());
    let result = asteroids_ordered(&map, station)[199].position();
    result.1 * 100 + result.0
}

fn parse_input(input: &[String]) -> Vec<(i32, i32)> {
    let mut map = Vec::new();
    for row in 0..input.len() {
        for column in 0..input[0].len() {
            if &input[row][column..=column] == "#" {
                map.push((row as i32, column as i32));
            }
        }
    }
    map
}

fn best(map: &Vec<(i32, i32)>) -> (usize, (i32, i32)) {
    let mut largest = 0;
    let mut best = (0, 0);
    for asteroid in map {
        let offsets: HashSet<(i32, i32)> = map
            .iter()
            .map(|&(r, c)| simplify(r - asteroid.0, c - asteroid.1).0)
            .collect();
        if offsets.len() > largest {
            largest = offsets.len();
            best = *asteroid;
        }
    }
    (largest - 1, best)
}

fn asteroids_ordered(map: &Vec<(i32, i32)>, station: (i32, i32)) -> Vec<AsteroidOrd> {
    let mut asteroids: Vec<Asteroid> = Vec::new();
    let mut lines = HashMap::new();
    for (i, &position) in map.iter().enumerate() {
        let mut asteroid = Asteroid::new(position, station);
        let indices = lines
            .entry(asteroid.offset())
            .or_insert(Vec::<usize>::new());
        for &j in indices.iter() {
            if asteroid.gcd() > asteroids[j].gcd() {
                let new = asteroids[j].pass + 1;
                if new > asteroid.pass {
                    asteroid.pass = new;
                }
            } else {
                asteroids[j].pass += 1;
            }
        }
        indices.push(i);
        asteroids.push(asteroid);
    }
    let mut ordered: Vec<AsteroidOrd> = asteroids
        .iter()
        .map(|asteroid| AsteroidOrd::new(asteroid))
        .collect();
    ordered.sort_unstable();
    ordered
}

fn simplify(i: i32, j: i32) -> ((i32, i32), i32) {
    let gcd = gcd(i.abs(), j.abs());
    if gcd == 0 {
        return ((0, 0), 0);
    }
    ((i / gcd, j / gcd), gcd)
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

mod asteroids {
    pub struct Asteroid {
        position: (i32, i32),
        offset: (i32, i32),
        gcd: i32,
        pub pass: u32,
    }

    impl Asteroid {
        pub fn new(position: (i32, i32), station: (i32, i32)) -> Asteroid {
            let (offset, gcd) = super::simplify(position.0 - station.0, position.1 - station.1);
            Asteroid {
                position,
                offset,
                gcd,
                pass: 0,
            }
        }

        pub fn offset(&self) -> (i32, i32) {
            self.offset
        }

        pub fn gcd(&self) -> i32 {
            self.gcd
        }
    }

    #[derive(Eq)]
    pub struct AsteroidOrd {
        position: (i32, i32),
        pass: u32,
        quadrant: u8,
        i: u32,
        j: u32,
    }

    impl AsteroidOrd {
        pub fn new(asteroid: &Asteroid) -> AsteroidOrd {
            let (quadrant, i, j) = match asteroid.offset {
                (r, c) if r < 0 && c >= 0 => (0, c, -r),
                (r, c) if r >= 0 && c > 0 => (1, r, c),
                (r, c) if r > 0 && c <= 0 => (2, -c, r),
                (r, c) if r <= 0 && c < 0 => (3, -r, -c),
                _ => panic!(),
            };
            AsteroidOrd {
                position: asteroid.position,
                pass: asteroid.pass,
                quadrant,
                i: i as u32,
                j: j as u32,
            }
        }

        pub fn position(&self) -> (i32, i32) {
            self.position
        }
    }

    impl PartialEq for AsteroidOrd {
        fn eq(&self, other: &Self) -> bool {
            self.position == other.position
        }
    }

    impl PartialOrd for AsteroidOrd {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for AsteroidOrd {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.pass
                .cmp(&other.pass)
                .then(self.quadrant.cmp(&other.quadrant))
                .then((self.i * other.j).cmp(&(other.i * self.j)))
        }
    }
}
