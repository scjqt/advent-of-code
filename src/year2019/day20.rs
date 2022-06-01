use array2d::Array2D;
use search::bft;
use std::collections::HashMap;
const OFFSETS: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

pub fn part1(input: &[String]) {
    let (maze, portals, start, end) = parse(input);
    let goal = bft(
        State::start(start),
        |s| {
            let mut states = Vec::new();
            let steps = s.steps + 1;
            for offset in OFFSETS {
                let adj = maze.offset(s.pos, offset).unwrap();
                if maze[adj] == '.' {
                    states.push(State::new(adj, steps, 0));
                }
            }
            if let Some(&pos) = portals.get(&s.pos) {
                states.push(State::new(pos, steps, 0));
            }
            states
        },
        |s| s.pos,
    )
    .find(|s| s.pos == end)
    .unwrap();
    println!("{}", goal.steps);
}

pub fn part2(input: &[String]) {
    let (maze, portals, start, end) = parse(input);
    let goal = bft(
        State::start(start),
        |s| {
            let mut states = Vec::new();
            let steps = s.steps + 1;
            for offset in OFFSETS {
                let adj = maze.offset(s.pos, offset).unwrap();
                if maze[adj] == '.' {
                    states.push(State::new(adj, steps, s.layer));
                }
            }
            if let Some(&pos) = portals.get(&s.pos) {
                if outer_portal(s.pos, &maze) {
                    if s.layer > 0 {
                        states.push(State::new(pos, steps, s.layer - 1));
                    }
                } else {
                    states.push(State::new(pos, steps, s.layer + 1));
                }
            }
            states
        },
        |s| (s.pos, s.layer),
    )
    .find(|s| s.pos == end && s.layer == 0)
    .unwrap();
    println!("{}", goal.steps);
}

struct State {
    pos: (usize, usize),
    steps: u16,
    layer: u8,
}

impl State {
    fn start(start: (usize, usize)) -> State {
        State {
            pos: start,
            steps: 0,
            layer: 0,
        }
    }

    fn new(pos: (usize, usize), steps: u16, layer: u8) -> State {
        State { pos, steps, layer }
    }
}

fn outer_portal(pos: (usize, usize), maze: &Array2D<char>) -> bool {
    pos.0 == 2 || pos.1 == 2 || pos.0 == maze.width() - 3 || pos.1 == maze.height() - 3
}

fn parse(
    input: &[String],
) -> (
    Array2D<char>,
    HashMap<(usize, usize), (usize, usize)>,
    (usize, usize),
    (usize, usize),
) {
    let mut maze = Array2D::new(input[0].len(), input.len(), ' ');
    let mut pos_iter = maze.positions();
    let mut names = HashMap::new();
    let mut portals = HashMap::new();
    for line in input {
        for c in line.chars() {
            let (x, y) = pos_iter.next().unwrap();
            maze[(x, y)] = c;
            if c.is_ascii_alphabetic() {
                if y > 0 && maze[(x, y - 1)].is_ascii_alphabetic() {
                    if y > 1 && maze[(x, y - 2)] == '.' {
                        add_portal((x, y - 2), (maze[(x, y - 1)], c), &mut portals, &mut names);
                    } else {
                        add_portal((x, y + 1), (maze[(x, y - 1)], c), &mut portals, &mut names);
                    }
                } else if x > 0 && maze[(x - 1, y)].is_ascii_alphabetic() {
                    if x > 1 && maze[(x - 2, y)] == '.' {
                        add_portal((x - 2, y), (maze[(x - 1, y)], c), &mut portals, &mut names);
                    } else {
                        add_portal((x + 1, y), (maze[(x - 1, y)], c), &mut portals, &mut names);
                    }
                }
            }
        }
    }
    (maze, portals, names[&('A', 'A')], names[&('Z', 'Z')])
}

fn add_portal(
    pos: (usize, usize),
    name: (char, char),
    portals: &mut HashMap<(usize, usize), (usize, usize)>,
    names: &mut HashMap<(char, char), (usize, usize)>,
) {
    if let Some(&dest) = names.get(&name) {
        portals.insert(pos, dest);
        portals.insert(dest, pos);
    } else {
        names.insert(name, pos);
    }
}
