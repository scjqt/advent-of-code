use super::intcode;

pub fn part1(input: &[String]) -> i32 {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    let mut total = 0;
    for y in 0..50 {
        for x in 0..50 {
            if affected(&computer, x, y) {
                total += 1;
            }
        }
    }
    total
}

pub fn part2(input: &[String]) -> i64 {
    let mut computer = intcode::Computer::new(&input[0]).unwrap();
    computer.run();
    let mut start_x = 0;
    let mut y = 0;
    loop {
        while !affected(&computer, start_x, y) {
            start_x += 1;
        }
        let mut x = start_x;
        while affected(&computer, x + 99, y) {
            if affected(&computer, x, y + 99) {
                return x * 10000 + y;
            }
            x += 1;
        }
        y += 1;
    }
}

fn affected(computer: &intcode::Computer, x: i64, y: i64) -> bool {
    let mut computer = computer.clone();
    computer.input(x);
    computer.input(y);
    computer.output().unwrap() == 1
}
