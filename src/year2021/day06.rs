use std::collections::HashMap;

pub fn part1(input: &[String]) -> u64 {
    simulate(input, 80)
}

pub fn part2(input: &[String]) -> u64 {
    simulate(input, 256)
}

fn parse(input: &[String]) -> [u64; 6] {
    let mut frequencies = [0; 6];
    for age in input[0].split(',') {
        frequencies[age.parse::<usize>().unwrap()] += 1;
    }
    frequencies
}

fn simulate(input: &[String], days: u16) -> u64 {
    let frequencies = parse(input);

    (0..6)
        .map(|age| frequencies[age as usize] * fish(days + 6 - age, &mut HashMap::new()))
        .sum()
}

fn fish(days: u16, cache: &mut HashMap<u16, u64>) -> u64 {
    if days < 9 {
        return (days / 7 + 1) as u64;
    }
    if !cache.contains_key(&days) {
        let population = fish(days - 7, cache) + fish(days - 9, cache);
        cache.insert(days, population);
    }
    *cache.get(&days).unwrap()
}
