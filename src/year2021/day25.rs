use array2d::Array2D;

pub fn part1(input: &[String]) -> impl ToString {
    let mut map = parse(input);
    let mut count = 0;
    let mut updated = true;
    while updated {
        (map, updated) = step(map);
        count += 1;
    }
    count
}

fn step(map: Array2D<Cucumber>) -> (Array2D<Cucumber>, bool) {
    let mut new = Array2D::new(map.width(), map.height(), Cucumber::Empty);
    let mut updated = false;
    for ((x, y), cucumber) in map.iter_positions() {
        if cucumber == &Cucumber::East {
            let moved = ((x + 1) % map.width(), y);
            if map[moved] == Cucumber::Empty {
                new[moved] = Cucumber::East;
                updated = true;
            } else {
                new[(x, y)] = Cucumber::East;
            }
        }
    }
    for ((x, y), cucumber) in map.iter_positions() {
        if cucumber == &Cucumber::South {
            let moved = (x, (y + 1) % map.height());
            if map[moved] == Cucumber::South || new[moved] == Cucumber::East {
                new[(x, y)] = Cucumber::South;
            } else {
                new[moved] = Cucumber::South;
                updated = true;
            }
        }
    }
    (new, updated)
}

fn parse(input: &[String]) -> Array2D<Cucumber> {
    let mut map = Array2D::new(input[0].len(), input.len(), Cucumber::Empty);
    let mut pos = map.positions();
    for line in input {
        for c in line.chars() {
            map[pos.next().unwrap()] = match c {
                '.' => Cucumber::Empty,
                '>' => Cucumber::East,
                'v' => Cucumber::South,
                _ => panic!(),
            };
        }
    }
    map
}

#[derive(PartialEq, Eq, Clone)]
enum Cucumber {
    Empty,
    East,
    South,
}
