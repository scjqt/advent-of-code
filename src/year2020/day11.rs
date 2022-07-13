use waiting_area::WaitingArea;

pub fn part1(input: &[String]) -> impl ToString {
    let mut state = WaitingArea::new(input);
    while state.update_part_one() {}
    state.occupied()
}

pub fn part2(input: &[String]) -> impl ToString {
    let mut state = WaitingArea::new(input);
    while state.update_part_two() {}
    state.occupied()
}

#[derive(Copy, Clone)]
enum State {
    Floor,
    Empty,
    Occupied,
}

mod waiting_area {
    use super::State::{self, *};
    use array2d::Array2D;
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

    pub struct WaitingArea {
        grid1: Array2D<State>,
        grid2: Array2D<State>,
        active: bool,
        occupied: u16,
    }

    impl WaitingArea {
        pub fn new(input: &[String]) -> WaitingArea {
            let width = input[0].len();
            let height = input.len();
            let mut grid1 = Array2D::new(width, height, Floor);
            let mut pos = grid1.positions();
            for line in input {
                for char in line.chars() {
                    grid1[pos.next().unwrap()] = match char {
                        'L' => Empty,
                        '.' => Floor,
                        _ => panic!(),
                    };
                }
            }
            WaitingArea {
                grid1,
                grid2: Array2D::new(width, height, Floor),
                active: false,
                occupied: 0,
            }
        }

        pub fn update_part_one(&mut self) -> bool {
            let mut altered = false;
            self.occupied = 0;
            let (active, inactive) = if self.active {
                (&self.grid2, &mut self.grid1)
            } else {
                (&self.grid1, &mut self.grid2)
            };
            for pos in active.positions() {
                let mut occupied = 0;
                for offset in OFFSETS {
                    if let Some(adj) = active.offset(pos, offset) {
                        if let Occupied = active[adj] {
                            occupied += 1;
                        }
                    }
                }
                inactive[pos] = match active[pos] {
                    Empty if occupied == 0 => {
                        altered = true;
                        Occupied
                    }
                    Occupied if occupied > 3 => {
                        altered = true;
                        Empty
                    }
                    other => other,
                };
                if let Occupied = inactive[pos] {
                    self.occupied += 1;
                }
            }
            self.active = !self.active;
            altered
        }

        pub fn update_part_two(&mut self) -> bool {
            let mut altered = false;
            self.occupied = 0;
            let (active, inactive) = if self.active {
                (&self.grid2, &mut self.grid1)
            } else {
                (&self.grid1, &mut self.grid2)
            };
            for pos in active.positions() {
                let mut occupied = 0;
                for offset in OFFSETS {
                    let mut current = pos;
                    while let Some(new) = active.offset(current, offset) {
                        current = new;
                        match active[current] {
                            Occupied => {
                                occupied += 1;
                                break;
                            }
                            Empty => break,
                            _ => (),
                        }
                    }
                }
                inactive[pos] = match active[pos] {
                    Empty if occupied == 0 => {
                        altered = true;
                        Occupied
                    }
                    Occupied if occupied > 4 => {
                        altered = true;
                        Empty
                    }
                    other => other,
                };
                if let Occupied = inactive[pos] {
                    self.occupied += 1;
                }
            }
            self.active = !self.active;
            altered
        }

        pub fn occupied(&self) -> u16 {
            self.occupied
        }
    }
}
