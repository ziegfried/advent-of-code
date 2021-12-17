#[derive(Debug)]
struct State {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

type TargetArea = ((i32, i32), (i32, i32));

fn step(state: State) -> State {
    State {
        x: state.x + state.vx,
        y: state.y + state.vy,
        vx: state.vx + (state.vx.signum() * -1),
        vy: state.vy - 1,
    }
}

fn contains(((x1, x2), (y1, y2)): TargetArea, x: i32, y: i32) -> bool {
    x >= x1 && x <= x2 && y >= y1 && y <= y2
}

fn out_of_range(((_, x2), (y1, _)): TargetArea, x: i32, y: i32, vy: i32) -> bool {
    x > x2 || (y < y1 && vy <= 0)
}

fn check_hit(vx: i32, vy: i32, target: TargetArea) -> Option<i32> {
    let mut state = State { x: 0, y: 0, vx, vy };
    let mut max_y = 0;
    loop {
        state = step(state);
        max_y = i32::max(max_y, state.y);
        if contains(target, state.x, state.y) {
            return Some(max_y);
        }
        if out_of_range(target, state.x, state.y, state.vy) {
            return None;
        }
    }
}

fn part1(target: TargetArea) -> i32 {
    let mut max_y = i32::MIN;
    for x in 0..1000 {
        for y in -500..1000 {
            if let Some(y) = check_hit(x, y, target) {
                if y > max_y {
                    max_y = y;
                }
            }
        }
    }
    max_y
}

fn part2(target: TargetArea) -> i32 {
    let mut hit_count = 0;
    for x in 0..1000 {
        for y in -500..1000 {
            if check_hit(x, y, target).is_some() {
                hit_count += 1;
            }
        }
    }
    hit_count
}

fn main() {
    println!("Part 1: {:?}", part1(((288, 330), (-96, -50))));
    println!("Part 2: {:?}", part2(((288, 330), (-96, -50))));
}

#[test]
fn test_part1() {
    assert_eq!(part1(((20, 30), (-10, -5))), 45);
}

#[test]
fn test_part2() {
    assert_eq!(part2(((20, 30), (-10, -5))), 112);
}
