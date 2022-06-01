use std::collections::HashSet;

pub fn part1(input: &[String]) {
    println!("{}", target(input));
}

pub fn part2(input: &[String]) {
    let target = target(input);
    let numbers: Vec<u64> = input.iter().map(|x| x.parse().unwrap()).collect();
    for start in 0..numbers.len() - 1 {
        let mut end = start + 1;
        let mut sum = numbers[start] + numbers[end];
        while end < numbers.len() - 1 && sum < target {
            end += 1;
            sum += numbers[end];
        }
        if sum == target {
            let set = (&numbers[start..end + 1]).iter();
            println!("{}", set.clone().min().unwrap() + set.max().unwrap());
            break;
        }
    }
}

fn target(input: &[String]) -> u64 {
    let numbers: Vec<u64> = input.iter().map(|x| x.parse().unwrap()).collect();
    for i in 25..numbers.len() {
        if !valid(&numbers[i - 25..i], numbers[i]) {
            return numbers[i];
        }
    }
    0
}

fn valid(numbers: &[u64], sum: u64) -> bool {
    let mut set = HashSet::new();
    for number in numbers {
        if set.contains(number) {
            return true;
        }
        if sum >= *number {
            set.insert(sum - number);
        }
    }
    false
}
