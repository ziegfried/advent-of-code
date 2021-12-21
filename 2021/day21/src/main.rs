use std::collections::HashMap;

fn parse(input: &str) -> (u32, u32) {
    let (l1, l2) = input.trim().split_once("\n").unwrap();
    (
        l1.strip_prefix("Player 1 starting position: ")
            .unwrap()
            .parse::<u32>()
            .unwrap(),
        l2.strip_prefix("Player 2 starting position: ")
            .unwrap()
            .parse::<u32>()
            .unwrap(),
    )
}

#[derive(Debug)]
struct Player {
    n: u32,
    pos: u32,
    score: u32,
}

fn part1(input: &str) -> u32 {
    let (p1, p2) = parse(input);
    let mut die = (1..=100).cycle();
    let mut p1 = Player {
        n: 1,
        pos: p1,
        score: 0,
    };
    let mut p2 = Player {
        n: 2,
        pos: p2,
        score: 0,
    };
    let winner;
    let mut rolled = 0;
    'outer: loop {
        for player in [&mut p1, &mut p2] {
            let amount = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
            rolled += 3;
            player.pos = (player.pos - 1 + amount) % 10 + 1;
            player.score += player.pos;
            if player.score >= 1000 {
                winner = player.n;
                break 'outer;
            }
        }
    }
    let loser = if winner == 1 { p2 } else { p1 };
    loser.score * rolled
}

fn count_wins(
    p1: (u64, u64),
    p2: (u64, u64),
    p1_turn: bool,
    memo: &mut HashMap<((u64, u64), (u64, u64), bool), (u64, u64)>,
) -> (u64, u64) {
    if let Some(result) = memo.get(&(p1, p2, p1_turn)) {
        return *result;
    }
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for roll1 in 1..=3 {
        for roll2 in 1..=3 {
            for roll3 in 1..=3 {
                let amount = roll1 + roll2 + roll3;
                let (pos, score) = if p1_turn { p1 } else { p2 };
                let next_pos = (pos - 1 + amount) % 10 + 1;
                let new_score = score + next_pos;
                if new_score >= 21 {
                    if p1_turn {
                        p1_wins += 1;
                    } else {
                        p2_wins += 1;
                    }
                } else {
                    let (w1, w2) = count_wins(
                        if p1_turn { (next_pos, new_score) } else { p1 },
                        if p1_turn { p2 } else { (next_pos, new_score) },
                        !p1_turn,
                        memo,
                    );
                    p1_wins += w1;
                    p2_wins += w2;
                }
            }
        }
    }
    memo.insert((p1, p2, p1_turn), (p1_wins, p2_wins));
    (p1_wins, p2_wins)
}

fn part2(input: &str) -> u64 {
    let (p1, p2) = parse(input);
    let (p1wins, p2wins) = count_wins((p1 as u64, 0), (p2 as u64, 0), true, &mut HashMap::new());
    if p1wins >= p2wins {
        p1wins
    } else {
        p2wins
    }
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 739785);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 444356092776315);
}
