#[derive(Debug, Clone)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
enum TurnDir {
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Instruction {
    Move(Dir, usize),
    Turn(TurnDir, usize),
    Forward(usize),
}

#[derive(Debug, Clone)]
struct Pos(isize, isize);

fn parse_instruction(s: &str) -> Instruction {
    let n = s[1..].parse::<usize>().unwrap();
    match s.chars().next().unwrap() {
        'N' => Instruction::Move(Dir::North, n),
        'S' => Instruction::Move(Dir::South, n),
        'E' => Instruction::Move(Dir::East, n),
        'W' => Instruction::Move(Dir::West, n),
        'L' => Instruction::Turn(TurnDir::Left, n),
        'R' => Instruction::Turn(TurnDir::Right, n),
        'F' => Instruction::Forward(n),
        _ => panic!(format!("invalid instruction {:?}", s)),
    }
}

fn deg_to_right_turn_count(deg: usize, turn: TurnDir) -> usize {
    use TurnDir::*;
    (match turn {
        Left => 360 - (deg % 360),
        Right => deg % 360,
    }) / 90
}

fn turn_right_once(dir: Dir) -> Dir {
    use Dir::*;
    match dir {
        North => East,
        East => South,
        South => West,
        West => North,
    }
}

fn turn(dir: Dir, deg: usize, turn: TurnDir) -> Dir {
    (0..(deg_to_right_turn_count(deg, turn))).fold(dir, |cur, _| turn_right_once(cur))
}

fn rotate_waypoint(waypoint: &Pos, deg: usize, turn: TurnDir) -> Pos {
    (0..(deg_to_right_turn_count(deg, turn))).fold(waypoint.clone(), |Pos(x, y), _| Pos(y * -1, x))
}

fn apply_dir(pos: &Pos, dir: &Dir, amount: isize) -> Pos {
    use Dir::*;
    match dir {
        North => Pos(pos.0 + amount, pos.1),
        East => Pos(pos.0, pos.1 + amount),
        South => Pos(pos.0 - amount, pos.1),
        West => Pos(pos.0, pos.1 - amount),
    }
}

fn apply_instruction(pos: &Pos, cur_dir: &Dir, inst: &Instruction) -> (Pos, Dir) {
    use Instruction::*;
    match inst {
        Move(move_dir, n) => (apply_dir(&pos, move_dir, *n as isize), cur_dir.clone()),
        Forward(n) => (apply_dir(&pos, cur_dir, *n as isize), cur_dir.clone()),
        Turn(turn_dir, deg) => (pos.clone(), turn(cur_dir.clone(), *deg, turn_dir.clone())),
    }
}

struct ShipState {
    pos: Pos,
    waypoint: Pos,
}

impl ShipState {
    pub fn apply_instruction(&mut self, inst: &Instruction) {
        use Instruction::*;
        match inst {
            Move(dir, n) => {
                self.waypoint = apply_dir(&self.waypoint, dir, *n as isize);
            }
            Forward(n) => {
                let Pos(dx, dy) = self.waypoint;
                self.pos = (0..(*n)).fold(self.pos.clone(), |Pos(x, y), _| Pos(x + dx, y + dy));
            }
            Turn(turn_dir, deg) => {
                self.waypoint = rotate_waypoint(&self.waypoint, *deg, turn_dir.clone());
            }
        }
    }
}

fn part1(inputs: &Vec<Instruction>) -> isize {
    let (final_pos, _) = inputs
        .iter()
        .fold((Pos(0, 0), Dir::East), |(pos, dir), inst| {
            apply_instruction(&pos, &dir, inst)
        });

    let Pos(x, y) = final_pos;
    x.abs() + y.abs()
}

fn part2(inputs: &Vec<Instruction>) -> isize {
    let mut ship = ShipState {
        pos: Pos(0, 0),
        waypoint: Pos(1, 10),
    };

    for inst in inputs {
        &ship.apply_instruction(inst);
    }
    let Pos(x, y) = ship.pos;
    x.abs() + y.abs()
}

fn main() {
    let inputs = include_str!("in.txt")
        .split('\n')
        .map(parse_instruction)
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                &include_str!("test.txt")
                    .split('\n')
                    .map(parse_instruction)
                    .collect::<Vec<_>>()
            ),
            25
        );
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                &include_str!("test.txt")
                    .split('\n')
                    .map(parse_instruction)
                    .collect::<Vec<_>>()
            ),
            286
        );
    }
}
