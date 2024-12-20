#[derive(Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::str::FromStr for Point {
    type Err = ();

    fn from_str(text: &str) -> Result<Point, ()> {
        let v: Vec<Result<isize, _>> = text.trim().split(",").map(|n| n.trim().parse()).collect();
        if v.len() != 2 {
            return Err(());
        }
        let x = match v[0] {
            Ok(x) => x,
            Err(_) => return Err(()),
        };
        let y = match v[1] {
            Ok(x) => x,
            Err(_) => return Err(()),
        };
        Ok(Point { x, y })
    }
}

impl Point {
    fn product(self, factor: isize) -> Point {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
    #[allow(unused)]
    fn to_string(self) -> String {
        format!("({}, {})", self.x, self.y)
    }

    fn box_in(mut self, width: isize, height: isize) -> Point {
        self.x = self.x % width;
        if self.x < 0 {
            self.x += width;
        }
        self.y = self.y % height;
        if self.y < 0 {
            self.y += height;
        }
        assert!(self.y >= 0);
        assert!(self.x >= 0);
        self
    }
}

mod part_1 {
    use super::*;
    pub fn solve(input: &str, width: usize, height: usize) {
        let robots = input
            .trim()
            .lines()
            .map(|l| {
                l.trim()
                    .split_whitespace()
                    .map(|s| s[2..].parse::<Point>().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|r| r[0] + r[1].product(100))
            .map(|p| p.box_in(width as isize, height as isize))
            .collect::<Vec<_>>();

        let mut map: Vec<Vec<usize>> = (0..height)
            .map(|_| (0..width).map(|_| 0).collect())
            .collect();

        for r in &robots {
            map[r.y as usize][r.x as usize] += 1;
        }

        let quad_height = height / 2;
        let quad_width = width / 2;

        let mut quad_0 = 0;
        let mut quad_1 = 0;
        let mut quad_2 = 0;
        let mut quad_3 = 0;

        for y in 0..quad_height {
            for x in 0..quad_width {
                quad_0 += map[y][x];
                quad_1 += map[y][x + width - quad_width];
                quad_2 += map[y + height - quad_height][x];
                quad_3 += map[y + height - quad_height][x + width - quad_width];
            }
        }
        let solution = quad_0 * quad_1 * quad_2 * quad_3;
        println!("Solution 1: {solution}");
    }
}

#[allow(unused)]
fn map_print(map: &Vec<Vec<usize>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{} ", map[y][x]);
        }
        println!();
    }
}

#[allow(unused)]
fn map_pretty_print(map: &Vec<Vec<usize>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

mod part_2 {
    use super::*;

    #[derive(Debug)]
    struct Roboter {
        p: Point,
        v: Point,
    }

    impl Roboter {
        fn run(&mut self, time: usize, width: usize, height: usize) {
            self.p = self.p + self.v.product(time as isize);
            self.p = self.p.box_in(width as isize, height as isize);
        }
    }

    impl std::str::FromStr for Roboter {
        type Err = ();
        fn from_str(text: &str) -> Result<Roboter, ()> {
            let points: Vec<Result<Point, _>> = text
                .trim()
                .split_whitespace()
                .map(|p| p[2..].parse())
                .collect();
            if points.len() != 2 {
                return Err(());
            }
            let p = points[0]?;
            let v = points[1]?;

            Ok(Roboter { p, v })
        }
    }

    pub struct Map {
        roboter: Vec<Roboter>,
        width: usize,
        height: usize,
        map: Vec<Vec<usize>>,
    }

    impl Map {
        pub fn new(text: &str, width: usize, height: usize) -> Map {
            let roboter = text
                .trim()
                .lines()
                .map(|l| l.trim().parse().unwrap())
                .collect();

            let map = (0..height).map(|_| (0..width).map(|_| 0).collect()).collect();

            Map {
                roboter,
                width,
                height,
                map,
            }
        }
        pub fn run(&mut self, time: usize) {
            for r in &mut self.roboter {
                r.run(time, self.width, self.height);
            }

            for row in &mut self.map {
                for p in row {
                    *p = 0;
                }
            }

            for r in &self.roboter {
                self.map[r.p.y as usize][r.p.x as usize] += 1;
            }
        }

        pub fn pretty_print(&self) {
            for row in &self.map {
                for v in row {
                    if *v > 0 {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
    }
}

fn main() {
    //     let input = "
    // p=0,4 v=3,-3
    // p=6,3 v=-1,-3
    // p=10,3 v=-1,2
    // p=2,0 v=2,-1
    // p=0,0 v=1,3
    // p=3,0 v=-2,-2
    // p=7,6 v=-1,-3
    // p=3,0 v=-1,-2
    // p=9,3 v=2,3
    // p=7,3 v=-1,2
    // p=2,4 v=2,-3
    // p=9,5 v=-3,-3
    // ";
    //     let width = 11;
    //     let height = 7;

    let input = &std::fs::read_to_string("input").unwrap();
    let width = 101;
    let height = 103;
    part_1::solve(input, width, height);

    let mut map = part_2::Map::new(input, width, height);

    for i in 1..10000 {
        println!("Time: {i}");
        map.run(1);
        map.pretty_print();
        println!();
        println!();
    }
}
