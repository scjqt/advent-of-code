use state::State;
use Instruction::*;

pub fn part1(input: &[String]) {
    let instructions: Vec<Instruction> = input.iter().map(|x| interpret(&x)).collect();
    let mut state = State::new();
    state.advance_until_end(&instructions, -1);
    println!("{}", state.accumulator());
}

pub fn part2(input: &[String]) {
    let instructions: Vec<Instruction> = input.iter().map(|x| interpret(&x)).collect();
    for i in 0..instructions.len() {
        if let Jmp(_) | Nop(_) = instructions[i] {
            let mut state = State::new();
            state.advance_until_end(&instructions, i as i32);
            if state.index() == instructions.len() as i32 {
                println!("{}", state.accumulator());
                break;
            }
        }
    }
}

pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn interpret(line: &str) -> Instruction {
    let val: i32 = (&line[4..]).parse().unwrap();
    match &line[..3] {
        "acc" => Acc(val),
        "jmp" => Jmp(val),
        "nop" => Nop(val),
        _ => panic!(),
    }
}

mod state {
    use super::Instruction::{self, *};
    use std::collections::HashSet;

    pub struct State {
        index: i32,
        accumulator: i32,
        visited: HashSet<i32>,
    }

    impl State {
        pub fn new() -> State {
            State {
                index: 0,
                accumulator: 0,
                visited: HashSet::new(),
            }
        }

        pub fn advance(&mut self, instruction: &Instruction) {
            match instruction {
                Acc(val) => {
                    self.accumulator += val;
                    self.index += 1;
                }
                Jmp(val) => self.index += val,
                Nop(_) => self.index += 1,
            }
        }

        pub fn advance_until_end(&mut self, instructions: &Vec<Instruction>, swap: i32) {
            while !self.visited.contains(&self.index)
                && self.index >= 0
                && self.index < instructions.len() as i32
            {
                self.visited.insert(self.index);
                if self.index == swap {
                    let instruction = match instructions[self.index as usize] {
                        Acc(_) => panic!(),
                        Jmp(val) => Nop(val),
                        Nop(val) => Jmp(val),
                    };
                    self.advance(&instruction);
                } else {
                    self.advance(&instructions[self.index as usize]);
                }
            }
        }

        pub fn accumulator(&self) -> i32 {
            self.accumulator
        }

        pub fn index(&self) -> i32 {
            self.index
        }
    }
}
