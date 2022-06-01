use super::intcode;

pub fn part1(input: &[String]) {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.set_direct(1, 12);
    computer.set_direct(2, 2);
    computer.run();
    println!("{}", computer.get_direct(0));
}

pub fn part2(input: &[String]) {
    let start = intcode::Computer::new(&input[0]).unwrap();
    'end: for noun in 0..100 {
        for verb in 0..100 {
            let mut computer = start.clone();
            computer.set_direct(1, noun);
            computer.set_direct(2, verb);
            computer.run();
            if computer.get_direct(0) == 19690720 {
                println!("{}", 100 * noun + verb);
                break 'end;
            }
        }
    }
}
