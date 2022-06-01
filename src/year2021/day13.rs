use std::collections::HashSet;

pub fn part1(input: &[String]) {
    let mut i = 0;
    let mut dots = parse(input, &mut i);
    i += 1;
    dots = fold(dots, &input[i]);
    println!("{}", dots.len());
}

pub fn part2(input: &[String]) {
    let mut i = 0;
    let mut dots = parse(input, &mut i);
    i += 1;
    while i < input.len() {
        dots = fold(dots, &input[i]);
        i += 1;
    }

    let (mut max_x, mut max_y) = (0, 0);
    for &(x, y) in dots.iter() {
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", if dots.contains(&(x, y)) { '#' } else { '.' });
        }
        println!();
    }
}

fn fold(dots: HashSet<(i16, i16)>, line: &str) -> HashSet<(i16, i16)> {
    let mut parts = line.split('=');
    let axis = parts.next().unwrap().chars().last().unwrap();
    let value: i16 = parts.next().unwrap().parse().unwrap();
    dots.into_iter()
        .map(|(x, y)| {
            if axis == 'x' {
                (value - (x - value).abs(), y)
            } else {
                (x, value - (y - value).abs())
            }
        })
        .collect()
}

fn parse(input: &[String], i: &mut usize) -> HashSet<(i16, i16)> {
    let mut dots: HashSet<(i16, i16)> = HashSet::new();

    while !input[*i].is_empty() {
        let mut parts = input[*i].split(',');
        dots.insert((
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        ));
        *i += 1;
    }

    dots
}
