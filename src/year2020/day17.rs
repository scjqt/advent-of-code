use life::Life;

pub fn part1(input: &[String]) -> usize {
    let mut life = Life::new(input);
    for _ in 0..6 {
        life.update(false);
    }
    life.active()
}

pub fn part2(input: &[String]) -> usize {
    let mut life = Life::new(input);
    for _ in 0..6 {
        life.update(true);
    }
    life.active()
}

mod life {
    use std::collections::HashMap;
    use std::collections::HashSet;

    pub struct Life {
        state1: HashSet<(i32, i32, i32, i32)>,
        state2: HashSet<(i32, i32, i32, i32)>,
        state: bool,
    }

    impl Life {
        pub fn new(input: &[String]) -> Life {
            let mut state1 = HashSet::new();
            input.iter().enumerate().for_each(|(y, line)| {
                line.chars().enumerate().for_each(|(x, char)| {
                    if char == '#' {
                        state1.insert((x as i32, y as i32, 0, 0));
                    }
                })
            });
            Life {
                state1,
                state2: HashSet::new(),
                state: false,
            }
        }

        pub fn update(&mut self, hypercubes: bool) {
            let (active, inactive) = if self.state {
                self.state1.clear();
                (&self.state2, &mut self.state1)
            } else {
                self.state2.clear();
                (&self.state1, &mut self.state2)
            };
            let mut adjacent: HashMap<(i32, i32, i32, i32), u8> = HashMap::new();
            for (x, y, z, w) in active.iter() {
                for offz in -1..=1 {
                    for offy in -1..=1 {
                        for offx in -1..=1 {
                            for offw in if hypercubes { -1..=1 } else { 0..=0 } {
                                if offw != 0 || offz != 0 || offy != 0 || offx != 0 {
                                    *adjacent
                                        .entry((x + offx, y + offy, z + offz, w + offw))
                                        .or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }
            for (&(x, y, z, w), &count) in adjacent.iter() {
                if count == 3 || (count == 2 && active.contains(&(x, y, z, w))) {
                    inactive.insert((x, y, z, w));
                }
            }
            self.state = !self.state;
        }

        pub fn active(&self) -> usize {
            if self.state {
                self.state2.len()
            } else {
                self.state1.len()
            }
        }
    }
}
