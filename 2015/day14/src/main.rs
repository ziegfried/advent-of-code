#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    speed: usize,
    fly_duration: usize,
    rest_duration: usize,
}

impl Reindeer {
    pub fn distance_at_time(&self, time: usize) -> usize {
        let period = self.fly_duration + self.rest_duration;
        let remain = time % period;
        let full_periods = (time - remain) / period;
        self.fly_duration * self.speed * full_periods
            + if remain > self.fly_duration {
                self.speed * self.fly_duration
            } else {
                self.speed * remain
            }
    }
}

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(
        r"^(.+?) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$"
    )
    .unwrap();
}

fn parse_line(line: &str) -> Reindeer {
    let m = LINE_RE.captures(line).unwrap();
    Reindeer {
        name: String::from(&m[1]),
        speed: *&m[2].parse::<usize>().unwrap(),
        fly_duration: *&m[3].parse::<usize>().unwrap(),
        rest_duration: *&m[4].parse::<usize>().unwrap(),
    }
}

fn winner_distance(input: &str, time: usize) -> usize {
    let deers = input.split('\n').map(parse_line);
    deers.map(|d| d.distance_at_time(time)).max().unwrap()
}

fn winner_points(input: &str, end_time: usize) -> usize {
    let deers = input.split('\n').map(parse_line).collect::<Vec<Reindeer>>();
    let mut points = HashMap::new();
    for time in 1..end_time {
        let (winner, _) = deers
            .iter()
            .map(|d| (d, d.distance_at_time(time)))
            .max_by(|(_, a), (_, b)| a.cmp(&b))
            .unwrap();
        points.insert(
            &winner.name,
            points.get(&winner.name).or(Some(&0)).unwrap() + 1,
        );
    }
    *points.values().max().unwrap()
}

fn main() {
    println!("Part 1: {}", winner_distance(include_str!("in.txt"), 2503));
    println!("Part 2: {}", winner_points(include_str!("in.txt"), 2503));
}

#[test]
fn test_parse_line() {
    assert_eq!(
        parse_line("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."),
        Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            fly_duration: 10,
            rest_duration: 127,
        }
    )
}

#[test]
fn test_part1() {
    assert_eq!(winner_distance(include_str!("test.txt"), 1000), 1120);
}

#[test]
fn test_part2() {
    assert_eq!(winner_points(include_str!("test.txt"), 1000), 689);
}
