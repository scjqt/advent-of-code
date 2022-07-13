use ship::Ship;
use Instruction::*;

pub fn part1(input: &[String]) -> impl ToString {
    let mut ship = Ship::new(1, 0);
    input.iter().for_each(|x| ship.update(from_string(x)));
    ship.manhattan()
}

pub fn part2(input: &[String]) -> impl ToString {
    let mut ship = Ship::new(10, 1);
    input
        .iter()
        .for_each(|x| ship.update_with_waypoint(from_string(x)));
    ship.manhattan()
}

pub enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

fn from_string(data: &str) -> Instruction {
    let instruction = &data[0..1];
    let value: i32 = (&data[1..data.len()]).parse().unwrap();
    match instruction {
        "N" => North(value),
        "S" => South(value),
        "E" => East(value),
        "W" => West(value),
        "L" => Left(value),
        "R" => Right(value),
        "F" => Forward(value),
        _ => panic!(),
    }
}

mod ship {
    use super::Instruction::{self, *};

    pub struct Ship {
        x: i32,
        y: i32,
        dirx: i32,
        diry: i32,
    }

    impl Ship {
        pub fn new(dirx: i32, diry: i32) -> Ship {
            Ship {
                x: 0,
                y: 0,
                dirx,
                diry,
            }
        }

        pub fn update(&mut self, instruction: Instruction) {
            match instruction {
                North(val) => self.y += val,
                South(val) => self.y -= val,
                East(val) => self.x += val,
                West(val) => self.x -= val,
                Left(val) => self.rotate(val as usize),
                Right(val) => self.rotate(360 - val as usize),
                Forward(val) => {
                    self.x += val * self.dirx;
                    self.y += val * self.diry
                }
            }
        }

        pub fn update_with_waypoint(&mut self, instruction: Instruction) {
            match instruction {
                North(val) => self.diry += val,
                South(val) => self.diry -= val,
                East(val) => self.dirx += val,
                West(val) => self.dirx -= val,
                Left(val) => self.rotate(val as usize),
                Right(val) => self.rotate(360 - val as usize),
                Forward(val) => {
                    self.x += val * self.dirx;
                    self.y += val * self.diry
                }
            }
        }

        fn rotate(&mut self, angle: usize) {
            let matrix = [(0, -1, 1, 0), (-1, 0, 0, -1), (0, 1, -1, 0)][angle / 90 - 1];
            let old = (self.dirx, self.diry);
            self.dirx = old.0 * matrix.0 + old.1 * matrix.1;
            self.diry = old.0 * matrix.2 + old.1 * matrix.3;
        }

        pub fn manhattan(&self) -> i32 {
            i32::abs(self.x) + i32::abs(self.y)
        }
    }
}
