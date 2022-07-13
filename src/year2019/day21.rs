use super::intcode;

pub fn part1(input: &[String]) -> impl ToString {
    let instructions = ["OR D J", "OR A T", "AND C T", "NOT T T", "AND T J", "WALK"];
    springbot(input, &instructions)
}

pub fn part2(input: &[String]) -> impl ToString {
    let instructions = [
        "OR E J", "OR H J", "AND D J", "OR A T", "AND C T", "AND B T", "NOT T T", "AND T J", "RUN",
    ];
    springbot(input, &instructions)
}

fn springbot(input: &[String], instructions: &[&str]) -> i64 {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    while computer.output().is_some() {}
    for instruction in instructions {
        for c in instruction.chars() {
            computer.input((c as u8) as i64);
        }
        computer.input(10);
    }
    while let Some(value) = computer.output() {
        if value >= 1 << 8 {
            return value;
        }
    }
    panic!();
}
