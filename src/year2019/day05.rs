use super::intcode;

pub fn part1(input: &[String]) {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    computer.input(1);
    let mut last = 0;
    while let Some(value) = computer.output() {
        last = value;
    }
    println!("{}", last);
}

pub fn part2(input: &[String]) {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    computer.input(5);
    println!("{}", computer.output().unwrap());
}
