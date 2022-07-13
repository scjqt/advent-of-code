use array2d::Array2D;

pub fn part1(input: &[String]) -> impl ToString {
    let mut image = Image::new(input, 4);
    for _ in 0..2 {
        image.enhance();
    }
    image.count()
}

pub fn part2(input: &[String]) -> impl ToString {
    let mut image = Image::new(input, 100);
    for _ in 0..50 {
        image.enhance();
    }
    image.count()
}

struct Image {
    algorithm: [bool; 512],
    image: Array2D<bool>,
}

impl Image {
    fn new(input: &[String], padding: usize) -> Image {
        let mut algorithm = [false; 512];
        for (i, c) in input[0].chars().enumerate() {
            if c == '#' {
                algorithm[i] = true;
            }
        }
        let dim = Array2D::new(input[2].len(), input.len() - 2, false);
        let mut image = Array2D::new(dim.width() + padding * 2, dim.height() + padding * 2, false);
        let mut pos = dim.positions().map(|(x, y)| (x + padding, y + padding));
        for line in &input[2..] {
            for c in line.chars() {
                image[pos.next().unwrap()] = c == '#';
            }
        }
        Image { algorithm, image }
    }

    fn enhance(&mut self) {
        self.image = Array2D::from_fn(self.image.width() - 2, self.image.height() - 2, |pos| {
            self.new_value(pos)
        });
    }

    fn new_value(&self, pos: (usize, usize)) -> bool {
        const OFFSETS: [(isize, isize); 9] = [
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ];
        let mut i = 0;
        for offset in OFFSETS {
            i <<= 1;
            if self.image[self.image.offset(pos, offset).unwrap()] {
                i += 1;
            }
        }
        self.algorithm[i]
    }

    fn count(self) -> usize {
        self.image.into_iter().filter(|x| *x).count()
    }
}
