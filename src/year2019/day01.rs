pub fn part1(input: &[String]) -> impl ToString {
    input
        .iter()
        .map(|mass| mass.parse::<i32>().unwrap() / 3 - 2)
        .sum::<i32>()
}

pub fn part2(input: &[String]) -> impl ToString {
    let mut total = 0;
    for mut mass in input.iter().map(|m| m.parse::<i32>().unwrap()) {
        while mass > 0 {
            mass = mass / 3 - 2;
            total += mass.max(0);
        }
    }
    total
}
