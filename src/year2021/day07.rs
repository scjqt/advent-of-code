pub fn part1(input: &[String]) {
    let mut crabs: Vec<u32> = parse(input);
    crabs.sort_unstable();
    println!("{}", fuel_one(&crabs, crabs[(crabs.len() - 1) / 2]));
}

pub fn part2(input: &[String]) {
    let crabs: Vec<u32> = parse(input);
    let (mut low, mut high) = (*crabs.iter().min().unwrap(), *crabs.iter().max().unwrap());
    loop {
        let position = (low + high) / 2;
        let fuel = fuel_two(&crabs, position);
        if fuel > fuel_two(&crabs, position - 1) {
            high = position;
        } else if fuel > fuel_two(&crabs, position + 1) {
            low = position;
        } else {
            println!("{}", fuel);
            break;
        }
    }
}

fn fuel_one(crabs: &[u32], position: u32) -> u32 {
    crabs
        .iter()
        .map(|&x| {
            if x > position {
                x - position
            } else {
                position - x
            }
        })
        .sum::<u32>()
}

fn fuel_two(crabs: &[u32], position: u32) -> u32 {
    crabs
        .iter()
        .map(|&x| {
            let diff = if x > position {
                x - position
            } else {
                position - x
            };
            (diff * (diff + 1)) / 2
        })
        .sum::<u32>()
}

fn parse(input: &[String]) -> Vec<u32> {
    input[0].split(',').map(|x| x.parse().unwrap()).collect()
}
