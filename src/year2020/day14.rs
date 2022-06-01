use std::collections::HashMap;

pub fn part1(input: &[String]) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut and: u64 = 0;
    let mut or: u64 = 0;
    for line in input {
        if &line[0..4] == "mask" {
            and = 0;
            or = 0;
            for (i, char) in line[7..].chars().enumerate() {
                and += if char == 'X' { 1 << 35 - i } else { 0 };
                or += if char == '1' { 1 << 35 - i } else { 0 };
            }
        } else {
            let (address, value) = decode(line);
            memory.insert(address, (value & and) | or);
        }
    }
    println!("{}", memory.values().sum::<u64>());
}

pub fn part2(input: &[String]) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = "";
    for line in input {
        if &line[0..4] == "mask" {
            mask = &line[7..];
        } else {
            let (address, value) = decode(line);
            set_memory(&mut memory, mask, address, value);
        }
    }
    println!("{}", memory.values().sum::<u64>());
}

fn decode(line: &str) -> (u64, u64) {
    (
        line[line.find('[').unwrap() + 1..line.find(']').unwrap()]
            .parse()
            .unwrap(),
        line[line.find('=').unwrap() + 2..].parse().unwrap(),
    )
}

fn set_memory(memory: &mut HashMap<u64, u64>, mask: &str, address: u64, value: u64) {
    if mask.len() == 0 {
        memory.insert(address, value);
    } else {
        match &mask[0..1] {
            "1" => set_memory(memory, &mask[1..], address | 1 << mask.len() - 1, value),
            "0" => set_memory(memory, &mask[1..], address, value),
            "X" => {
                set_memory(memory, &mask[1..], address | 1 << mask.len() - 1, value);
                set_memory(memory, &mask[1..], address & !(1 << mask.len() - 1), value);
            }
            _ => panic!(),
        }
    }
}
