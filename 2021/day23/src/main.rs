#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}
use Amphipod::*;

type Hallway = [Option<Amphipod>; 11];
type DestRoom = [Option<Amphipod>; 4];
type DestRooms = [DestRoom; 4];

fn dest_room_to_hallway(dest_room_idx: usize) -> usize {
    match dest_room_idx {
        0 => 2,
        1 => 4,
        2 => 6,
        3 => 8,
        _ => panic!(),
    }
}

fn home_of(dest_room_idx: usize) -> Amphipod {
    match dest_room_idx {
        0 => Amber,
        1 => Bronze,
        2 => Copper,
        3 => Desert,
        _ => panic!(),
    }
}

fn home_for(a: Amphipod) -> usize {
    match a {
        Amber => 0,
        Bronze => 1,
        Copper => 2,
        Desert => 3,
    }
}

fn move_energy(a: Amphipod) -> usize {
    match a {
        Amber => 1,
        Bronze => 10,
        Copper => 100,
        Desert => 1000,
    }
}

fn parse_amphipod(c: char) -> Amphipod {
    match c {
        'A' => Amber,
        'B' => Bronze,
        'C' => Copper,
        'D' => Desert,
        _ => panic!(),
    }
}

fn parse_dest_rooms(input: &str) -> DestRooms {
    let (a1, b1, c1, d1, a2, b2, c2, d2): (char, char, char, char, char, char, char, char) =
        serde_scan::scan!(r"#############
#...........#
###{}#{}#{}#{}###
  #{}#{}#{}#{}#
  #########" <- input)
        .unwrap();

    [
        [
            Some(parse_amphipod(a1)),
            Some(parse_amphipod(a2)),
            Some(Amber),
            Some(Amber),
        ],
        [
            Some(parse_amphipod(b1)),
            Some(parse_amphipod(b2)),
            Some(Bronze),
            Some(Bronze),
        ],
        [
            Some(parse_amphipod(c1)),
            Some(parse_amphipod(c2)),
            Some(Copper),
            Some(Copper),
        ],
        [
            Some(parse_amphipod(d1)),
            Some(parse_amphipod(d2)),
            Some(Desert),
            Some(Desert),
        ],
    ]
}

#[derive(Debug, Clone)]
enum Pos {
    Hallway(usize),
    DestRoom(usize, usize),
}

#[derive(Debug)]
struct Move(Amphipod, Pos, Pos, usize);

fn is_unobstructed(hallway: &Hallway, pos: usize, a: usize, b: usize) -> bool {
    let from = usize::min(a, b);
    let to = usize::max(a, b);
    (from..=to).all(|i| i == pos || hallway[i] == None)
}

fn hallway_dist(a: usize, b: usize) -> usize {
    let from = usize::min(a, b);
    let to = usize::max(a, b);
    to - from
}

fn possible_moves_from_hallway(
    hallway_idx: usize,
    hallway: &Hallway,
    dest_rooms: &DestRooms,
) -> Option<Move> {
    match hallway[hallway_idx] {
        Some(hallway_a) => {
            let new_home = home_for(hallway_a);
            let new_home_hallway_idx = dest_room_to_hallway(new_home);
            if is_unobstructed(hallway, hallway_idx, hallway_idx, new_home_hallway_idx) {
                if dest_rooms[new_home]
                    .iter()
                    .all(|v| v == &None || v == &Some(hallway_a))
                {
                    let (new_home_top, _) = find_top(&dest_rooms[new_home]).unwrap_or((4, Amber));
                    return Some(Move(
                        hallway_a,
                        Pos::Hallway(hallway_idx),
                        Pos::DestRoom(new_home, new_home_top - 1),
                        (move_inout_cost(new_home_top - 1)
                            + hallway_dist(hallway_idx, new_home_hallway_idx))
                            * move_energy(hallway_a),
                    ));
                }
            }
        }
        None => {}
    }
    None
}

fn can_move_to_hallway(room: usize) -> bool {
    room != 2 && room != 4 && room != 6 && room != 8
}

fn find_top(dest_room: &DestRoom) -> Option<(usize, Amphipod)> {
    dest_room
        .iter()
        .enumerate()
        .find(|(_, it)| it.is_some())
        .map(|(idx, a)| (idx, a.unwrap()))
}

fn move_inout_cost(slot: usize) -> usize {
    slot + 1
}

fn possible_moves_from_room(
    room_idx: usize,
    hallway: &Hallway,
    dest_rooms: &DestRooms,
) -> Option<Vec<Move>> {
    let dest_room = dest_rooms[room_idx];
    if let Some((top_slot, top_a)) = find_top(&dest_room) {
        if (top_slot..4).all(|i| dest_room[i] == Some(home_of(room_idx))) {
            return None;
        }
        let mut result: Vec<Move> = vec![];
        let cur_pos = Pos::DestRoom(room_idx, top_slot);
        let new_home = home_for(top_a);
        if new_home != room_idx
            && is_unobstructed(
                hallway,
                99,
                dest_room_to_hallway(room_idx),
                dest_room_to_hallway(new_home),
            )
        {
            if dest_rooms[new_home]
                .iter()
                .all(|v| v == &None || v == &Some(top_a))
            {
                let (new_home_slot, _) = find_top(&dest_rooms[new_home]).unwrap_or((4, Amber));
                result.push(Move(
                    top_a,
                    cur_pos.clone(),
                    Pos::DestRoom(new_home, new_home_slot - 1),
                    (move_inout_cost(top_slot)
                        + move_inout_cost(new_home_slot - 1)
                        + hallway_dist(
                            dest_room_to_hallway(room_idx),
                            dest_room_to_hallway(new_home),
                        ))
                        * move_energy(top_a),
                ))
            }
        }

        for hw in 0..11 {
            if can_move_to_hallway(hw) {
                if is_unobstructed(hallway, 99, dest_room_to_hallway(room_idx), hw) {
                    result.push(Move(
                        top_a,
                        cur_pos.clone(),
                        Pos::Hallway(hw),
                        (move_inout_cost(top_slot)
                            + hallway_dist(dest_room_to_hallway(room_idx), hw))
                            * move_energy(top_a),
                    ))
                }
            }
        }

        if result.len() > 0 {
            return Some(result);
        }
    }

    None
}

fn apply_move(m: &Move, hallway: &Hallway, dest_rooms: &DestRooms) -> (Hallway, DestRooms) {
    let mut hallway = hallway.clone();
    let mut dest_rooms = dest_rooms.clone();
    let Move(a, from, to, _cost) = m;
    match from {
        &Pos::DestRoom(idx, slot) => {
            assert_eq!(&dest_rooms[idx][slot].unwrap(), a);
            dest_rooms[idx][slot] = None;
        }
        &Pos::Hallway(idx) => {
            assert_eq!(&hallway[idx].unwrap(), a);
            hallway[idx] = None;
        }
    }
    match to {
        &Pos::DestRoom(idx, slot) => {
            assert_eq!(dest_rooms[idx][slot], None);
            dest_rooms[idx][slot] = Some(*a);
        }
        &Pos::Hallway(idx) => {
            assert_eq!(hallway[idx], None);
            hallway[idx] = Some(*a);
        }
    }
    (hallway, dest_rooms)
}

fn all_possible_moves(hallway: &Hallway, dest_rooms: &DestRooms) -> Vec<Move> {
    let dr1_moves = (0..4)
        .filter_map(|dr| possible_moves_from_room(dr, hallway, dest_rooms))
        .flat_map(|v| v);
    let hallway_moves =
        (0..11usize).filter_map(|room| possible_moves_from_hallway(room, hallway, dest_rooms));
    dr1_moves.chain(hallway_moves).collect()
}

fn is_complete(_hallway: &Hallway, dest_rooms: &DestRooms) -> bool {
    dest_rooms.iter().enumerate().all(|(idx, dr)| {
        let a = home_of(idx);
        dr.iter().all(|da| da == &Some(a))
    })
}

#[allow(unused)]
fn dbg_amphipod(a: Amphipod) -> char {
    match a {
        Amber => 'A',
        Bronze => 'B',
        Copper => 'C',
        Desert => 'D',
    }
}

#[allow(unused)]
fn dbg_opt_amphipod(a: Option<Amphipod>) -> char {
    match a {
        None => '.',
        Some(a) => dbg_amphipod(a),
    }
}

#[allow(unused)]
fn dbg_state(hallway: &Hallway, dest_rooms: &DestRooms) {
    let hallway = hallway
        .iter()
        .map(|p| match p {
            None => '.',
            Some(a) => dbg_amphipod(*a),
        })
        .collect::<String>();
    let (dr_a, dr_b, dr_c, dr_d) = (
        &dest_rooms[0],
        &dest_rooms[1],
        &dest_rooms[2],
        &dest_rooms[3],
    );
    println!(
        r"#############
#{}#
###{}#{}#{}#{}###
  #{}#{}#{}#{}#
  #{}#{}#{}#{}#
  #{}#{}#{}#{}#
  #########",
        hallway,
        dbg_opt_amphipod(dr_a[0]),
        dbg_opt_amphipod(dr_b[0]),
        dbg_opt_amphipod(dr_c[0]),
        dbg_opt_amphipod(dr_d[0]),
        dbg_opt_amphipod(dr_a[1]),
        dbg_opt_amphipod(dr_b[1]),
        dbg_opt_amphipod(dr_c[1]),
        dbg_opt_amphipod(dr_d[1]),
        dbg_opt_amphipod(dr_a[2]),
        dbg_opt_amphipod(dr_b[2]),
        dbg_opt_amphipod(dr_c[2]),
        dbg_opt_amphipod(dr_d[2]),
        dbg_opt_amphipod(dr_a[3]),
        dbg_opt_amphipod(dr_b[3]),
        dbg_opt_amphipod(dr_c[3]),
        dbg_opt_amphipod(dr_d[3]),
    );
    println!();
}

fn opt_min(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    match (a, b) {
        (None, None) => None,
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (Some(a), Some(b)) => Some(usize::min(a, b)),
    }
}

fn make_moves(
    hallway: &Hallway,
    dest_rooms: &DestRooms,
    cost: usize,
    stop_at: Option<usize>,
) -> Option<usize> {
    if is_complete(hallway, dest_rooms) {
        return Some(cost);
    }
    if let Some(stop_at) = stop_at {
        if cost >= stop_at {
            return None;
        }
    }
    let moves = all_possible_moves(hallway, dest_rooms);
    if moves.len() == 0 {
        return None;
    }
    let mut cur_min = None;
    for m in moves {
        let Move(_, _, _, move_cost) = m;
        let new_cost = cost + move_cost;
        if let Some(cur_min) = opt_min(cur_min, stop_at) {
            if new_cost > cur_min {
                continue;
            }
        }
        let (new_hallway, new_dest_rooms) = apply_move(&m, hallway, dest_rooms);
        let new_min = make_moves(
            &new_hallway,
            &new_dest_rooms,
            new_cost,
            opt_min(cur_min, stop_at),
        );
        cur_min = opt_min(cur_min, new_min);
    }
    cur_min
}

fn part1(input: &str) -> usize {
    let hallway: Hallway = [
        None, None, None, None, None, None, None, None, None, None, None,
    ];
    let dest_rooms = parse_dest_rooms(input);
    make_moves(&hallway, &dest_rooms, 0, None).unwrap()
}

fn part2(input: &str) -> usize {
    let hallway: Hallway = [
        None, None, None, None, None, None, None, None, None, None, None,
    ];
    let mut dest_rooms = parse_dest_rooms(input);

    dest_rooms[0][3] = dest_rooms[0][1];
    dest_rooms[1][3] = dest_rooms[1][1];
    dest_rooms[2][3] = dest_rooms[2][1];
    dest_rooms[3][3] = dest_rooms[3][1];
    dest_rooms[0][1] = Some(Desert);
    dest_rooms[0][2] = Some(Desert);
    dest_rooms[1][1] = Some(Copper);
    dest_rooms[1][2] = Some(Bronze);
    dest_rooms[2][1] = Some(Bronze);
    dest_rooms[2][2] = Some(Amber);
    dest_rooms[3][1] = Some(Amber);
    dest_rooms[3][2] = Some(Copper);

    make_moves(&hallway, &dest_rooms, 0, None).unwrap()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 12521);
}

// #[test]
// fn test_part2() {
//     assert_eq!(part2(include_str!("test.txt")), 44169);
// }
