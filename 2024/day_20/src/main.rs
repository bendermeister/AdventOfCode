use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]

struct Point(isize, isize);
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

struct Node {
    point: Point,
    cost: usize,
}

impl Node {
    fn new(point: Point, cost: usize) -> Node {
        Node { point, cost }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, _: &Node) -> bool {
        todo!()
    }
}

impl Eq for Node {}

fn is_wall(map: &Vec<Vec<char>>, point: Point) -> bool {
    if point.0 < 0 || point.1 < 0 {
        return true;
    }
    let x = point.0 as usize;
    let y = point.1 as usize;
    if y >= map.len() || x >= map[x].len() {
        return true;
    }
    match map[y][x] {
        '.' => false,
        '#' => true,
        _ => panic!("unexpected tile"),
    }
}

fn solve_maze(map: &Vec<Vec<char>>, start: Point, end: Point) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(Node::new(start, 0));
    let mut graph = HashMap::new();
    let mut seen = HashSet::new();

    let children = [Point(-1, 0), Point(1, 0), Point(0, 1), Point(0, -1)];

    while let Some(current_node) = queue.pop() {
        let point = current_node.point;
        let cost = current_node.cost;
        if seen.contains(&point) {
            continue;
        }
        seen.insert(point);
        let children = children
            .iter()
            .map(|c| *c + point)
            .filter(|c| !is_wall(map, *c));

        for child in children {
            let child_cost = *graph.entry(child).or_insert(std::usize::MAX);
            if cost + 1 < child_cost {
                graph.insert(child, cost + 1);
                queue.push(Node::new(child, cost + 1));
            }
        }
    }
    return *graph.entry(end).or_insert(std::usize::MAX);
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
###############
";

    let mut map: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();
    let height = map.len();
    let width = map[0].len();

    let mut start = Point(0, 0);
    let mut end = Point(0, 0);

    for y in 0..height {
        for x in 0..width {
            match map[y][x] {
                'S' => {
                    start = Point(x as isize, y as isize);
                    map[y][x] = '.';
                }
                'E' => {
                    end = Point(x as isize, y as isize);
                    map[y][x] = '.';
                }
                _ => (),
            }
        }
    }

    let base_time = solve_maze(&map, start, end);

    let children = [Point(1, 0), Point(-1, 0), Point(0, 1), Point(0, -1)];
    for y in 1..height-1 {
        for x in 1..width-1 {
            let point = Point(x as isize, y as isize);
            let children = children.iter().map(|c| *c + point);
            let point_char = map[y][x];
            map[y][x] = '.';
            for child in children {
                let x = child.0 as usize;
                let y = child.1 as usize;
                let child_char = map[y][x];
                map[y][x] = '.';
                let time = solve_maze(&map, start, end);
                println!("{time}");
                map[y][x] = child_char;
            }
            map[y][x] = point_char;
        }
    }

    println!("Base Time: {base_time}");
}
