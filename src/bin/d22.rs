use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::io::{stdin, Read};

type Card = u8;

fn recursive_combat(player1: &mut VecDeque<Card>, player2: &mut VecDeque<Card>) -> bool {
    let mut prev_states: HashSet<(VecDeque<Card>, VecDeque<Card>)> = HashSet::new();
    while !player1.is_empty() && !player2.is_empty() {
        let key = (player1.clone(), player2.clone());
        if prev_states.contains(&key) {
            return true;
        }
        prev_states.insert(key);

        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();
        if (c1 as usize) <= player1.len() && (c2 as usize) <= player2.len() {
            // Recurse
            let mut player1_sub: VecDeque<_> =
                player1.iter().take(c1 as usize).map(|&x| x).collect();
            let mut player2_sub: VecDeque<_> =
                player2.iter().take(c2 as usize).map(|&x| x).collect();
            let p1_sub_winner = recursive_combat(&mut player1_sub, &mut player2_sub);
            if p1_sub_winner {
                player1.push_back(c1);
                player1.push_back(c2);
            } else {
                player2.push_back(c2);
                player2.push_back(c1);
            }
        } else {
            combat_round(player1, c1, player2, c2);
        }
    }
    !player1.is_empty()
}

fn combat_round(
    player1: &mut VecDeque<Card>,
    card1: Card,
    player2: &mut VecDeque<Card>,
    card2: Card,
) {
    match card1.cmp(&card2) {
        Ordering::Greater => {
            player1.push_back(card1);
            player1.push_back(card2);
        }
        Ordering::Less => {
            player2.push_back(card2);
            player2.push_back(card1);
        }
        Ordering::Equal => panic!("Equal cards"),
    }
}

fn score(deck: &VecDeque<Card>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| (i + 1) * v as usize)
        .sum()
}

fn deck_from_str(s: &str) -> VecDeque<Card> {
    s.lines().skip(1).map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let split = input.find("\n\n").unwrap();
    let mut player1 = deck_from_str(&input[..split]);
    let mut player2 = deck_from_str(&input[split + 2..]);

    let mut player1_part1 = player1.clone();
    let mut player2_part1 = player2.clone();

    while !player1_part1.is_empty() && !player2_part1.is_empty() {
        let c1 = player1_part1.pop_front().unwrap();
        let c2 = player2_part1.pop_front().unwrap();
        combat_round(&mut player1_part1, c1, &mut player2_part1, c2);
    }
    let winner = if player1_part1.is_empty() {
        player2_part1
    } else {
        player1_part1
    };

    let part1 = score(&winner);
    println!("{}", part1);

    let part2_winner = if recursive_combat(&mut player1, &mut player2) {
        player1
    } else {
        player2
    };
    let part2 = score(&part2_winner);
    println!("{}", part2);
}
