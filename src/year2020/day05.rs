pub fn part1(input: &[String]) {
    let highest = input.iter().map(seat_id).max().unwrap();
    println!("{}", highest);
}

pub fn part2(input: &[String]) {
    let mut occupied = vec![false; 1 << 10];
    for seat in input {
        occupied[seat_id(seat)] = true;
    }
    for i in 1..(1 << 10) - 1 {
        if !occupied[i] && occupied[i - 1] && occupied[i + 1] {
            println!("{}", i);
            break;
        }
    }
}

fn seat_id(seat: &String) -> usize {
    let mut id = 0;
    for letter in seat.chars() {
        id <<= 1;
        if let 'B' | 'R' = letter {
            id |= 1;
        }
    }
    id
}
