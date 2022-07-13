use super::intcode;
use std::collections::{HashSet, VecDeque};

pub fn part1(input: &[String]) -> impl ToString {
    let mut network = Network::new(input);
    while network.nat == (-1, 0) {
        network.update();
    }
    network.nat.1
}

pub fn part2(input: &[String]) -> impl ToString {
    let mut network = Network::new(input);
    let mut history = HashSet::new();
    loop {
        if let Some(nat) = network.update() {
            if history.contains(&nat.1) {
                return nat.1;
            }
            history.insert(nat.1);
        }
    }
}

struct Network {
    computers: [intcode::Computer; 50],
    packets: [VecDeque<(i64, i64)>; 50],
    nat: (i64, i64),
}

impl Network {
    fn new(input: &[String]) -> Network {
        let mut computer = intcode::Computer::new(&input[0]).unwrap();
        computer.run();
        let mut computers = [(); 50].map(|_| computer.clone());
        for i in 0..50 {
            computers[i].input(i as i64);
        }
        Network {
            computers,
            packets: [(); 50].map(|_| VecDeque::new()),
            nat: (-1, 0),
        }
    }

    fn update(&mut self) -> Option<(i64, i64)> {
        let computers = &mut self.computers;
        let mut updated = false;
        for i in 0..50 {
            if computers[i].state() == intcode::State::Input {
                if let Some((x, y)) = self.packets[i].pop_front() {
                    updated = true;
                    computers[i].input(x);
                    computers[i].input(y);
                } else {
                    computers[i].input(-1);
                }
            } else if computers[i].state() == intcode::State::Output {
                let j = computers[i].output().unwrap() as usize;
                let packet = (
                    computers[i].output().unwrap(),
                    computers[i].output().unwrap(),
                );
                if j == 255 {
                    self.nat = packet;
                } else {
                    updated = true;
                    self.packets[j].push_back(packet);
                }
            }
        }
        if updated {
            return None;
        }
        self.packets[0].push_back(self.nat);
        Some(self.nat)
    }
}
