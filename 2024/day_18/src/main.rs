use std::collections::HashMap;
use std::collections::HashSet;

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
}

impl std::str::FromStr for Point {
    type Err = ();
    fn from_str(text: &str) -> Result<Point, ()> {
        let mut iter = text.trim().split(",");
        let x = match iter.next() {
            Some(v) => v,
            None => return Err(()),
        };
        let y = match iter.next() {
            Some(v) => v,
            None => return Err(()),
        };
        match iter.next() {
            Some(_) => return Err(()),
            None => (),
        }
        let x = match x.parse() {
            Ok(val) => val,
            Err(_) => return Err(()),
        };
        let y = match y.parse() {
            Ok(val) => val,
            Err(_) => return Err(()),
        };

        Ok(Point { x, y })
    }
}

struct Maze {
    walls: HashSet<Point>,
    width: isize,
}

impl Maze {
    fn new(width: isize) -> Maze {
        Maze {
            walls: HashSet::new(),
            width,
        }
    }

    fn add_wall(&mut self, wall: Point) {
        self.walls.insert(wall);
    }

    fn is_wall(&self, point: Point) -> bool {
        if point.x < 0 || point.y < 0 {
            return true;
        }
        if point.x >= self.width || point.y >= self.width {
            return true;
        }
        return self.walls.contains(&point);
    }

    // NOTE: this is one of the most retarded ways to implement dijkstra, but i am to lazy to
    // figure out how min heaps work in rust and this is fast enough
    fn cost_to_solve(&self) -> Option<usize> {
        let mut graph = HashMap::new();
        for y in 0..self.width {
            for x in 0..self.width {
                let point = Point::new(x, y);
                if self.is_wall(point) {
                    continue;
                }
                graph.insert(point, std::usize::MAX);
            }
        }

        graph.insert(Point::new(0, 0), 0);
        let mut seen = HashSet::new();

        let neighbors = [
            Point::new(1, 0),
            Point::new(-1, 0),
            Point::new(0, 1),
            Point::new(0, -1),
        ];

        while seen.len() < graph.len() {
            let mut current_cost = std::usize::MAX;
            let mut current_node = *graph.iter().next().unwrap().0;

            for (node, cost) in graph.iter().filter(|(node, _)| !seen.contains(*node)) {
                if current_cost >= *cost {
                    current_node = *node;
                    current_cost = *cost;
                }
            }
            seen.insert(current_node);

            if current_cost == std::usize::MAX {
                continue;
            }

            let neighbors: Vec<_> = neighbors
                .iter()
                .map(|n| *n + current_node)
                .filter(|n| !self.is_wall(*n))
                .filter(|n| *graph.get(n).unwrap() > current_cost + 1)
                .collect();
            for n in neighbors {
                graph.insert(n, current_cost + 1);
            }
        }

        let cost = *graph
            .get(&Point::new(self.width - 1, self.width - 1))
            .unwrap();
        if cost == std::usize::MAX {
            return None;
        }
        return Some(cost);
    }
}

fn main() {
    /*let input = "
    5,4
    4,2
    4,5
    3,0
    2,1
    6,3
    2,4
    1,5
    0,6
    3,3
    2,6
    5,1
    1,2
    5,5
    2,5
    6,5
    1,4
    0,4
    6,4
    1,1
    6,1
    1,0
    0,5
    1,6
    2,0
    ";

        let width = 7;
        let end = 12;  */

    let input = &std::fs::read_to_string("input").unwrap();
    let width = 71;
    let end = 1024;

    let mut maze = Maze::new(width);
    let walls: Vec<Point> = input
        .trim()
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect();

    for wall in &walls[..end] {
        maze.add_wall(*wall);
    }

    let solution_1 = maze.cost_to_solve().unwrap();
    println!("Solution 1: {solution_1}");

    for wall in &walls[end..] {
        maze.add_wall(*wall);
        if let None = maze.cost_to_solve() {
            println!("Solution 2: {},{}", wall.x, wall.y);
            break;
        }
    }
}
