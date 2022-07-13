use array2d::Array2D;

pub fn part1(input: &[String]) -> u16 {
    get_low_points(input).0
}

pub fn part2(input: &[String]) -> i32 {
    let low_points = get_low_points(input).1;

    let mut highest = [0; 3];

    for count in low_points.into_iter() {
        if count > highest[0] {
            highest[2] = highest[1];
            highest[1] = highest[0];
            highest[0] = count;
        } else if count > highest[1] {
            highest[2] = highest[1];
            highest[1] = count;
        } else if count > highest[2] {
            highest[2] = count;
        }
    }

    highest[0] * highest[1] * highest[2]
}

fn get_low_points(input: &[String]) -> (u16, Array2D<i32>) {
    let mut heightmap: Array2D<u16> = Array2D::new(input[0].len(), input.len(), 0);
    let mut pos = heightmap.positions();
    for row in input {
        for height in row.chars() {
            heightmap[pos.next().unwrap()] = height.to_string().parse().unwrap();
        }
    }
    let heightmap = heightmap;

    let mut low_points = Array2D::new(heightmap.width(), heightmap.height(), 0);
    let mut total = 0;

    for pos in heightmap.positions() {
        if heightmap[pos] != 9 {
            let mut current = pos;

            while let Some(adjacent) = lower_neighbour(&heightmap, current) {
                current = adjacent;
            }

            let count = low_points.get_mut(current).unwrap();
            if *count == 0 {
                total += heightmap[current] + 1;
            }
            *count += 1;
        }
    }

    (total, low_points)
}

fn lower_neighbour(heightmap: &Array2D<u16>, pos: (usize, usize)) -> Option<(usize, usize)> {
    const OFFSETS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let height = heightmap[pos];
    for offset in OFFSETS {
        if let Some(adj) = heightmap.offset(pos, offset) {
            if heightmap[adj] < height {
                return Some(adj);
            }
        }
    }
    None
}
