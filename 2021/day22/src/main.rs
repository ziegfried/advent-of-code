use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Box {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
    state: State,
}
impl Box {
    fn from_str(input: &str) -> Self {
        let (state, x1, x2, y1, y2, z1, z2): (&str, i32, i32, i32, i32, i32, i32) =
            serde_scan::scan!("{} x={}..{},y={}..{},z={}..{}" <- input).unwrap();
        Self::new(
            (x1, x2 + 1),
            (y1, y2 + 1),
            (z1, z2 + 1),
            match state {
                "on" => State::On,
                "off" => State::Off,
                _ => panic!(),
            },
        )
    }
    fn new(x: (i32, i32), y: (i32, i32), z: (i32, i32), state: State) -> Self {
        assert!(x.0 <= x.1, "{} < {}", x.0, x.1);
        assert!(y.0 <= y.1, "{} < {}", x.0, x.1);
        assert!(z.0 <= z.1, "{} < {}", x.0, x.1);
        Self { x, y, z, state }
    }
    fn contains(&self, other: &Box) -> bool {
        self.x.0 <= other.x.0
            && self.x.1 >= other.x.1
            && self.y.0 <= other.y.0
            && self.y.1 >= other.y.1
            && self.z.0 <= other.z.0
            && self.z.1 >= other.z.1
    }
    fn intersects(&self, other: &Box) -> bool {
        self.x.1 > other.x.0
            && self.x.0 < other.x.1
            && self.y.1 > other.y.0
            && self.y.0 < other.y.1
            && self.z.1 > other.z.0
            && self.z.0 < other.z.1
    }
    fn size(&self) -> usize {
        (self.x.1 - self.x.0) as usize
            * (self.y.1 - self.y.0) as usize
            * (self.z.1 - self.z.0) as usize
    }
}

fn cap_at(b: &Box, cap: &Box) -> Option<Box> {
    if b.intersects(cap) {
        Some(Box::new(
            (i32::max(cap.x.0, b.x.0), i32::min(cap.x.1, b.x.1)),
            (i32::max(cap.y.0, b.y.0), i32::min(cap.y.1, b.y.1)),
            (i32::max(cap.z.0, b.z.0), i32::min(cap.z.1, b.z.1)),
            b.state,
        ))
    } else {
        None
    }
}

fn part1(input: &str) -> usize {
    let init = Box::new((-50, 51), (-50, 51), (-50, 51), State::On);
    let mut boxes: Vec<Box> = vec![];
    for line in input.lines() {
        if let Some(new_box) = cap_at(&Box::from_str(line), &init) {
            boxes = boxes
                .iter()
                .flat_map(|existing| split_existing(existing, &new_box))
                .filter(|b| b.state == State::On)
                .collect();
            if new_box.state == State::On {
                boxes.push(new_box);
            }
        }
    }
    boxes.iter().map(|b| b.size()).sum()
}

fn split_existing(existing: &Box, new_box: &Box) -> Vec<Box> {
    if new_box.contains(existing) {
        return vec![];
    }
    if !new_box.intersects(existing) {
        return vec![existing.clone()];
    }
    let mut result = vec![];
    fn axis_split_points((p1, p2): (i32, i32), (a, b): (i32, i32)) -> Vec<i32> {
        let mut result = vec![p1];
        if a > p1 && a < p2 {
            result.push(a);
        }
        if b > p1 && b < p2 {
            result.push(b);
        }
        result.push(p2);
        result
    }
    for (x1, x2) in axis_split_points(existing.x, new_box.x)
        .iter()
        .tuple_windows()
    {
        for (y1, y2) in axis_split_points(existing.y, new_box.y)
            .iter()
            .tuple_windows()
        {
            for (z1, z2) in axis_split_points(existing.z, new_box.z)
                .iter()
                .tuple_windows()
            {
                let (x, y, z) = ((*x1, *x2), (*y1, *y2), (*z1, *z2));
                let bn = Box::new(x, y, z, State::On);
                if !new_box.contains(&bn) {
                    result.push(bn);
                }
            }
        }
    }
    result
}

fn part2(input: &str) -> usize {
    let mut boxes: Vec<Box> = vec![];
    for line in input.lines() {
        let new_box = Box::from_str(line);
        boxes = boxes
            .iter()
            .flat_map(|existing| split_existing(existing, &new_box))
            .filter(|b| b.state == State::On)
            .collect();
        if new_box.state == State::On {
            boxes.push(new_box);
        }
    }
    boxes.iter().map(|b| b.size()).sum()
}

fn main() {
    println!("Part 1: {}", part1(include_str!("in.txt")));
    println!("Part 2: {}", part2(include_str!("in.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("test1.txt")), 39);
    assert_eq!(part1(include_str!("test2.txt")), 590784);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("test1.txt")), 39);
    assert_eq!(part2(include_str!("test2.txt")), 39769202357779);
    assert_eq!(part2(include_str!("test3.txt")), 2758514936282235);
}
