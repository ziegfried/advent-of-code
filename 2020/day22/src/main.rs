use std::collections::{HashMap, HashSet};

fn parse_player(s: &str) -> Vec<usize> {
    s.lines()
        .skip(1)
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn score(cards: &Vec<usize>) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * c)
        .sum()
}

fn part1(input: &str) -> usize {
    let mut players = input.split("\n\n").map(parse_player);
    let mut player1 = players.next().unwrap();
    let mut player2 = players.next().unwrap();
    let winner;
    loop {
        let a = &player1.remove(0);
        let b = &player2.remove(0);

        if a > b {
            &player1.push(*a);
            &player1.push(*b);
        } else {
            &player2.push(*b);
            &player2.push(*a);
        }

        if &player1.len() == &0 {
            winner = Some(player2);
            break;
        }
        if &player2.len() == &0 {
            winner = Some(player1);
            break;
        }
    }
    score(&winner.unwrap())
}

fn sub_game(
    player1_input: &Vec<usize>,
    player2_input: &Vec<usize>,
    memo: &mut HashMap<(Vec<usize>, Vec<usize>), (bool, usize)>,
) -> (bool, usize) {
    if let Some((b, score)) = memo.get(&(player1_input.clone(), player2_input.clone())) {
        return (*b, *score);
    }
    let mut player1 = player1_input.clone();
    let mut player2 = player2_input.clone();
    let mut player1_prev_decks = HashSet::new();
    let mut player2_prev_decks = HashSet::new();
    let winner_player1;
    let winning_score: usize;
    loop {
        if player1_prev_decks.contains(&player1) || player2_prev_decks.contains(&player2) {
            winner_player1 = true;
            winning_score = score(&player1);
            break;
        }
        &player1_prev_decks.insert(player1.clone());
        &player2_prev_decks.insert(player2.clone());
        let a = &player1.remove(0);
        let b = &player2.remove(0);
        let player1_wins = if a <= &player1.len() && b <= &player2.len() {
            let (one_wins, _) = sub_game(&player1[0..*a].to_vec(), &player2[0..*b].to_vec(), memo);
            one_wins
        } else {
            a > b
        };
        if player1_wins {
            &player1.push(*a);
            &player1.push(*b);
        } else {
            &player2.push(*b);
            &player2.push(*a);
        }
        if &player1.len() == &0 {
            winning_score = score(&player2);
            winner_player1 = false;
            break;
        }
        if &player2.len() == &0 {
            winning_score = score(&player1);
            winner_player1 = true;
            break;
        }
    }
    let result = (winner_player1, winning_score);
    memo.insert((player1_input.clone(), player2_input.clone()), result);
    result
}

fn part2(input: &str) -> usize {
    let mut players = input.split("\n\n").map(parse_player);
    let player1 = players.next().unwrap();
    let player2 = players.next().unwrap();
    let (_, winner_score) = sub_game(&player1, &player2, &mut HashMap::new());
    winner_score
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part() {
    assert_eq!(part1(include_str!("test.txt")), 306);
}

#[test]
fn test_par2() {
    assert_eq!(part2(include_str!("test2.txt")), 105);
    assert_eq!(part2(include_str!("test.txt")), 291);
}
