use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
    f64::consts::PI,
    hash::{Hash, Hasher},
};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn point(x: i32, y: i32) -> Point {
    Point { x: x, y: y }
}

#[derive(PartialEq, PartialOrd, Debug)]
struct Angle(f64);
impl Hash for Angle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{}", self.0).hash(state);
    }
}
impl Eq for Angle {}
impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Less)
    }
}

fn angle(a: Point, b: Point) -> Angle {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    let a = dy.atan2(dx) - (PI / 2_f64);
    Angle(if a < 0_f64 { 2_f64 * PI + a } else { a })
}

fn distance(a: Point, b: Point) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    (dx.powi(2) + dy.powi(2)).sqrt()
}

fn parse_asteroid_map(map: &str) -> Vec<Point> {
    let lines = map.split('\n').collect::<Vec<_>>();
    let lines_ref = &lines;
    let line_len = lines.get(0).unwrap().len();
    (0..lines.len())
        .flat_map(|y| {
            (0..line_len).filter_map(move |x| {
                if lines_ref.get(y).unwrap().chars().nth(x).unwrap() == '#' {
                    Some(point(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<Point>>()
}

fn number_of_asteroids_in_sight(asteroid: Point, all_asteroids: &Vec<Point>) -> u32 {
    let mut angles = HashSet::<Angle>::new();
    for candidate in all_asteroids {
        if *candidate != asteroid {
            angles.insert(angle(asteroid, candidate.clone()));
        }
    }
    angles.len() as u32
}

fn best_location(asteroids: &Vec<Point>) -> (u32, Point) {
    asteroids
        .clone()
        .iter()
        .map(|asteroid| {
            (
                number_of_asteroids_in_sight(asteroid.clone(), &asteroids),
                asteroid.clone(),
            )
        })
        .max_by(|(a, _), (b, _)| a.cmp(b))
        .unwrap()
}

fn part1(input: &str) -> u32 {
    let asteroids = parse_asteroid_map(input);
    best_location(&asteroids).0
}

fn part2(input: &str) -> i32 {
    let asteroids = parse_asteroid_map(input);
    let (_, monitoring_station) = best_location(&asteroids);
    let mut angle_map = BTreeMap::<Angle, Vec<Point>>::new();
    for asteroid in asteroids {
        if asteroid != monitoring_station {
            let angle = angle(monitoring_station, asteroid);
            if let Some(vec) = angle_map.get_mut(&angle) {
                vec.push(asteroid.clone());
            } else {
                angle_map.insert(angle, vec![asteroid.clone()]);
            }
        }
    }
    for (_, points) in angle_map.iter_mut() {
        points.sort_by(|a, b| {
            distance(monitoring_station, *a)
                .partial_cmp(&distance(monitoring_station, *b))
                .unwrap()
        });
    }
    let mut i = 0;
    loop {
        let mut nuked = 0;
        for (_, points) in angle_map.iter_mut() {
            if points.len() > 0 {
                let point = points.remove(0);
                i += 1;
                if i == 200 {
                    return 100 * point.x + point.y;
                }
                nuked += 1;
            }
        }
        if nuked == 0 {
            break;
        }
    }
    panic!("did not find 200th asteroid to nuke")
}

fn main() {
    println!("{}", part1(include_str!("in.txt")));
    println!("{}", part2(include_str!("in.txt")));
}

#[test]
fn test_distance_between_points() {
    assert_eq!(distance(point(4, 4), point(4, 3)), 1_f64);
}

#[test]
fn test_angle() {
    assert_eq!(
        angle(point(3, 4), point(1, 0)),
        angle(point(3, 4), point(2, 2))
    );
    assert_eq!(
        angle(point(4, 4), point(4, 0)),
        angle(point(4, 4), point(4, 3))
    );
    assert_eq!(angle(point(4, 4), point(4, 0)), Angle(0_f64));
    assert_eq!(angle(point(4, 4), point(4, 8)), Angle(PI));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test1.txt")), 8);
    assert_eq!(part1(include_str!("test2.txt")), 33);
    assert_eq!(part1(include_str!("test3.txt")), 210);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test3.txt")), 802);
}
