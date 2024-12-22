use std::rc::Rc;
use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Tile {
    Visited(usize),
    Unvisited,
    Wall,
    End,
    Start,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self, t: usize) -> Direction {
        if t == 0 {
            return self;
        }
        match self {
            Direction::Up => Direction::Right.turn(t - 1),
            Direction::Right => Direction::Down.turn(t - 1),
            Direction::Down => Direction::Left.turn(t - 1),
            Direction::Left => Direction::Up.turn(t - 1),
        }
    }
}

impl Tile {
    fn from(c: char) -> Result<Tile, ()> {
        match c {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Unvisited),
            'E' => Ok(Tile::End),
            'S' => Ok(Tile::Start),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    #[allow(unused)]
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }

    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
    fn to_coords(self) -> (usize, usize) {
        (self.x, self.y)
    }
    fn step(self, dir: Direction) -> Point {
        match dir {
            Direction::Up => Point::new(self.x, self.y - 1),
            Direction::Right => Point::new(self.x + 1, self.y),
            Direction::Down => Point::new(self.x, self.y + 1),
            Direction::Left => Point::new(self.x - 1, self.y),
        }
    }
}

fn turn_cost(t: usize) -> usize {
    match t {
        0 => 0,
        1 | 3 => 1000,
        2 => 2000,
        _ => panic!("don't turn that often"),
    }
}

struct Node {
    pos: Point,
    cost: usize,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn position_set(&self) -> HashSet<Point> {
        let mut set = HashSet::new();
        set.insert(self.pos);

        for child in self.children.iter() {
            let child = child.position_set();
            for p in child.iter() {
                set.insert(*p);
            }
        }
        return set;
    }

    #[allow(unused)]
    fn dump(&self, level: usize) {
        println!("{}{}", " ".repeat(level), self.pos.to_string());
        for child in self.children.iter() {
            child.dump(level + 1);
        }
    }
}

fn build_tree(mut me: Node, dir: Direction, map: &mut Vec<Vec<Tile>>) -> Option<Node> {
    let (x, y) = me.pos.to_coords();
    match map[y][x] {
        Tile::End => return Some(me),
        Tile::Start => panic!("start should have been removed"),
        Tile::Wall => return None,
        Tile::Visited(last_cost) if last_cost < me.cost => return None,
        _ => (),
    }
    map[y][x] = Tile::Visited(me.cost);

    let mut min_cost = std::usize::MAX;

    for t in 0..=3 {
        let child = Node {
            pos: me.pos.step(dir.turn(t)),
            cost: me.cost + 1 + turn_cost(t),
            children: Vec::new(),
        };
        let child = match build_tree(child, dir.turn(t), map) {
            Some(child) => child,
            None => continue,
        };
        if child.cost < min_cost {
            min_cost = child.cost;
        }
        me.children.push(Rc::new(child));
    }
    if me.children.len() == 0 {
        return None;
    }
    me.cost = min_cost;
    me.children = me.children.iter().filter(|c| c.cost == min_cost).map(|c| c.clone()).collect();
    map[y][x] = Tile::Visited(min_cost);
    return Some(me);
}

fn main() {
    #[allow(unused)]
    let input = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    #[allow(unused)]
    let input = "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    let input = &std::fs::read_to_string("input").unwrap();

    let mut map: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|l| l.trim())
        .map(|l| l.chars().map(|c| Tile::from(c).unwrap()).collect())
        .collect();

    let mut start = Point::new(0, 0);
    'outer: for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Tile::Start = map[y][x] {
                start = Point::new(x, y);
                break 'outer;
            }
        }
    }
    map[start.y][start.x] = Tile::Unvisited;
    let root = Node {
        pos: start,
        cost: 0,
        children: Vec::new(),
    };
    let root = build_tree(root, Direction::Right, &mut map).unwrap();
    println!("Solution 1: {}", root.cost);
    let set = root.position_set();
    println!("Solution 2: {}", set.len());
}
