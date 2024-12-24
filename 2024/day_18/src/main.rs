use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
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

    fn dijkstra(&self) HashMap<Point, (Option<Point>, usize)> {
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

    let mut maze = Maze::new(7);
    for wall in input.trim().lines().map(|l| l.trim().parse().unwrap()) {
        maze.add_wall(wall);
    }
}
