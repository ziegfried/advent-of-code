use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}
use Amphipod::*;

type Hallway = [Option<Amphipod>; 11];
type SideRoom = [Option<Amphipod>; 4];
type SideRooms = [SideRoom; 4];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    hallway: Hallway,
    side_rooms: SideRooms,
}

#[derive(Debug, Clone)]
enum Pos {
    Hallway(usize),
    SideRoom(usize, usize),
}

#[derive(Debug)]
struct Move(Amphipod, Pos, Pos, usize);

fn parse_amphipod(c: char) -> Amphipod {
    match c {
        'A' => Amber,
        'B' => Bronze,
        'C' => Copper,
        'D' => Desert,
        _ => panic!(),
    }
}

fn parse_side_rooms(input: &str) -> SideRooms {
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
fn dbg_state(hallway: &Hallway, side_rooms: &SideRooms) {
    let hallway = hallway
        .iter()
        .map(|p| match p {
            None => '.',
            Some(a) => dbg_amphipod(*a),
        })
        .collect::<String>();
    let (sr1, sr2, sr3, sr4) = (
        &side_rooms[0],
        &side_rooms[1],
        &side_rooms[2],
        &side_rooms[3],
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
        dbg_opt_amphipod(sr1[0]),
        dbg_opt_amphipod(sr2[0]),
        dbg_opt_amphipod(sr3[0]),
        dbg_opt_amphipod(sr4[0]),
        dbg_opt_amphipod(sr1[1]),
        dbg_opt_amphipod(sr2[1]),
        dbg_opt_amphipod(sr3[1]),
        dbg_opt_amphipod(sr4[1]),
        dbg_opt_amphipod(sr1[2]),
        dbg_opt_amphipod(sr2[2]),
        dbg_opt_amphipod(sr3[2]),
        dbg_opt_amphipod(sr4[2]),
        dbg_opt_amphipod(sr1[3]),
        dbg_opt_amphipod(sr2[3]),
        dbg_opt_amphipod(sr3[3]),
        dbg_opt_amphipod(sr4[3]),
    );
    println!();
}

fn side_room_to_hallway(side_room_idx: usize) -> usize {
    match side_room_idx {
        0 => 2,
        1 => 4,
        2 => 6,
        3 => 8,
        _ => panic!(),
    }
}

fn home_of(side_room_idx: usize) -> Amphipod {
    match side_room_idx {
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

fn is_unobstructed(hallway: &Hallway, pos: Option<usize>, a: usize, b: usize) -> bool {
    let from = usize::min(a, b);
    let to = usize::max(a, b);
    (from..=to).all(|i| Some(i) == pos || hallway[i] == None)
}

fn hallway_distance(a: usize, b: usize) -> usize {
    usize::max(a, b) - usize::min(a, b)
}

fn can_move_to_hallway(idx: usize) -> bool {
    idx != 2 && idx != 4 && idx != 6 && idx != 8
}

fn possible_move_from_hallway(hallway_idx: usize, state: &State) -> Option<Move> {
    match state.hallway[hallway_idx] {
        Some(hallway_a) => {
            let new_home = home_for(hallway_a);
            let new_home_hallway_idx = side_room_to_hallway(new_home);
            if is_unobstructed(
                &state.hallway,
                Some(hallway_idx),
                hallway_idx,
                new_home_hallway_idx,
            ) {
                if state.side_rooms[new_home]
                    .iter()
                    .all(|v| v == &None || v == &Some(hallway_a))
                {
                    let new_home_top = find_open_slot(&state.side_rooms[new_home]);
                    return Some(Move(
                        hallway_a,
                        Pos::Hallway(hallway_idx),
                        Pos::SideRoom(new_home, new_home_top),
                        (move_inout_cost(new_home_top)
                            + hallway_distance(hallway_idx, new_home_hallway_idx))
                            * move_energy(hallway_a),
                    ));
                }
            }
        }
        None => {}
    }
    None
}

fn find_top(side_room: &SideRoom) -> Option<(usize, Amphipod)> {
    side_room
        .iter()
        .enumerate()
        .find(|(_, it)| it.is_some())
        .map(|(idx, a)| (idx, a.unwrap()))
}

fn find_open_slot(side_room: &SideRoom) -> usize {
    for i in (0..4).rev() {
        if side_room[i] == None {
            return i;
        }
    }
    panic!();
}

fn move_inout_cost(slot: usize) -> usize {
    slot + 1
}

fn possible_moves_from_side_room(side_room_idx: usize, state: &State) -> Option<Vec<Move>> {
    let side_room = state.side_rooms[side_room_idx];
    if let Some((top_slot, top_a)) = find_top(&side_room) {
        if (top_slot..4).all(|i| side_room[i] == Some(home_of(side_room_idx))) {
            return None;
        }
        let mut result: Vec<Move> = vec![];
        let cur_pos = Pos::SideRoom(side_room_idx, top_slot);
        let new_home = home_for(top_a);
        if new_home != side_room_idx
            && is_unobstructed(
                &state.hallway,
                None,
                side_room_to_hallway(side_room_idx),
                side_room_to_hallway(new_home),
            )
        {
            if state.side_rooms[new_home]
                .iter()
                .all(|v| v == &None || v == &Some(top_a))
            {
                let new_home_slot = find_open_slot(&state.side_rooms[new_home]);
                result.push(Move(
                    top_a,
                    cur_pos.clone(),
                    Pos::SideRoom(new_home, new_home_slot),
                    (move_inout_cost(top_slot)
                        + move_inout_cost(new_home_slot)
                        + hallway_distance(
                            side_room_to_hallway(side_room_idx),
                            side_room_to_hallway(new_home),
                        ))
                        * move_energy(top_a),
                ))
            }
        }
        for hw_idx in 0..11 {
            if can_move_to_hallway(hw_idx) {
                if is_unobstructed(
                    &state.hallway,
                    None,
                    side_room_to_hallway(side_room_idx),
                    hw_idx,
                ) {
                    result.push(Move(
                        top_a,
                        cur_pos.clone(),
                        Pos::Hallway(hw_idx),
                        (move_inout_cost(top_slot)
                            + hallway_distance(side_room_to_hallway(side_room_idx), hw_idx))
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

fn all_possible_moves(state: &State) -> Vec<Move> {
    state
        .side_rooms
        .iter()
        .enumerate()
        .filter_map(|(sr_idx, _)| possible_moves_from_side_room(sr_idx, state))
        .flat_map(|v| v)
        .chain(
            state
                .hallway
                .iter()
                .enumerate()
                .filter_map(|(hw_idx, _)| possible_move_from_hallway(hw_idx, state)),
        )
        .collect()
}

fn apply_move(m: &Move, state: &State) -> State {
    let mut hallway = state.hallway.clone();
    let mut side_rooms = state.side_rooms.clone();
    let Move(a, from, to, _cost) = m;
    match from {
        &Pos::SideRoom(idx, slot) => {
            assert_eq!(&side_rooms[idx][slot].unwrap(), a);
            side_rooms[idx][slot] = None;
        }
        &Pos::Hallway(idx) => {
            assert_eq!(&hallway[idx].unwrap(), a);
            hallway[idx] = None;
        }
    }
    match to {
        &Pos::SideRoom(idx, slot) => {
            assert_eq!(side_rooms[idx][slot], None);
            side_rooms[idx][slot] = Some(*a);
        }
        &Pos::Hallway(idx) => {
            assert_eq!(hallway[idx], None);
            hallway[idx] = Some(*a);
        }
    }
    State {
        hallway,
        side_rooms,
    }
}

fn is_complete(state: &State) -> bool {
    state.side_rooms.iter().enumerate().all(|(idx, dr)| {
        let a = home_of(idx);
        dr.iter().all(|da| da == &Some(a))
    })
}

fn make_moves(state: &State, memo: &mut HashMap<State, Option<usize>>) -> Option<usize> {
    if is_complete(state) {
        return Some(0);
    }
    if let Some(memoized_result) = memo.get(state) {
        return *memoized_result;
    }
    let result = {
        all_possible_moves(state)
            .iter()
            .filter_map(|m| {
                let Move(_, _, _, move_cost) = m;
                let new_state = apply_move(m, state);
                make_moves(&new_state, memo).map(|sub_cost| move_cost + sub_cost)
            })
            .min()
    };
    memo.insert(state.clone(), result);
    result
}

fn part1(input: &str) -> usize {
    let hallway: Hallway = Default::default();
    let side_rooms = parse_side_rooms(input);
    let state = State {
        hallway,
        side_rooms,
    };
    make_moves(&state, &mut HashMap::new()).unwrap()
}

fn part2(input: &str) -> usize {
    let hallway: Hallway = Default::default();
    let mut side_rooms = parse_side_rooms(input);
    side_rooms[0][3] = side_rooms[0][1];
    side_rooms[1][3] = side_rooms[1][1];
    side_rooms[2][3] = side_rooms[2][1];
    side_rooms[3][3] = side_rooms[3][1];
    side_rooms[0][1] = Some(Desert);
    side_rooms[0][2] = Some(Desert);
    side_rooms[1][1] = Some(Copper);
    side_rooms[1][2] = Some(Bronze);
    side_rooms[2][1] = Some(Bronze);
    side_rooms[2][2] = Some(Amber);
    side_rooms[3][1] = Some(Amber);
    side_rooms[3][2] = Some(Copper);
    let state = State {
        hallway,
        side_rooms,
    };
    make_moves(&state, &mut HashMap::new()).unwrap()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test.txt")), 12521);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test.txt")), 44169);
}
