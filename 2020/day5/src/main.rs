use std::ops::Range;

enum Half {
    Upper,
    Lower,
}

use Half::*;

trait Halfable<T> {
    fn half(&self, which: Half) -> Self;
}

impl Halfable<usize> for Range<usize> {
    fn half(&self, which: Half) -> Range<usize> {
        let mid = self.start + (self.end - self.start) / 2;
        match which {
            Upper => Range {
                start: mid,
                end: self.end,
            },
            Lower => Range {
                start: self.start,
                end: mid,
            },
        }
    }
}

fn parse_row(s: &str) -> usize {
    let r = s.chars().fold(0..127, |p, c| match c {
        'B' => p.half(Upper),
        'F' => p.half(Lower),
        _ => panic!("nah"),
    });
    // 1-based
    r.end
}

fn parse_seat(s: &str) -> usize {
    let r = s.chars().fold(0..8, |p, c| match c {
        'R' => p.half(Upper),
        'L' => p.half(Lower),
        _ => panic!("nah"),
    });
    // 0 based
    r.start
}

fn parse_line(line: &str) -> (usize, usize, usize) {
    let row = parse_row(&line[0..7]);
    let seat = parse_seat(&line[7..10]);
    let seat_id = row * 8 + seat;
    (row, seat, seat_id)
}

fn main() {
    let results = include_str!("../in.txt")
        .split('\n')
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    let mut ids: Vec<usize> = results
        .iter()
        .filter(|(row, _seat, _id)| *row > 1 && *row < 128)
        .map(|(_a, _b, id)| *id)
        .collect();
    ids.sort();

    println!("Part 1: Seat {}", ids.get(ids.len() - 1).unwrap());

    if let Some(w) = ids.windows(2).find(|w| w[1] - w[0] > 1) {
        println!("Part 2: Seat {}", w[0] + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halfs() {
        assert_eq!((0..128).half(Upper), 64..128);
        assert_eq!((0..128).half(Lower), 0..64);
    }

    #[test]
    fn test_parse_row() {
        assert_eq!(parse_row("FBFBBFF"), 44);
        assert_eq!(parse_seat("RLR"), 5);
    }
}
