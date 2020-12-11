#[macro_use]
extern crate lazy_static;

#[derive(Debug, PartialEq, Clone)]
enum SeatState {
    Floor,
    Empty,
    Occupied,
}

use SeatState::*;

lazy_static! {
    static ref DIRECTIONS: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
}

#[derive(Debug)]
struct SeatMap {
    pub seats: Vec<SeatState>,
    pub next_seats: Vec<SeatState>,
    pub row_count: usize,
    pub row_len: usize,
}

impl SeatMap {
    pub fn new(s: &str) -> SeatMap {
        let input = s.split('\n').collect::<Vec<_>>();
        let row_len = &input[0].len();
        let row_count = &input.len();
        let seats = input
            .iter()
            .flat_map(|row| {
                row.chars().map(|c| match c {
                    'L' => Empty,
                    '.' => Floor,
                    '#' => Occupied,
                    _ => panic!(),
                })
            })
            .collect::<Vec<SeatState>>();
        SeatMap {
            seats: seats,
            next_seats: Vec::with_capacity(row_count * row_len),
            row_count: *row_count,
            row_len: *row_len,
        }
    }

    pub fn get(&self, (row, col): (usize, usize)) -> &SeatState {
        self.seats.get(self.row_len * row + col).unwrap()
    }

    pub fn neighbors(&self, (row, col): (usize, usize)) -> Vec<&SeatState> {
        let res = DIRECTIONS
            .iter()
            .filter(|(dx, dy)| {
                row as isize + dx >= 0
                    && row as isize + dx < self.row_count as isize
                    && col as isize + dy >= 0
                    && col as isize + dy < self.row_len as isize
            })
            .map(|(dx, dy)| self.get(((row as isize + dx) as usize, (col as isize + dy) as usize)))
            .collect::<Vec<&SeatState>>();
        res
    }

    pub fn visible_seat(
        &self,
        (row, col): (usize, usize),
        (dx, dy): (isize, isize),
    ) -> Option<SeatState> {
        let mut cur_row = row as isize;
        let mut cur_col = col as isize;
        loop {
            cur_row = cur_row + dx;
            cur_col = cur_col + dy;
            if cur_row >= 0
                && cur_col >= 0
                && cur_row < self.row_count as isize
                && cur_col < self.row_len as isize
            {
                match self.get((cur_row as usize, cur_col as usize)) {
                    Occupied => {
                        return Some(Occupied);
                    }
                    Empty => {
                        return Some(Empty);
                    }
                    Floor => {
                        // noop
                    }
                };
            } else {
                return None;
            }
        }
    }

    pub fn visible_seats(&self, pos: (usize, usize)) -> Vec<SeatState> {
        DIRECTIONS
            .iter()
            .filter_map(|dir| self.visible_seat(pos, *dir))
            .collect::<Vec<SeatState>>()
    }

    fn calc_next(&self) -> Vec<SeatState> {
        (0..self.row_count)
            .flat_map(|row| {
                (0..self.row_len).map(move |col| {
                    let pos = (row, col);
                    let cur = self.get(pos);
                    match cur {
                        Floor => Floor,
                        Empty => {
                            if self
                                .neighbors(pos)
                                .iter()
                                .all(|s| **s == Empty || **s == Floor)
                            {
                                Occupied
                            } else {
                                Empty
                            }
                        }
                        Occupied => {
                            if self
                                .neighbors(pos)
                                .iter()
                                .filter(|s| ***s == Occupied)
                                .count()
                                >= 4
                            {
                                Empty
                            } else {
                                Occupied
                            }
                        }
                    }
                })
            })
            .collect::<Vec<SeatState>>()
    }

    fn calc_next2(&self) -> Vec<SeatState> {
        (0..self.row_count)
            .flat_map(|row| {
                (0..self.row_len).map(move |col| {
                    let pos = (row, col);
                    let cur = self.get(pos);
                    match cur {
                        Floor => Floor,
                        Empty => {
                            if self
                                .visible_seats(pos)
                                .iter()
                                .all(|s| *s == Empty || *s == Floor)
                            {
                                Occupied
                            } else {
                                Empty
                            }
                        }
                        Occupied => {
                            if self
                                .visible_seats(pos)
                                .iter()
                                .filter(|s| **s == Occupied)
                                .count()
                                >= 5
                            {
                                Empty
                            } else {
                                Occupied
                            }
                        }
                    }
                })
            })
            .collect::<Vec<SeatState>>()
    }

    pub fn apply_rules(&mut self) -> bool {
        let next = self.calc_next();
        if self.seats != next {
            self.seats = next;
            true
        } else {
            false
        }
    }

    pub fn apply_rules_part2(&mut self) -> bool {
        let next = self.calc_next2();
        if self.seats != next {
            self.seats = next;
            true
        } else {
            false
        }
    }

    pub fn occupied_count(&self) -> usize {
        (0..self.row_count)
            .flat_map(|row| (0..self.row_len).map(move |col| self.get((row, col))))
            .filter(|s| **s == Occupied)
            .count()
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for row in 0..self.row_count {
            let s: String = (0..self.row_len)
                .map(|col| self.get((row, col)))
                .map(|state| match state {
                    Occupied => '#',
                    Empty => 'L',
                    Floor => '.',
                })
                .collect();
            println!("{}", s);
        }
    }
}

fn main() {
    let input = include_str!("../in.txt");
    let mut seat_map1 = SeatMap::new(input);
    loop {
        if !&seat_map1.apply_rules() {
            break;
        }
    }
    println!("Part 1: {}", &seat_map1.occupied_count());

    let mut seat_map2 = SeatMap::new(input);
    loop {
        if !&seat_map2.apply_rules_part2() {
            break;
        }
    }

    println!("Part 2: {}", &seat_map2.occupied_count());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_visible1() {
        let seat_map = SeatMap::new(include_str!("../test_visible1.txt"));
        assert_eq!(seat_map.visible_seats((3, 3)), vec![]);
    }
}
