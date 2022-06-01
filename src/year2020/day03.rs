use array2d::Array2D;

pub fn part1(input: &[String]) {
    println!("{}", encounters(&parse(input), 3, 1));
}

pub fn part2(input: &[String]) {
    let map = parse(input);
    println!(
        "{}",
        encounters(&map, 1, 1)
            * encounters(&map, 3, 1)
            * encounters(&map, 5, 1)
            * encounters(&map, 7, 1)
            * encounters(&map, 1, 2)
    )
}

fn encounters(map: &Array2D<bool>, offx: usize, offy: usize) -> i64 {
    let (mut x, mut y) = (0, 0);
    let mut total = 0;
    while y < map.height() {
        total += if map[[x, y]] { 1 } else { 0 };
        x = (x + offx) % map.width();
        y += offy;
    }
    total
}

fn parse(input: &[String]) -> Array2D<bool> {
    let mut arr = Array2D::new(input[0].len(), input.len(), false);
    let mut pos_iter = arr.positions();
    for line in input {
        for tile in line.chars() {
            let pos = pos_iter.next().unwrap();
            if tile == '#' {
                arr[pos] = true;
            }
        }
    }
    arr
}
