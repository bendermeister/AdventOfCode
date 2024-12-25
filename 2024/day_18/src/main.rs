use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
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

impl std::str::FromStr for Point {
    type Err = ();
    fn from_str(text: &str) -> Result<Point, ()> {
        let mut iter = text.trim().split(",").map(|n| n.parse());
        let x = match iter.next() {
            Some(val) => match val {
                Ok(val) => val,
                Err(_) => return Err(()),
            },
            None => return Err(()),
        };
        let y = match iter.next() {
            Some(val) => match val {
                Ok(val) => val,
                Err(_) => return Err(()),
            },
            None => return Err(()),
        };
        Ok(Point { x, y })
    }
}

struct Node {
    parent: Point,
    cost: i32,
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
    fn eq(&self, other: &Node) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

impl Node {
    fn new(parent: Point, cost: i32) -> Node {
        Node { parent, cost }
    }
}

struct Maze {
    walls: HashSet<Point>,
    width: i32,
    tree: HashMap<Point, Node>,
}

impl Maze {
    fn new(width: i32) -> Maze {
        let maze = Maze {
            walls: HashSet::new(),
            tree: HashMap::new(),
            width,
        };
        return maze;
    }

    fn is_wall(&self, wall: Point) -> bool {
        if wall.x < 0 || wall.y < 0 {
            return true;
        }
        if wall.x >= self.width || wall.y >= self.width {
            return true;
        }
        return self.walls.contains(&wall);
    }

    fn dijkstra(&mut self) -> HashMap<Point, Node> {
        let mut queue = BinaryHeap::new();
        queue.push((Node::new(Point::new(0, 0), 0)));
        let mut seen = HashSet::new();
        let neighbors = [
            Point::new(1, 0),
            Point::new(-1, 0),
            Point::new(0, 1),
            Point::new(0, -1),
        ];

        let mut graph = HashMap::new();

        while let Some(current) = queue.pop() {
            let point = current.parent;
            let cost = current.cost;
            if seen.contains(&point) {
                continue;
            }
            seen.insert(point);
            let neighbors = neighbors
                .iter()
                .map(|n| *n + point)
                .filter(|n| !seen.contains(n))
                .filter(|n| !self.is_wall(*n));
            for n in neighbors {
                let entry = graph.entry(n).or_insert(Node::new(point, std::i32::MAX));
                if entry.cost > cost + 1 {
                    entry.cost = cost + 1;
                    entry.parent = point;
                }
                queue.push(Node::new(n, cost + 1));
            }
        }

        return graph;
    }

    fn add_wall(&mut self, wall: Point) {
        self.walls.insert(wall);
    }
}

fn main() {
    let input = "
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
    let end = 12;

    let input = &std::fs::read_to_string("input").unwrap();
    let width = 71;
    let end = 1024;

    let walls: Vec<Point> = input
        .trim()
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect();

    let mut maze = Maze::new(width);

    for wall in &walls[..end] {
        maze.add_wall(*wall);
    }

    let solution_1 = maze.dijkstra().get(&Point::new(width - 1, width - 1)).unwrap().cost;
    println!("Solution 1: {}", solution_1);

    for wall in &walls[end..] {
        maze.add_wall(*wall);
        if let None = maze.dijkstra().get(&Point::new(width - 1, width - 1)) {
            println!("Solution 2: {},{}", wall.x, wall.y);
            break;
        }
    }

}
