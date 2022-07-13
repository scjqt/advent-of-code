use super::intcode;

pub fn part1(input: &[String]) -> impl ToString {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    computer.input(1);
    let mut last = 0;
    while let Some(value) = computer.output() {
        last = value;
    }
    last
}

pub fn part2(input: &[String]) -> impl ToString {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    computer.input(5);
    computer.output().unwrap()
}
