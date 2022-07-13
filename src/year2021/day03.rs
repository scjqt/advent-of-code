use std::collections::HashSet;

pub fn part1(input: &[String]) -> impl ToString {
    let mut frequencies = Vec::new();
    frequencies.resize(input[0].len(), 0);
    for number in input {
        for (i, digit) in number.chars().enumerate() {
            frequencies[i] += if digit == '1' { 1 } else { -1 };
        }
    }
    let gamma = frequencies
        .iter()
        .fold(0, |acc, &freq| (acc << 1) + if freq > 0 { 1 } else { 0 });
    gamma * ((1 << frequencies.len()) - gamma - 1)
}

pub fn part2(input: &[String]) -> impl ToString {
    let len = input[0].len() as u16;
    let numbers: HashSet<u16> = input
        .iter()
        .map(|number| {
            number.chars().fold(0, |acc, digit| {
                (acc << 1) + digit.to_digit(10).unwrap() as u16
            })
        })
        .collect();

    purge(numbers.clone(), len, false) as u32 * purge(numbers, len, true) as u32
}

fn purge(mut numbers: HashSet<u16>, len: u16, mode: bool) -> u16 {
    let mut i = len;
    while numbers.len() > 1 {
        i -= 1;
        let mut freq = 0;
        for &number in numbers.iter() {
            freq += if (number >> i) & 1 == 1 { 1 } else { -1 };
        }
        let value = if (freq < 0) ^ mode { 0 } else { 1 };
        numbers.retain(|number| (*number >> i) & 1 == value);
    }
    numbers.into_iter().collect::<Vec<_>>()[0]
}
