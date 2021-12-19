#[macro_use]
extern crate serde_scan;
use serde_scan::ScanError;

type TargetArea = ((i32, i32), (i32, i32));

#[derive(Debug)]
struct State {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

#[inline(always)]
fn step(state: State) -> State {
    State {
        x: state.x + state.vx,
        y: state.y + state.vy,
        vx: state.vx - state.vx.signum(),
        vy: state.vy - 1,
    }
}

#[inline(always)]
fn contains(((x_min, x_max), (y_min, y_max)): TargetArea, x: i32, y: i32) -> bool {
    x >= x_min && x <= x_max && y >= y_min && y <= y_max
}

#[inline(always)]
fn out_of_range(((_, x_max), (y_min, _)): TargetArea, x: i32, y: i32, vy: i32) -> bool {
    x > x_max || (y < y_min && vy <= 0)
}

fn check_hit(vx: i32, vy: i32, target: TargetArea) -> Option<i32> {
    let mut state = State { x: 0, y: 0, vx, vy };
    let mut max_y = i32::MIN;
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

fn parts(target: TargetArea) -> (i32, i32) {
    let mut max_y = i32::MIN;
    let mut hit_count = 0;
    for vx in 0..1000 {
        for vy in -500..1000 {
            if let Some(y) = check_hit(vx, vy, target) {
                hit_count += 1;
                if y > max_y {
                    max_y = y;
                }
            }
        }
    }
    (max_y, hit_count)
}

fn parse(input: &str) -> Result<TargetArea, ScanError> {
    let (x_min, x_max, y_min, y_max): (i32, i32, i32, i32) =
        scan!("target area: x={}..{}, y={}..{}" <- input)?;
    Ok(((x_min, x_max), (y_min, y_max)))
}

fn main() {
    let (part1, part2) = parts(parse(include_str!("in.txt")).unwrap());
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[test]
fn test_part1() {
    assert_eq!(parts(parse(include_str!("test.txt")).unwrap()).0, 45);
}

#[test]
fn test_part2() {
    assert_eq!(parts(parse(include_str!("test.txt")).unwrap()).1, 112);
}
