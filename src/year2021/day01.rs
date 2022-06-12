pub fn part1(input: &[String]) {
    println!("{}", total(parse(input)));
}

pub fn part2(input: &[String]) {
    let total = total(
        parse(input)
            .windows(3)
            .map(|depths| depths.iter().sum())
            .collect(),
    );
    println!("{}", total);
}

fn parse(input: &[String]) -> Vec<u16> {
    input.iter().map(|depth| depth.parse().unwrap()).collect()
}

fn total(values: Vec<u16>) -> u16 {
    values
        .windows(2)
        .fold(0, |acc, pair| acc + if pair[1] > pair[0] { 1 } else { 0 })
}
