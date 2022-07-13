use game::Game;

pub fn part1(input: &[String]) -> impl ToString {
    let mut game = Game::new(
        input[0]
            .chars()
            .map(|x| x.to_string().parse().unwrap())
            .collect(),
    );
    for _ in 0..100 {
        game.update(3);
    }
    game.result_one()
}

pub fn part2(input: &[String]) -> impl ToString {
    let mut values: Vec<usize> = input[0]
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();
    for x in values.len() + 1..=1000000 {
        values.push(x);
    }
    let mut game = Game::new(values);
    for _ in 0..10000000 {
        game.update(3);
    }
    game.result_two()
}

mod game {
    pub struct Game {
        cups: Vec<usize>,
        current: usize,
    }

    impl Game {
        pub fn new(values: Vec<usize>) -> Game {
            let mut cups = Vec::new();
            cups.resize_with(values.len() + 1, || 0);
            for i in 0..values.len() - 1 {
                cups[values[i]] = values[i + 1];
            }
            cups[values[values.len() - 1]] = values[0];
            Game {
                cups,
                current: values[0],
            }
        }

        pub fn update(&mut self, n: usize) {
            let mut to_move = vec![self.cups[self.current]];
            for i in 0..n - 1 {
                to_move.push(self.cups[to_move[i]]);
            }
            let mut destination = if self.current == 1 {
                self.cups.len() - 1
            } else {
                self.current - 1
            };
            while to_move.contains(&destination) {
                destination = if destination == 1 {
                    self.cups.len() - 1
                } else {
                    destination - 1
                };
            }
            self.cups[self.current] = self.cups[to_move[n - 1]];
            self.cups[to_move[n - 1]] = self.cups[destination];
            self.cups[destination] = to_move[0];
            self.current = self.cups[self.current];
        }

        pub fn result_one(&self) -> String {
            let mut result = String::new();
            let mut current = 1;
            for _ in 0..self.cups.len() - 2 {
                current = self.cups[current];
                result += &current.to_string();
            }
            result
        }

        pub fn result_two(&self) -> usize {
            self.cups[1] * self.cups[self.cups[1]]
        }
    }
}
