use super::intcode;

pub fn part1(input: &[String]) -> i64 {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    computer.input(1);
    while let Some(value) = computer.output() {
        return value;
    }
    panic!()
}

pub fn part2(input: &[String]) -> i64 {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    computer.input(2);
    computer.output().unwrap()
}
