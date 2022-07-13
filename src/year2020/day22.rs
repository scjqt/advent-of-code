use std::collections::{HashSet, VecDeque};

pub fn part1(input: &[String]) -> impl ToString {
    let (mut player1, mut player2) = decks(input);
    while player1.len() * player2.len() > 0 {
        let (one, two) = (player1.pop_front().unwrap(), player2.pop_front().unwrap());
        if one > two {
            player1.push_back(one);
            player1.push_back(two);
        } else {
            player2.push_back(two);
            player2.push_back(one);
        }
    }
    score(if player1.len() == 0 { player2 } else { player1 })
}

pub fn part2(input: &[String]) -> impl ToString {
    let (player1, player2) = decks(input);
    i32::abs(play(player1, player2))
}

fn play(mut player1: VecDeque<u8>, mut player2: VecDeque<u8>) -> i32 {
    let mut previous: HashSet<(VecDeque<u8>, VecDeque<u8>)> = HashSet::new();
    while player1.len() * player2.len() > 0 {
        let state = (player1.clone(), player2.clone());
        if previous.contains(&state) {
            return 1;
        } else {
            previous.insert(state);
        }
        let (one, two) = (player1.pop_front().unwrap(), player2.pop_front().unwrap());
        let winner = if player1.len() as u8 >= one && player2.len() as u8 >= two {
            let (mut new1, mut new2) = (player1.clone(), player2.clone());
            new1.truncate(one as usize);
            new2.truncate(two as usize);
            play(new1, new2) > 0
        } else {
            one > two
        };
        if winner {
            player1.push_back(one);
            player1.push_back(two);
        } else {
            player2.push_back(two);
            player2.push_back(one);
        }
    }
    if player1.len() > 0 {
        score(player1)
    } else {
        -score(player2)
    }
}

fn score(deck: VecDeque<u8>) -> i32 {
    deck.iter()
        .enumerate()
        .map(|(i, &val)| val as i32 * (deck.len() - i) as i32)
        .sum::<i32>()
}

fn decks(input: &[String]) -> (VecDeque<u8>, VecDeque<u8>) {
    let mut decks = input.split(|x| x.len() == 0);
    (
        decks
            .next()
            .unwrap()
            .iter()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect(),
        decks
            .next()
            .unwrap()
            .iter()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect(),
    )
}
