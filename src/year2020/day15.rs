use memory_game::Game;

pub fn part1(input: &[String]) -> impl ToString {
    let mut game = Game::new(input[0].split(',').map(|x| x.parse().unwrap()).collect());
    game.update_until(2020)
}

pub fn part2(input: &[String]) -> impl ToString {
    let mut game = Game::new(input[0].split(',').map(|x| x.parse().unwrap()).collect());
    game.update_until(30000000)
}

mod memory_game {
    use std::collections::HashMap;

    pub struct Game {
        memory: HashMap<u32, u32>,
        last: u32,
        count: u32,
    }

    impl Game {
        pub fn new(starting: Vec<u32>) -> Game {
            let mut memory = HashMap::new();
            let mut last = 0;
            for (i, val) in starting.iter().enumerate() {
                if i == starting.len() - 1 {
                    last = *val;
                } else {
                    memory.insert(*val, (i + 1) as u32);
                }
            }
            Game {
                memory,
                last,
                count: starting.len() as u32,
            }
        }

        pub fn update(&mut self) {
            let new = if self.memory.contains_key(&self.last) {
                self.count - self.memory.get(&self.last).unwrap()
            } else {
                0
            };
            self.memory.insert(self.last, self.count);
            self.last = new;
            self.count += 1;
        }

        pub fn update_until(&mut self, count: u32) -> u32 {
            while self.count < count {
                self.update();
            }
            self.last
        }
    }
}
