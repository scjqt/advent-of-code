pub fn part1(input: &[String]) {
    let mut fewest = usize::MAX;
    let mut result = 0;
    for i in 0..input[0].len() / (25 * 6) {
        let (mut zeros, mut ones, mut twos) = (0, 0, 0);
        for digit in input[0][i * (25 * 6)..(i + 1) * (25 * 6)].chars() {
            match digit {
                '0' => zeros += 1,
                '1' => ones += 1,
                '2' => twos += 1,
                _ => panic!(),
            }
        }
        if zeros < fewest {
            fewest = zeros;
            result = ones * twos;
        }
    }
    println!("{}", result);
}

pub fn part2(input: &[String]) {
    let mut image = Vec::new();
    image.resize_with(25 * 6, || '2');
    for i in 0..input[0].len() / (25 * 6) {
        for j in 0..(25 * 6) {
            if image[j] == '2' {
                image[j] = input[0].chars().nth(i * (25 * 6) + j).unwrap();
            }
        }
    }
    for y in 0..6 {
        for x in 0..25 {
            print!("{}", if image[x + y * 25] == '1' { "■" } else { " " });
        }
        println!();
    }
}
