use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::slice::Iter;

fn parse_id(s: &str) -> usize {
    s["Tile ".len()..(s.len() - 1)].parse().unwrap()
}

fn bits_to_string(bits: Vec<bool>) -> String {
    bits.iter()
        .map(|b| if *b { '#' } else { '.' })
        .collect::<String>()
}

#[derive(Clone)]
struct Bitmap(Vec<Vec<bool>>);

impl Bitmap {
    fn new(size: usize) -> Self {
        let bitmap = vec![vec![false; size]; size];
        Bitmap(bitmap)
    }

    fn size(&self) -> usize {
        self.0[0].len()
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        self.0[x][y] = val;
    }

    fn get(&self, x: usize, y: usize) -> bool {
        match self.0.get(x) {
            None => false,
            Some(a) => match a.get(y) {
                None => false,
                Some(v) => *v,
            },
        }
    }

    fn rotate_right(&self) -> Bitmap {
        let size = self.size();
        let mut rotated = vec![vec![false; size]; size];
        for i in 0..size {
            for j in 0..size {
                rotated[i][j] = self.0[size - j - 1][i];
            }
        }
        Bitmap(rotated)
    }

    fn flip(&self) -> Bitmap {
        let size = self.size();
        let mut flipped = vec![vec![false; size]; size];
        for i in 0..size {
            for j in 0..size {
                flipped[i][j] = self.0[j][i];
            }
        }
        Bitmap(flipped)
    }

    fn count(&self) -> usize {
        let mut result = 0;
        let size = self.size();
        for x in 0..size {
            for y in 0..size {
                if self.0[x][y] {
                    result += 1;
                }
            }
        }
        result
    }

    #[allow(dead_code)]
    fn print(&self) {
        let size = self.size();
        for x in 0..size {
            let line = self.0[x]
                .iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>();
            println!("{}", line);
        }
    }
}

const TILE_SIZE: usize = 10;

#[derive(Clone)]
struct Tile {
    id: usize,
    bitmap: Bitmap,
}

impl Tile {
    fn new(s: &str) -> Self {
        let lines = s.lines().collect::<Vec<&str>>();
        let mut bitmap = Bitmap::new(TILE_SIZE);
        for (x, line) in lines.iter().skip(1).enumerate() {
            for (y, ch) in line.chars().enumerate() {
                if ch == '#' {
                    bitmap.set(x, y, true);
                }
            }
        }
        Tile {
            id: parse_id(lines.get(0).unwrap()),
            bitmap: bitmap,
        }
    }

    fn rotate_right(&self) -> Tile {
        let rotated = self.bitmap.rotate_right();
        Tile {
            id: self.id,
            bitmap: rotated,
        }
    }

    fn flip(&self) -> Tile {
        let flipped = self.bitmap.flip();
        Tile {
            id: self.id,
            bitmap: flipped,
        }
    }

    fn edge(&self, edge: Edge) -> String {
        let size = self.bitmap.size();
        match edge {
            Edge::N => bits_to_string(self.bitmap.0[0].iter().cloned().collect::<Vec<bool>>()),
            Edge::E => bits_to_string(
                self.bitmap
                    .0
                    .iter()
                    .map(|row| row[size - 1])
                    .collect::<Vec<bool>>(),
            ),
            Edge::S => bits_to_string(
                self.bitmap.0[size - 1]
                    .iter()
                    .cloned()
                    .collect::<Vec<bool>>(),
            ),
            Edge::W => bits_to_string(
                self.bitmap
                    .0
                    .iter()
                    .map(|row| row[0])
                    .collect::<Vec<bool>>(),
            ),
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tile").field("id", &self.id).finish()
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Edge {
    N,
    E,
    S,
    W,
}

impl Edge {
    pub fn iter() -> Iter<'static, Edge> {
        use Edge::*;
        static DIRECTIONS: [Edge; 4] = [N, S, E, W];
        DIRECTIONS.iter()
    }
}

#[derive(Clone)]
struct RotatedTile {
    pub tile: Tile,
    rotation: Edge,
    flipped: bool,
}

impl RotatedTile {
    fn new(tile: Tile, rotation: Edge, flip: bool) -> Self {
        let rotated = match rotation {
            Edge::N => tile,
            Edge::E => tile.rotate_right(),
            Edge::S => tile.rotate_right().rotate_right(),
            Edge::W => tile.rotate_right().rotate_right().rotate_right(),
        };

        RotatedTile {
            tile: if flip { rotated.flip() } else { rotated },
            rotation: rotation,
            flipped: flip,
        }
    }

    fn edge(&self, edge: Edge) -> String {
        self.tile.edge(edge)
    }
}

impl fmt::Debug for RotatedTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tile")
            .field("id", &self.tile.id)
            .field("rot", &self.rotation)
            .field("f", &self.flipped)
            .finish()
    }
}

fn rev(s: String) -> String {
    s.chars().rev().collect::<String>()
}

fn solve(s: &str) -> (usize, usize) {
    let tiles = s.split("\n\n").map(|s| Tile::new(s)).collect::<Vec<Tile>>();
    let count: usize = tiles.len();
    let mut map: HashMap<String, Vec<Tile>> = HashMap::new();
    for t in tiles.clone() {
        for e in Edge::iter() {
            let data = t.edge(e.clone());
            let val = t.clone();
            map.entry(data.clone()).or_insert(vec![]).push(val.clone());
            map.entry(rev(data)).or_insert(vec![]).push(val.clone());
        }
    }
    let size = (count as f64).sqrt() as usize;
    fn find_match(
        cur: &Vec<RotatedTile>,
        tiles: &Vec<Tile>,
        edges: &HashMap<String, Vec<Tile>>,
        size: usize,
    ) -> Option<Vec<RotatedTile>> {
        if cur.len() == size * size {
            return Some(cur.clone());
        }
        let pos = cur.len();
        let row = pos / size;
        let col = pos % size;
        let mut top: Option<String> = None;
        let mut left: Option<String> = None;
        if col > 0 {
            let left_tile = cur.get(pos - 1).unwrap();
            left = Some(left_tile.edge(Edge::E));
        }
        if row > 0 {
            let top_tile = cur.get(pos - size).unwrap();
            top = Some(top_tile.edge(Edge::S));
        }
        let candidates = if top == None && left == None {
            tiles.clone()
        } else {
            let a: HashSet<Tile> = match top.clone() {
                None => tiles.clone(),
                Some(top) => edges.get(&top).unwrap().clone(),
            }
            .iter()
            .map(|t| t.clone())
            .collect::<HashSet<Tile>>();
            let b: HashSet<Tile> = match left.clone() {
                None => tiles.clone(),
                Some(edge) => edges.get(&edge).unwrap().clone(),
            }
            .iter()
            .map(|t| t.clone())
            .collect::<HashSet<Tile>>();
            let s = a
                .intersection(&b)
                .filter(|t| !cur.iter().any(|o| t.id == o.tile.id))
                .map(|t| t.clone())
                .collect::<Vec<Tile>>();
            s
        };
        for c in candidates {
            for e in Edge::iter() {
                for flipped in vec![true, false] {
                    let tile = RotatedTile::new(c.clone(), e.clone(), flipped);
                    if let Some(ref l) = left {
                        let left_edge = tile.edge(Edge::W);
                        if &left_edge != l {
                            continue;
                        }
                    }
                    if let Some(ref t) = top {
                        let top_edge = tile.edge(Edge::N);
                        if &top_edge != t {
                            continue;
                        }
                    }

                    let mut next = cur.clone();
                    next.push(tile);
                    if let Some(res) = find_match(&next, tiles, edges, size) {
                        return Some(res);
                    }
                }
            }
        }

        return None;
    }

    let arrangement = find_match(&vec![], &tiles, &map, size).unwrap();
    let corners = vec![
        arrangement.get(0).unwrap().tile.id,
        arrangement.get(size - 1).unwrap().tile.id,
        arrangement.get(size * size - size).unwrap().tile.id,
        arrangement.get(size * size - 1).unwrap().tile.id,
    ];
    let part1 = corners.iter().fold(1, |a, b| a * b);

    let mut bitmap = Bitmap::new(size * 8);
    for x in 0..size {
        for y in 0..size {
            let tile = arrangement.get(size * x + y).unwrap();
            for i in 0..8 {
                for j in 0..8 {
                    bitmap.set(x * 8 + i, y * 8 + j, tile.tile.bitmap.get(i + 1, j + 1));
                }
            }
        }
    }

    let seemonster = include_str!("seemonster.txt")
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(y, ch)| match ch {
                    '#' => Some((x, y)),
                    _ => None,
                })
        })
        .collect::<Vec<(usize, usize)>>();

    let mut bm = bitmap;

    let mut found_at = vec![];
    'outer: for _ in 0..4 {
        for _ in 0..2 {
            for x in 0..(size * 8) {
                for y in 0..(size * 8) {
                    let found = seemonster.iter().all(|(sx, sy)| bm.get(x + sx, y + sy));
                    if found {
                        found_at.push((x, y));
                    }
                }
            }
            if found_at.len() > 0 {
                break 'outer;
            }
            bm = bm.flip();
        }
        bm = bm.rotate_right();
    }

    for (x, y) in found_at {
        for (sx, sy) in seemonster.iter() {
            bm.set(x + sx, y + sy, false);
        }
    }

    let part2 = bm.count();

    (part1, part2)
}

fn main() {
    let (part1, part2) = solve(include_str!("in.txt"));
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[test]
fn test_example() {
    assert_eq!(solve(include_str!("test.txt")), (20899048083289, 273))
}

#[test]
fn test_tile() {
    let tiles = include_str!("test.txt")
        .split("\n\n")
        .map(|s| Tile::new(s))
        .collect::<Vec<Tile>>();

    let tile = tiles.get(0).unwrap();
    assert_eq!(tile.id, 2311);
    assert_eq!(tile.edge(Edge::N), "..##.#..#.");
    assert_eq!(tile.edge(Edge::S), "..###..###");
    assert_eq!(tile.edge(Edge::E), "...#.##..#");
    assert_eq!(tile.edge(Edge::W), ".#####..#.");
}
