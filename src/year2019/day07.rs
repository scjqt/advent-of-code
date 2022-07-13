use super::intcode;
use itertools::Itertools;

pub fn part1(input: &[String]) -> i64 {
    let computer = intcode::Computer::new(&input[0]).unwrap();
    let mut largest = 0;
    for permutation in (0..5).permutations(5) {
        let mut amplifiers = Vec::new();
        amplifiers.resize_with(5, || computer.clone());
        let mut signal = 0;
        for i in 0..5 {
            amplifiers[i].run();
            amplifiers[i].input(permutation[i]);
            amplifiers[i].input(signal);
            signal = amplifiers[i].output().unwrap();
        }
        if signal > largest {
            largest = signal;
        }
    }
    largest
}

pub fn part2(input: &[String]) -> i64 {
    let computer = intcode::Computer::new(&input[0]).unwrap();
    let mut largest = 0;
    for permutation in (5..10).permutations(5) {
        let mut amplifiers = Vec::new();
        amplifiers.resize_with(5, || computer.clone());
        let mut signal = 0;
        for i in 0..5 {
            amplifiers[i].run();
            amplifiers[i].input(permutation[i]);
            amplifiers[i].input(signal);
            signal = amplifiers[i].output().unwrap();
        }
        while amplifiers[4].state() != intcode::State::Halted {
            for i in 0..5 {
                amplifiers[i].input(signal);
                signal = amplifiers[i].output().unwrap();
            }
        }
        if signal > largest {
            largest = signal;
        }
    }
    largest
}
