use std::collections::HashSet;
use std::collections::HashMap;

enum Tile {
    Free(usize),
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
    fn to_coords(&self) -> (usize, usize) {
        (self.x.try_into().unwrap(), self.y.try_into().unwrap())
    }
    fn to_string(&self) -> String {
        format!("({},{})", self.x, self.y)
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from(p: (usize, usize)) -> Self {
        Point {
            x: p.0.try_into().unwrap(),
            y: p.1.try_into().unwrap(),
        }
    }
}

fn neighbor_point_deltas() -> [Point; 4] {
    [
        Point::new(1, 0),
        Point::new(-1, 0),
        Point::new(0, 1),
        Point::new(0, -1),
    ]
}

fn explore_helper(
    seen: &mut HashSet<Point>,
    map: &mut Vec<Vec<Tile>>,
    current: Point,
    end: Point,
) -> Option<usize> {
    if seen.contains(&current) {
        return None;
    }
    seen.insert(current);

    let (x, y) = current.to_coords();
    if current == end {
        map[y][x] = Tile::Free(0);
        return Some(1);
    }
    match map[y][x] {
        Tile::Wall => return None,
        _ => (),
    }

    let neighbors: Vec<_> = neighbor_point_deltas()
        .iter()
        .map(|n| *n + current)
        .map(|n| explore_helper(seen, map, n, end))
        .filter(|n| match n {
            Some(_) => true,
            None => false,
        })
        .map(|n| n.unwrap())
        .collect();
    assert!(neighbors.len() == 1);

    let cost = neighbors[0];
    map[y][x] = Tile::Free(cost);
    return Some(cost + 1);
}

fn explore(map: &mut Vec<Vec<Tile>>, start: Point, end: Point) -> usize {
    let mut seen = HashSet::new();
    explore_helper(&mut seen, map, start, end).unwrap() - 1
}

fn main() {
    let input = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    let mut map = Vec::new();
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);
    let mut points = Vec::new();

    for (y, line) in input.trim().lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.trim().chars().enumerate() {
            match c {
                '.' => {
                    row.push(Tile::Free(0));
                    points.push(Point::from((x, y)));
                }
                '#' => row.push(Tile::Wall),
                'S' => {
                    start = Point::from((x, y));
                    points.push(Point::from((x, y)));
                    row.push(Tile::Free(0));
                }
                'E' => {
                    end = Point::from((x, y));
                    points.push(Point::from((x, y)));
                    row.push(Tile::Free(0));
                }
                _ => panic!("encountered unexpected tile"),
            }
        }
        map.push(row);
    }

    let base_time = explore(&mut map, start, end);
    println!("Base Time: {base_time}");

    let mut result = Vec::new();

    for point in points.iter() {
        cheat(&mut result, &map, *point);
    }

    let mut histo = HashMap::new();
    for r in result.iter().filter(|r| **r < base_time) {
        *histo.entry(r).or_insert(0) += 1;
    }

    for (k, v) in histo.iter() {
        println!("{k}: {v}")
    }
}
