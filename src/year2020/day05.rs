use std::collections::HashSet;

pub fn part1(input: &[String]) {
    let mut highest = 0;
    for seat in input {
        let id = seat_id(seat);
        if id > highest {
            highest = id;
        }
    }
    println!("{}", highest);
}

pub fn part2(input: &[String]) {
    let mut seats: HashSet<i32> = (0..1 << 10).collect();
    for seat in input {
        seats.remove(&seat_id(seat));
    }
    for seat in &seats {
        if !seats.contains(&(*seat + 1)) && !seats.contains(&(*seat - 1)) {
            println!("{}", *seat);
            break;
        }
    }
}

fn seat_id(seat: &str) -> i32 {
    let mut total = 0;
    for (i, letter) in seat.chars().enumerate() {
        total += match letter {
            'B' | 'R' => 1 << (9 - i),
            _ => 0,
        };
    }
    total
}
