use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
    fn neighbors(self) -> [Point; 4] {
        [
            Point::new(-1, 0) + self,
            Point::new(1, 0) + self,
            Point::new(0, -1) + self,
            Point::new(0, 1) + self,
        ]
    }
    #[allow(unused)]
    fn to_string(&self) -> String {
        format!("({},{})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }
}

#[derive(Debug)]
enum Tile {
    Wall,
    Path(usize),
}

impl Tile {
    fn path_cost(&self) -> usize {
        match self {
            Tile::Path(cost) => *cost,
            Tile::Wall => panic!("did not expect wall"),
        }
    }
}

#[derive(Copy, Clone, Eq)]
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
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.cost.eq(&other.cost)
    }
}

struct Race {
    width: isize,
    height: isize,
    tiles: HashMap<Point, Tile>,
    base_time: usize,
}

impl Race {
    fn is_in_bounds(&self, point: Point) -> bool {
        if point.x < 0 || point.y < 0 {
            return false;
        }
        if point.y >= self.height || point.x >= self.width {
            return false;
        }
        return true;
    }

    #[allow(unused)]
    fn is_wall(&self, point: Point) -> bool {
        if !self.is_in_bounds(point) {
            return true;
        }
        match self.tiles.get(&point).unwrap() {
            Tile::Path(_) => false,
            Tile::Wall => true,
        }
    }

    fn dijkstra(&mut self, start: Point) {
        let mut queue = BinaryHeap::new();
        let mut seen = HashSet::new();
        queue.push(Node::new(start, 0));
        self.tiles.insert(start, Tile::Path(0));
        while let Some(node) = queue.pop() {
            if seen.contains(&node.point) {
                continue;
            }
            seen.insert(node.point);

            let neighbors = node.point.neighbors();
            let neighbors: Vec<_> = neighbors
                .iter()
                .filter(|n| self.is_in_bounds(**n))
                .map(|n| (n, node.cost + 1))
                .collect();

            for (point, cost) in neighbors {
                match self.tiles.get(point).unwrap() {
                    Tile::Wall => continue,
                    Tile::Path(current_cost) => {
                        if *current_cost > cost {
                            self.tiles.insert(*point, Tile::Path(cost));
                            queue.push(Node::new(*point, cost));
                        }
                    }
                }
            }
        }
    }
}

impl std::str::FromStr for Race {
    type Err = ();
    fn from_str(input: &str) -> Result<Race, ()> {
        let map: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|l| l.trim().chars().collect())
            .collect();

        let height = map.len();
        if height == 0 {
            return Err(());
        }
        let width = map[0].len();
        if width == 0 {
            return Err(());
        }

        let mut race = Race {
            width: width as isize,
            height: height as isize,
            tiles: HashMap::new(),
            base_time: 0,
        };

        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);
        for y in 0..height {
            for x in 0..width {
                match map[y][x] {
                    '.' => {
                        race.tiles
                            .insert(Point::from((x, y)), Tile::Path(std::usize::MAX));
                    }
                    '#' => {
                        race.tiles.insert(Point::from((x, y)), Tile::Wall);
                    }
                    'E' => {
                        end = Point::from((x, y));
                        race.tiles
                            .insert(Point::from((x, y)), Tile::Path(std::usize::MAX));
                    }
                    'S' => {
                        start = Point::from((x, y));
                        race.tiles
                            .insert(Point::from((x, y)), Tile::Path(std::usize::MAX));
                    }
                    _ => panic!("unexpected token"),
                }
            }
        }

        race.dijkstra(start);
        race.base_time = race.tiles.get(&end).unwrap().path_cost();
        return Ok(race);
    }
}

fn cheat_times(race: &Race, cheat_time: isize) -> Vec<usize> {
    let mut cheats = Vec::new();
    for (root_point, tile) in race.tiles.iter() {
        let root_cost = match tile {
            Tile::Path(cost) => cost,
            Tile::Wall => continue,
        };

        for dy in -cheat_time..=cheat_time {
            for dx in -cheat_time..=cheat_time {
                let distance = dx.abs() + dy.abs();
                if distance > cheat_time {
                    continue;
                }
                let distance = distance as usize;
                let point = *root_point + Point::new(dx, dy);
                if !race.is_in_bounds(point) {
                    continue;
                }
                if let Tile::Path(cost) = race.tiles.get(&point).unwrap() {
                    let total_cost = (race.base_time - cost) + distance + root_cost;
                    if total_cost < race.base_time {
                        cheats.push(total_cost);
                    }
                }
            }
        }
    }

return cheats;
}

fn main() {
    #[allow(unused)]
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

    let input = &std::fs::read_to_string("input").unwrap();

    let race: Race = input.parse().unwrap();
    println!("Base Time: {}", race.base_time);

    let solution_1 = cheat_times(&race, 2)
        .iter()
        .filter(|c| **c <= race.base_time - 100)
        .count();
    println!("Solution 1: {}", solution_1);

    let solution_2 = cheat_times(&race, 20)
        .iter()
        .filter(|c| **c <= race.base_time - 100)
        .count();
    println!("Solution 2: {}", solution_2);
}
