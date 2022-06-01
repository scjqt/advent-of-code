use array2d::Array2D;

pub fn part1(input: &[String]) {
    let mut cave = Cave::new(input);
    for _ in 0..100 {
        cave.step();
    }
    println!("{}", cave.total_flashes);
}

pub fn part2(input: &[String]) {
    let mut cave = Cave::new(input);
    while cave.step_flashes != 100 {
        cave.step();
    }
    println!("{}", cave.step);
}

struct Cave {
    grid: Array2D<u8>,
    step: u16,
    total_flashes: u16,
    step_flashes: u16,
}

impl Cave {
    fn new(input: &[String]) -> Cave {
        let mut grid = Array2D::new(10, 10, 0);
        let mut pos = grid.positions();
        for line in input {
            for c in line.chars() {
                grid[pos.next().unwrap()] = c.to_string().parse().unwrap();
            }
        }
        Cave {
            grid,
            step: 0,
            total_flashes: 0,
            step_flashes: 0,
        }
    }

    fn step(&mut self) {
        self.step += 1;

        for pos in self.grid.positions() {
            self.grid[pos] += 1;
            if self.grid[pos] == 10 {
                self.increment_adj(pos);
            }
        }

        self.step_flashes = 0;
        for level in self.grid.iter_mut() {
            if *level > 9 {
                self.step_flashes += 1;
                *level = 0;
            }
        }

        self.total_flashes += self.step_flashes;
    }

    fn increment_adj(&mut self, pos: (usize, usize)) {
        const OFFSETS: [(isize, isize); 8] = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        for offset in OFFSETS {
            if let Some(adj) = self.grid.offset(pos, offset) {
                self.grid[adj] += 1;
                if self.grid[adj] == 10 {
                    self.increment_adj(adj)
                }
            }
        }
    }
}
