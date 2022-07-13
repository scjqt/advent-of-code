use std::ops::RangeInclusive;

pub fn part1(input: &[String]) -> impl ToString {
    let mut total = 0;
    for number in range(&input[0]) {
        let string = number.to_string();
        let mut last = 0;
        let mut valid = false;
        for digit in string.chars().map(|c| c.to_digit(10).unwrap()) {
            if digit == last {
                valid = true;
            } else if digit < last {
                valid = false;
                break;
            }
            last = digit;
        }
        if valid {
            total += 1;
        }
    }
    total
}

pub fn part2(input: &[String]) -> impl ToString {
    let mut total = 0;
    for number in range(&input[0]) {
        let string = number.to_string();
        let mut last = 0;
        let mut valid = false;
        let mut two = false;
        let mut three = false;
        for digit in string.chars().map(|c| c.to_digit(10).unwrap()) {
            if digit == last {
                if two {
                    three = true;
                    two = false;
                } else if !three {
                    two = true;
                }
            } else {
                three = false;
                if digit < last {
                    valid = false;
                    two = false;
                    break;
                } else if two {
                    valid = true;
                }
            }
            last = digit;
        }
        if valid || two {
            total += 1;
        }
    }
    total
}

fn range(input: &str) -> RangeInclusive<u32> {
    let mut parts = input.split('-');
    let lower: u32 = parts.next().unwrap().parse().unwrap();
    let upper: u32 = parts.next().unwrap().parse().unwrap();
    lower..=upper
}
