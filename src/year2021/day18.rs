use snailfish::Number;

pub fn part1(input: &[String]) {
    let mut number = Number::new(&input[0]);
    for n in input.iter().skip(1) {
        number = number + Number::new(n);
    }
    println!("{}", number.magnitude());
}

pub fn part2(input: &[String]) {
    let numbers: Vec<_> = input.iter().map(|line| Number::new(line)).collect();
    let mut largest = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                largest = largest.max((numbers[i].clone() + numbers[j].clone()).magnitude())
            }
        }
    }
    println!("{}", largest);
}

mod snailfish {
    use std::ops::Add;

    #[derive(Clone)]
    pub struct Number {
        values: Vec<Regular>,
    }

    impl Number {
        pub fn new(s: &str) -> Number {
            let mut values = Vec::new();
            let mut depth = 0;
            for c in s.chars() {
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' => (),
                    x => values.push(Regular::new(x.to_string().parse().unwrap(), depth)),
                }
            }
            Number { values }
        }

        fn reduce_into(mut self) -> Number {
            let v = &mut self.values;
            'outer: loop {
                for i in 0..v.len() - 1 {
                    if v[i].depth == 5 {
                        if i > 0 {
                            v[i - 1].value += v[i].value;
                        }
                        if i < v.len() - 2 {
                            v[i + 2].value += v[i + 1].value;
                        }

                        v[i] = Regular::new(0, 4);
                        v.remove(i + 1);

                        continue 'outer;
                    }
                }

                for i in 0..v.len() {
                    if v[i].value >= 10 {
                        v.insert(i + 1, Regular::new((v[i].value + 1) / 2, v[i].depth + 1));
                        v[i] = Regular::new(v[i].value / 2, v[i].depth + 1);

                        continue 'outer;
                    }
                }

                return self;
            }
        }

        pub fn magnitude(mut self) -> u16 {
            while self.values.len() > 1 {
                for i in 0..self.values.len() - 1 {
                    let left = &self.values[i];
                    let right = &self.values[i + 1];
                    if left.depth == right.depth {
                        self.values[i] =
                            Regular::new(3 * left.value + 2 * right.value, left.depth - 1);
                        self.values.remove(i + 1);
                        break;
                    }
                }
            }
            self.values[0].value
        }
    }

    impl Add for Number {
        type Output = Number;

        fn add(mut self, rhs: Self) -> Self::Output {
            self.values.extend(rhs.values);
            self.values.iter_mut().for_each(|x| x.depth += 1);
            self.reduce_into()
        }
    }

    #[derive(Clone)]
    struct Regular {
        value: u16,
        depth: u8,
    }

    impl Regular {
        fn new(value: u16, depth: u8) -> Regular {
            Regular { value, depth }
        }
    }
}
