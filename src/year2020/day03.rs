use array2d::Array2D;

pub fn part1(input: &[String]) {
    println!("{}", encounters(&parse(input), 3, 1));
}

pub fn part2(input: &[String]) {
    let map = &parse(input);
    let encounters = encounters(map, 1, 1)
        * encounters(map, 3, 1)
        * encounters(map, 5, 1)
        * encounters(map, 7, 1)
        * encounters(map, 1, 2);
    println!("{}", encounters);
}

fn encounters(map: &Array2D<bool>, dx: usize, dy: usize) -> i64 {
    let (mut x, mut y) = (0, 0);
    let mut total = 0;
    while y < map.height() {
        total += if map[[x, y]] { 1 } else { 0 };
        x = (x + dx) % map.width();
        y += dy;
    }
    total
}

fn parse(input: &[String]) -> Array2D<bool> {
    let mut arr = Array2D::new(input[0].len(), input.len(), false);
    let mut pos_iter = arr.positions();
    for line in input {
        for tile in line.chars() {
            arr[pos_iter.next().unwrap()] = tile == '#';
        }
    }
    arr
}
