use super::intcode;

pub fn part1(input: &[String]) {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    computer.input(1);
    while let Some(value) = computer.output() {
        println!("{}", value);
    }
}

pub fn part2(input: &[String]) {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    computer.input(2);
    println!("{}", computer.output().unwrap());
}
