use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) -> usize {
    let (foods, _, potential) = parse(input);

    foods
        .iter()
        .map(|food| food.difference(&potential).collect::<HashSet<_>>().len())
        .sum()
}

pub fn part2(input: &[String]) -> String {
    let mut allergens = parse(input).1;
    let mut dangerous: Vec<(String, String)> = Vec::new();
    while allergens.len() > 0 {
        let mut allergen = "".to_string();
        for (key, ingredients) in allergens.iter() {
            if ingredients.len() == 1 {
                allergen = key.clone();
                break;
            }
        }
        let ingredient = allergens
            .get(&allergen)
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .clone();
        dangerous.push((allergen.to_string(), ingredient.clone()));
        allergens.remove(&allergen);
        allergens
            .values_mut()
            .for_each(|ingredients| ingredients.retain(|x| *x != ingredient));
    }
    dangerous.sort_by(|a, b| a.0.cmp(&b.0));
    let mut result = dangerous[0].1.clone();
    dangerous.iter().skip(1).for_each(|(_, x)| {
        result += ",";
        result += x;
    });
    result
}

fn parse(
    input: &[String],
) -> (
    Vec<HashSet<String>>,
    HashMap<String, HashSet<String>>,
    HashSet<String>,
) {
    let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();
    let mut foods: Vec<HashSet<String>> = Vec::new();
    for line in input {
        let mut parts = line[..line.len() - 1].split(" (contains ");
        let food: HashSet<String> = parts
            .next()
            .unwrap()
            .split(' ')
            .map(|ingredient| ingredient.to_string())
            .collect();
        parts.next().unwrap().split(", ").for_each(|allergen| {
            let ingredients = allergens
                .entry(allergen.to_string())
                .or_insert(food.clone());
            *ingredients = ingredients.intersection(&food).cloned().collect();
        });
        foods.push(food);
    }
    let mut potential = HashSet::new();
    allergens
        .values()
        .for_each(|ingredients| potential.extend(ingredients.clone()));
    (foods, allergens, potential)
}
