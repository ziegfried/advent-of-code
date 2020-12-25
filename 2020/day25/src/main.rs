const DIVIDER: usize = 20201227;
const SUBJECT_NUMBER: usize = 7;

fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % DIVIDER;
    }
    value
}

fn part1(card_pub: usize, door_pub: usize) -> usize {
    let mut card_pk: Option<usize> = None;
    let mut door_pk: Option<usize> = None;
    let mut value = 1;
    let mut loop_size = 1;
    loop {
        value = (value * SUBJECT_NUMBER) % DIVIDER;
        if value == card_pub {
            card_pk = Some(loop_size);
            break;
        }
        if value == door_pub {
            door_pk = Some(loop_size);
            break;
        }
        loop_size += 1;
    }

    if let Some(door_pk) = door_pk {
        transform(card_pub, door_pk)
    } else if let Some(card_pk) = card_pk {
        transform(door_pub, card_pk)
    } else {
        panic!();
    }
}

fn main() {
    println!("Part 1: {}", part1(8987316, 14681524));
}

#[test]
fn test_part1() {
    assert_eq!(part1(5764801, 17807724), 14897079)
}
