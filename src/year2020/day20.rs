use array2d::Array2D;
use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) {
    let (tiles, _, minx, miny) = parse(input);
    let mut total = 1;
    for tiley in 0..12 {
        for tilex in 0..12 {
            if [(0, 0), (11, 0), (0, 11), (11, 11)].contains(&(tilex, tiley)) {
                total *= tiles
                    .get(&(tilex as i8 + minx, tiley as i8 + miny))
                    .unwrap()
                    .id as u64;
            }
        }
    }
    println!("{}", total);
}

pub fn part2(input: &[String]) {
    let (tiles, raw, minx, miny) = parse(input);
    let mut image = Array2D::new(96, 96, false);
    for tiley in 0..12 {
        for tilex in 0..12 {
            let tile = tiles
                .get(&(tilex as i8 + minx, tiley as i8 + miny))
                .unwrap();
            for y in 1..9 {
                for x in 1..9 {
                    let (mut newx, mut newy) = (x, y);
                    if tile.flipped {
                        newy = 9 - newy;
                    }
                    for _ in 0..tile.rotation {
                        let temp = newx;
                        newx = newy;
                        newy = 9 - temp;
                    }
                    image[[tilex * 8 + newx - 1, tiley * 8 + newy - 1]] =
                        raw.get(&tile.id).unwrap().data[[x, y]];
                }
            }
        }
    }
    let sea_monster: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
        (4, 0),
        (5, 1),
        (6, 1),
        (7, 0),
        (10, 0),
        (11, 1),
        (12, 1),
        (13, 0),
        (16, 0),
        (17, 1),
        (18, 1),
        (18, 2),
        (19, 1),
    ];
    for r in 0..4 {
        if remove_sea_monsters(&mut image, orient_sea_monster(&sea_monster, false, r))
            || remove_sea_monsters(&mut image, orient_sea_monster(&sea_monster, true, r))
        {
            break;
        }
    }
    println!("{}", image.into_iter().filter(|x| *x).count());
}

fn parse(input: &[String]) -> (HashMap<(i8, i8), Tile>, HashMap<u16, Raw>, i8, i8) {
    let mut ids: HashSet<u16> = HashSet::new();
    let mut raw: HashMap<u16, Raw> = HashMap::new();
    let mut tiles: HashMap<(i8, i8), Tile> = HashMap::new();
    let mut i = 0;
    while i < input.len() {
        let id = input[i][5..9].parse().unwrap();
        let tile = Raw::from(&input[i + 1..i + 11]);
        if i == 0 {
            tiles.insert(
                (0, 0),
                Tile {
                    id,
                    border: tile.border.clone(),
                    flipped: false,
                    rotation: 0,
                    exposed: [0, 1, 2, 3].iter().cloned().collect(),
                },
            );
        } else {
            ids.insert(id);
        }
        raw.insert(id, tile);
        i += 12;
    }
    let (mut minx, mut miny) = (0, 0);
    while ids.len() > 0 {
        let mut remove = Vec::new();
        for &id in &ids {
            let mut new = None;
            'matched: for (&(tilex, tiley), tile) in tiles.iter() {
                for &i in &tile.exposed {
                    for (j, &side) in raw.get(&id).unwrap().border.iter().enumerate() {
                        match tile.border[i].0 {
                            a if a == side.0 => {
                                new = Some((tilex, tiley, i, j, true));
                                break 'matched;
                            }
                            a if a == side.1 => {
                                new = Some((tilex, tiley, i, j, false));
                                break 'matched;
                            }
                            _ => (),
                        };
                    }
                }
            }
            if let Some((mut tilex, mut tiley, i, mut j, flipped)) = new {
                let (offx, offy) = [(0, -1), (1, 0), (0, 1), (-1, 0)][i];
                tilex += offx;
                tiley += offy;
                let mut old = raw.get(&id).unwrap().border.clone();
                if flipped {
                    old = [
                        (old[2].1, old[2].0),
                        (old[1].1, old[1].0),
                        (old[0].1, old[0].0),
                        (old[3].1, old[3].0),
                    ];
                    j = match j {
                        0 => 2,
                        2 => 0,
                        x => x,
                    }
                }
                let mut border = [(0, 0); 4];
                let rotation = ((6 + j - i) % 4) as u8;
                let mut k = rotation;
                for l in 0..4 {
                    border[l] = old[k as usize];
                    k = (k + 1) % 4;
                }
                let mut exposed: HashSet<usize> = [0, 1, 2, 3].iter().cloned().collect();
                for (m, &(x, y)) in [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().enumerate() {
                    if let Some(adjacent) = tiles.get_mut(&(tilex + x, tiley + y)) {
                        adjacent.exposed.remove(&((m + 2) % 4));
                        exposed.remove(&m);
                    }
                }
                tiles.insert(
                    (tilex, tiley),
                    Tile {
                        id,
                        border,
                        flipped,
                        rotation,
                        exposed,
                    },
                );
                if tilex < minx {
                    minx = tilex;
                }
                if tiley < miny {
                    miny = tiley;
                }
                remove.push(id);
            }
        }
        for id in &remove {
            ids.remove(id);
        }
    }
    (tiles, raw, minx, miny)
}

fn orient_sea_monster(default: &Vec<(i32, i32)>, flipped: bool, rotation: u8) -> Vec<(i32, i32)> {
    default
        .iter()
        .map(|&(x, y)| {
            let (mut newx, mut newy) = (x, y);
            if flipped {
                newy = -newy;
            }
            for _ in 0..rotation {
                let temp = newy;
                newy = -newx;
                newx = temp;
            }
            (newx, newy)
        })
        .collect()
}

fn remove_sea_monsters(image: &mut Array2D<bool>, sea_monster: Vec<(i32, i32)>) -> bool {
    let (mut minx, mut miny, mut maxx, mut maxy) = (0, 0, 0, 0);
    sea_monster.iter().for_each(|&(x, y)| {
        match x {
            a if a < minx => minx = a,
            a if a > maxx => maxx = a,
            _ => (),
        }
        match y {
            a if a < miny => miny = a,
            a if a > maxy => maxy = a,
            _ => (),
        }
    });
    let mut correct = false;
    for y in -miny..96 - maxy {
        for x in -minx..96 - maxx {
            let mut is_monster = true;
            for &(offx, offy) in &sea_monster {
                if !image[[(x + offx) as usize, (y + offy) as usize]] {
                    is_monster = false;
                    break;
                }
            }
            if is_monster {
                correct = true;
                for &(offx, offy) in &sea_monster {
                    image[[(x + offx) as usize, (y + offy) as usize]] = false;
                }
            }
        }
    }
    correct
}

struct Raw {
    data: Array2D<bool>,
    border: [(u16, u16); 4],
}

impl Raw {
    fn from(input: &[String]) -> Raw {
        let mut data = Array2D::new(10, 10, false);
        let mut pos = data.positions();
        for line in input {
            for val in line.chars() {
                data[pos.next().unwrap()] = val == '#';
            }
        }
        let mut border = [(0, 0); 4];
        for a in 0..10 {
            if data[[a, 0]] {
                border[0].0 += 1 << (9 - a);
                border[0].1 += 1 << a;
            }
            if data[[a, 9]] {
                border[2].0 += 1 << a;
                border[2].1 += 1 << (9 - a);
            }
            if data[[0, a]] {
                border[3].0 += 1 << a;
                border[3].1 += 1 << (9 - a);
            }
            if data[[9, a]] {
                border[1].0 += 1 << (9 - a);
                border[1].1 += 1 << a;
            }
        }
        Raw { data, border }
    }
}

struct Tile {
    id: u16,
    border: [(u16, u16); 4],
    flipped: bool,
    rotation: u8,
    exposed: HashSet<usize>,
}
