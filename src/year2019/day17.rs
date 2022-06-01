use super::intcode;
use array2d::Array2D;
use vector::Vector;

pub fn part1(input: &[String]) {
    let view = parse_input(&input[0]);
    let mut total = 0;
    for y in 1..view.height() - 1 {
        for x in 1..view.width() - 1 {
            if view[[x + 0, y + 0]] == '#'
                && view[[x + 1, y + 0]] == '#'
                && view[[x - 1, y + 0]] == '#'
                && view[[x + 0, y + 1]] == '#'
                && view[[x + 0, y - 1]] == '#'
            {
                total += x * y;
            }
        }
    }
    println!("{}", total);
}

pub fn part2(input: &[String]) {
    let mut main_routine = get_instructions(parse_input(&input[0]));

    let functions = vec![
        extract_function(&mut main_routine, 'A'),
        extract_function(&mut main_routine, 'B'),
        extract_function(&mut main_routine, 'C'),
    ];

    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.set_direct(0, 2);
    computer.run();

    while computer.output().is_some() {}

    for (i, part) in main_routine.iter().enumerate() {
        if let Part::Function(letter) = part {
            computer.input((*letter as u8) as i64);
            computer.input(if i == main_routine.len() - 1 { 10 } else { 44 });
        } else {
            panic!();
        }
    }

    for function in functions.iter() {
        while computer.output().is_some() {}
        for (i, part) in function.iter().enumerate() {
            if let Part::Instruction(instruction) = part {
                computer.input(
                    (format!("{:?}", instruction.turn).chars().next().unwrap() as u8) as i64,
                );
                computer.input(44);
                for digit in instruction.distance.to_string().chars() {
                    computer.input((digit as u8) as i64);
                }
                computer.input(if i == function.len() - 1 { 10 } else { 44 });
            } else {
                panic!();
            }
        }
    }

    while computer.output().is_some() {}

    computer.input(('n' as u8) as i64);
    computer.input(10);

    let mut dust = 0;
    while let Some(output) = computer.output() {
        dust = output;
    }
    println!("{}", dust);
}

fn parse_input(input: &str) -> Array2D<char> {
    let mut computer = intcode::Computer::new(input).unwrap();
    computer.run();
    let mut view = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut x = 0;
    while let Some(value) = computer.output() {
        match value {
            10 => {
                if x > 1 {
                    height += 1;
                }
                if width == 0 {
                    width = x;
                }
                x = 0;
            }
            c => view.push(c as u8),
        }
        x += 1;
    }
    Array2D::from_fn(width, height, |(x, y)| view[x + y * width] as char)
}

fn get_instructions(view: Array2D<char>) -> Vec<Part> {
    let mut position = Vector::new(0, 0);
    let mut direction = Vector::new(0, 0);
    for ((x, y), &value) in view.iter_positions() {
        if value != '#' && value != '.' {
            position = Vector::new(x as isize, y as isize);
            direction = match value {
                '^' => Vector::new(0, -1),
                'v' => Vector::new(0, 1),
                '<' => Vector::new(-1, 0),
                '>' => Vector::new(1, 0),
                _ => panic!(),
            };
            break;
        }
    }

    let mut instructions = Vec::new();
    loop {
        let turn;
        let rotated = direction.rotated();
        if view.get((position + rotated).index()) == Some(&'#') {
            turn = Turn::R;
            direction = rotated;
        } else if view.get((position - rotated).index()) == Some(&'#') {
            turn = Turn::L;
            direction = -rotated;
        } else {
            break;
        }

        let mut distance = 0;
        while view.get((position + direction).index()) == Some(&'#') {
            position = position + direction;
            distance += 1;
        }

        instructions.push(Part::Instruction(Instruction { turn, distance }));
    }

    instructions
}

fn extract_function(main_routine: &mut Vec<Part>, letter: char) -> Vec<Part> {
    let mut i = 0;
    while let Part::Function(_) = main_routine[i] {
        i += 1;
    }
    let mut size = 2;
    let mut highest_coverage = 0;
    let mut indices = Vec::new();
    while let Some(Part::Instruction(_)) = main_routine.get(i + size) {
        size += 1;
        let current_indices: Vec<usize> = main_routine
            .windows(size)
            .enumerate()
            .filter(|&(_, f)| f == &main_routine[i..i + size])
            .map(|(i, _)| i)
            .rev()
            .collect();
        let current_coverage = current_indices.len() * size;
        if current_coverage > highest_coverage {
            highest_coverage = current_coverage;
            indices = current_indices;
        } else {
            size -= 1;
            break;
        }
    }

    let function = main_routine[i..i + size].to_vec();

    for i in indices.into_iter() {
        main_routine.drain(i + 1..i + size);
        main_routine[i] = Part::Function(letter);
    }

    function
}

mod vector {
    use std::ops::{Add, Neg, Sub};

    #[derive(Clone, Copy)]
    pub struct Vector {
        x: isize,
        y: isize,
    }

    impl Vector {
        pub fn new(x: isize, y: isize) -> Vector {
            Vector { x, y }
        }

        pub fn rotated(&self) -> Vector {
            Vector {
                x: -self.y,
                y: self.x,
            }
        }

        pub fn index(&self) -> (usize, usize) {
            (self.x as usize, self.y as usize)
        }
    }

    impl Add for Vector {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Vector {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Sub for Vector {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Vector {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl Neg for Vector {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Vector {
                x: -self.x,
                y: -self.y,
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Part {
    Function(char),
    Instruction(Instruction),
}

#[derive(PartialEq, Eq, Clone)]
struct Instruction {
    turn: Turn,
    distance: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Turn {
    R,
    L,
}
