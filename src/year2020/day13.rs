pub fn part1(input: &[String]) -> impl ToString {
    let timestamp: u32 = input[0].parse().unwrap();
    let mut lowest = 100;
    let mut id = 0;
    input[1]
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse().unwrap())
        .for_each(|x| {
            let wait = x - (timestamp % x);
            if wait < lowest {
                lowest = wait;
                id = x;
            }
        });
    id * lowest
}

pub fn part2(input: &[String]) -> impl ToString {
    let values: Vec<(u16, usize)> = input[1]
        .split(',')
        .enumerate()
        .filter_map(|(i, val)| {
            if val == "x" {
                None
            } else {
                Some((val.parse::<u16>().unwrap(), i))
            }
        })
        .collect();
    let mut timestamp: u64 = 0;
    let mut increment = 1;
    for value in &values {
        while (timestamp + value.1 as u64) % value.0 as u64 != 0 {
            timestamp += increment;
        }
        increment *= value.0 as u64;
    }
    timestamp
}
