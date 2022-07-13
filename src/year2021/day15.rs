use array2d::Array2D;
use search::dijkstra;

pub fn part1(input: &[String]) -> u16 {
    lowest_total_risk(parse(input))
}

pub fn part2(input: &[String]) -> u16 {
    let levels = parse(input);
    lowest_total_risk(Array2D::from_fn(
        levels.width() * 5,
        levels.height() * 5,
        |(x, y)| {
            (levels[[x % levels.width(), y % levels.height()]] - 1
                + (x / levels.width()) as u8
                + (y / levels.height()) as u8)
                % 9
                + 1
        },
    ))
}

fn lowest_total_risk(levels: Array2D<u8>) -> u16 {
    const OFFSETS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let end = (levels.width() - 1, levels.height() - 1);
    dijkstra(
        ((0, 0), 0),
        |&(pos, cost)| {
            let mut adj = Vec::new();
            for offset in OFFSETS {
                if let Some(p) = levels.offset(pos, offset) {
                    adj.push((p, cost + levels[p] as u16));
                }
            }
            adj
        },
        |&(pos, _)| pos,
        |&(_, cost)| cost,
    )
    .find(|&(pos, ..)| pos == end)
    .unwrap()
    .1
}

fn parse(input: &[String]) -> Array2D<u8> {
    let mut levels = Array2D::new(input[0].len(), input.len(), 0);
    let mut pos = levels.positions();
    for line in input {
        for level in line.chars() {
            levels[pos.next().unwrap()] = level.to_string().parse().unwrap();
        }
    }
    levels
}
