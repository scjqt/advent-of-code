pub fn part1(input: &[String]) -> impl ToString {
    let numbers = interpret(input);
    let mut ones = 0;
    let mut threes = 0;
    for i in 0..numbers.len() - 1 {
        match numbers[i + 1] - numbers[i] {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }
    ones * threes
}

pub fn part2(input: &[String]) -> impl ToString {
    let numbers = interpret(input);
    let mut total: u64 = 1;
    let mut consecutive = 1;
    for i in 0..numbers.len() - 1 {
        if i + 1 == numbers.len() || numbers[i + 1] - numbers[i] == 3 {
            total *= f(consecutive);
            consecutive = 1;
        } else {
            consecutive += 1;
        }
    }
    total
}

fn f(n: i8) -> u64 {
    match n {
        1 => 1,
        _ if n < 1 => 0,
        _ => f(n - 1) + f(n - 2) + f(n - 3),
    }
}

fn interpret(input: &[String]) -> Vec<u8> {
    let mut numbers: Vec<u8> = input.iter().map(|x| x.parse().unwrap()).collect();
    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers[numbers.len() - 1] + 3);
    numbers
}
