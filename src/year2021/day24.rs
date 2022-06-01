pub fn part1(input: &[String]) {
    let relationships = relationships(input);
    let mut number = [0; 14];
    for (i, j, o) in relationships {
        number[i] = 9.min(9 - o);
        number[j] = number[i] + o;
    }
    display(number);
}

pub fn part2(input: &[String]) {
    let relationships = relationships(input);
    let mut number = [0; 14];
    for (i, j, o) in relationships {
        number[i] = 1.max(1 - o);
        number[j] = number[i] + o;
    }
    display(number);
}

fn relationships(input: &[String]) -> Vec<(usize, usize, i8)> {
    let mut a = [0; 14];
    let mut b = [0; 14];
    for i in 0..14 {
        let j = i * 18;
        a[i] = parse(&input[j + 5]);
        b[i] = parse(&input[j + 15]);
    }

    let mut relationships = Vec::new();
    let mut stack = Vec::new();
    for i in 0..14 {
        if a[i] > 7 {
            stack.push((i, b[i]));
        } else {
            let (j, x) = stack.pop().unwrap();
            relationships.push((j, i, x + a[i]));
        }
    }
    relationships
}

fn parse(line: &str) -> i8 {
    line.split_whitespace().last().unwrap().parse().unwrap()
}

fn display(number: [i8; 14]) {
    println!(
        "{}",
        number
            .into_iter()
            .map(|d| d.to_string())
            .collect::<String>()
    );
}
