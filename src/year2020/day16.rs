use field::Field;
use std::collections::HashMap;

pub fn part1(input: &[String]) -> u32 {
    let mut i = 0;
    let fields = parse_fields(input, &mut i);
    i += 5;
    let mut total = 0;
    while i < input.len() {
        for &value in &parse_ticket(&input[i]) {
            let mut valid = false;
            for field in &fields {
                if field.valid(value) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                total += value;
            }
        }
        i += 1;
    }
    total
}

pub fn part2(input: &[String]) -> u64 {
    let mut i = 0;
    let fields = parse_fields(input, &mut i);
    let mut possibilities: HashMap<usize, Vec<usize>> = HashMap::new();
    i += 2;
    for i in 0..fields.len() {
        possibilities.insert(i, (0..fields.len()).collect());
    }
    let yours: Vec<u32> = parse_ticket(&input[i]);
    i += 3;
    while i < input.len() {
        let mut ticket_valid = true;
        let ticket: Vec<u32> = parse_ticket(&input[i]);
        for &value in &ticket {
            let mut valid = false;
            for field in &fields {
                if field.valid(value) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                ticket_valid = false;
                break;
            }
        }
        if ticket_valid {
            for (i, value) in ticket.iter().enumerate() {
                possibilities
                    .entry(i)
                    .and_modify(|x| x.retain(|&y| fields[y].valid(*value)));
            }
        }
        i += 1;
    }
    let mut ordered: HashMap<usize, usize> = HashMap::new();
    while let Some((i, field)) = reduce(&possibilities) {
        ordered.insert(i, field);
        possibilities.remove(&i);
        possibilities
            .values_mut()
            .for_each(|x| x.retain(|&y| y != field));
    }
    let mut product = 1;
    ordered.iter().for_each(|(&i, &field)| {
        if &fields[field].name()[..2] == "de" {
            product *= yours[i] as u64
        }
    });
    product
}

fn parse_fields(input: &[String], i: &mut usize) -> Vec<Field> {
    let mut fields = Vec::new();
    while input[*i].len() > 0 {
        fields.push(Field::new(&input[*i]));
        *i += 1;
    }
    fields
}

fn parse_ticket(line: &str) -> Vec<u32> {
    line.split(',').map(|x| x.parse().unwrap()).collect()
}

fn reduce(map: &HashMap<usize, Vec<usize>>) -> Option<(usize, usize)> {
    for (i, fields) in map {
        if fields.len() == 1 {
            return Some((*i, fields[0]));
        }
    }
    None
}

mod field {
    pub struct Field {
        name: String,
        range1: (u32, u32),
        range2: (u32, u32),
    }

    impl Field {
        pub fn new(line: &str) -> Field {
            let mut parts = line.split(": ");
            let name = String::from(parts.next().unwrap());
            parts = parts.next().unwrap().split(" or ");
            let mut range1 = parts.next().unwrap().split('-');
            let mut range2 = parts.next().unwrap().split('-');
            Field {
                name,
                range1: (
                    range1.next().unwrap().parse().unwrap(),
                    range1.next().unwrap().parse().unwrap(),
                ),
                range2: (
                    range2.next().unwrap().parse().unwrap(),
                    range2.next().unwrap().parse().unwrap(),
                ),
            }
        }

        pub fn valid(&self, value: u32) -> bool {
            (value >= self.range1.0 && value <= self.range1.1)
                || (value >= self.range2.0 && value <= self.range2.1)
        }

        pub fn name(&self) -> &str {
            &self.name
        }
    }
}
