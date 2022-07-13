use super::intcode;
use std::collections::HashMap;

pub fn part1(input: &[String]) -> impl ToString {
    paint_hull(0, &input[0]).len()
}

pub fn part2(input: &[String]) -> impl ToString {
    let hull = paint_hull(1, &input[0]);
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
    for (&(x, y), &colour) in hull.iter() {
        if colour == 1 {
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }
    let mut result = String::new();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            result.push(if let Some(1) = hull.get(&(x, y)) {
                '#'
            } else {
                ' '
            });
        }
        if y > min_y {
            result.push('\n');
        }
    }
    result
}

fn paint_hull(starting_panel: u8, input: &str) -> HashMap<(i32, i32), u8> {
    let mut computer = intcode::Computer::new(input).unwrap();
    computer.run();
    let mut hull = HashMap::new();
    hull.insert((0, 0), starting_panel);
    let mut current = (0, 0);
    let (mut move_x, mut move_y) = (0, 1);
    while computer.state() != intcode::State::Halted {
        let colour = hull.entry(current).or_insert(0);
        computer.input(*colour as i64);
        *colour = computer.output().unwrap() as u8;
        let (x, y) = match computer.output().unwrap() {
            0 => (-move_y, move_x),
            1 => (move_y, -move_x),
            _ => panic!(),
        };
        move_x = x;
        move_y = y;
        current.0 += move_x;
        current.1 += move_y;
    }
    hull
}
