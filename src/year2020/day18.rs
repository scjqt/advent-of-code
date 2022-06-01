pub fn part1(input: &[String]) {
    println!("{}", input.iter().map(|x| evaluate(x)).sum::<u64>());
}

pub fn part2(input: &[String]) {
    println!(
        "{}",
        input
            .iter()
            .map(|x| evaluate(&fix_precedence(x)))
            .sum::<u64>()
    );
}

fn evaluate(expression: &str) -> u64 {
    if expression.len() == 1 {
        expression[0..1].parse().unwrap()
    } else {
        let mut i = expression.len() - 1;
        let value = if &expression[i..=i] == ")" {
            let mut count = 1;
            while count > 0 {
                i -= 1;
                match &expression[i..=i] {
                    ")" => count += 1,
                    "(" => count -= 1,
                    _ => (),
                }
            }
            evaluate(&expression[i + 1..expression.len() - 1])
        } else {
            evaluate(&expression[i..=i])
        };
        if i == 0 {
            value
        } else {
            i -= 2;
            match &expression[i..=i] {
                "+" => evaluate(&expression[..i - 1]) + value,
                "*" => evaluate(&expression[..i - 1]) * value,
                _ => panic!(),
            }
        }
    }
}

fn fix_precedence(expression: &str) -> String {
    let mut chars: Vec<char> = expression.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '+' {
            if chars[i + 2] == '(' {
                let mut j = i + 2;
                let mut count = 1;
                while count > 0 {
                    j += 1;
                    match chars[j] {
                        '(' => count += 1,
                        ')' => count -= 1,
                        _ => (),
                    }
                }
                chars.insert(j + 1, ')');
            } else {
                chars.insert(i + 3, ')');
            }

            if chars[i - 2] == ')' {
                let mut j = i - 2;
                let mut count = 1;
                while count > 0 {
                    j -= 1;
                    match chars[j] {
                        ')' => count += 1,
                        '(' => count -= 1,
                        _ => (),
                    }
                }
                chars.insert(j, '(');
            } else {
                chars.insert(i - 2, '(');
            }
            i += 1;
        }
        i += 1;
    }
    chars.into_iter().collect()
}
