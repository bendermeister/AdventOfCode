mod solution_1 {
    fn count(
        field: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        parent: char,
        x: isize,
        y: isize,
    ) -> (usize, usize) {
        if x < 0 || y < 0 {
            return (0, 1);
        }
        let x = x as usize;
        let y = y as usize;
        if y >= field.len() {
            return (0, 1);
        }
        if x >= field[y].len() {
            return (0, 1);
        }
        if field[y][x] != parent {
            return (0, 1);
        }
        if visited[y][x] {
            return (0, 0);
        }
        visited[y][x] = true;

        let mut total_area = 1;
        let mut total_perimeter = 0;

        let x = x as isize;
        let y = y as isize;

        let (area, perimeter) = count(field, visited, parent, x + 1, y);
        total_area += area;
        total_perimeter += perimeter;

        let (area, perimeter) = count(field, visited, parent, x - 1, y);
        total_area += area;
        total_perimeter += perimeter;

        let (area, perimeter) = count(field, visited, parent, x, y - 1);
        total_area += area;
        total_perimeter += perimeter;

        let (area, perimeter) = count(field, visited, parent, x, y + 1);
        total_area += area;
        total_perimeter += perimeter;

        (total_area, total_perimeter)
    }

    pub fn solve(input: &str) {
        let field: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();
        let mut visited: Vec<Vec<bool>> = field
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();

        let mut sum = 0;
        for y in 0..field.len() {
            for x in 0..field[y].len() {
                let (area, perimeter) =
                    count(&field, &mut visited, field[y][x], x as isize, y as isize);
                sum += area * perimeter;
            }
        }
        println!("Solution 1: {sum}");
    }
}

mod solution_2 {
    use std::collections::HashSet;
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    struct Point {
        x: isize,
        y: isize,
    }

    impl Add for Point {
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

    struct Region {
        main: HashSet<Point>,
        parent: char,
    }

    impl Region {
        fn area(&self) -> usize {
            self.main.len()
        }

        fn grow(
            &mut self,
            field: &Vec<Vec<char>>,
            visited: &mut Vec<Vec<bool>>,
            x: isize,
            y: isize,
        ) {
            let point = Point::new(x, y);
            if x < 0 || y < 0 {
                return;
            }

            let x = x as usize;
            let y = y as usize;
            if y >= field.len() {
                return;
            }
            if x >= field[y].len() {
                return;
            }
            if field[y][x] != self.parent {
                return;
            }
            if visited[y][x] {
                return;
            }
            visited[y][x] = true;
            self.main.insert(point);

            let x = x as isize;
            let y = y as isize;

            self.grow(field, visited, x - 1, y);
            self.grow(field, visited, x + 1, y);
            self.grow(field, visited, x, y - 1);
            self.grow(field, visited, x, y + 1);
        }

        fn is_corner(&self, point: Point, dir_0: Point, dir_1: Point) -> bool {
            let has_0 = self.main.contains(&(point + dir_0));
            let has_1 = self.main.contains(&(point + dir_1));
            if self.main.contains(&(point + dir_0 + dir_1)) {
                return has_0 == has_1 && has_0 == false;
            }
            return has_0 == has_1;
        }


        fn perimeter(&self) -> usize {
            let mut sum = 0;
            let up = Point::new(0, -1);
            let down = Point::new(0, 1);
            let left = Point::new(-1, 0);
            let right = Point::new(1, 0);
            for p in self.main.iter() {
                if self.is_corner(*p, up, right) {
                    sum += 1;
                }
                if self.is_corner(*p, up, left) {
                    sum += 1;
                }
                if self.is_corner(*p, down, right) {
                    sum += 1;
                }
                if self.is_corner(*p, down, left) {
                    sum += 1;
                }
            }
            return sum;
        }

        fn new(field: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> Region {
            let mut region = Region {
                main: HashSet::new(),
                parent: field[y][x],
            };
            region.grow(field, visited, x as isize, y as isize);
            region
        }
    }

    pub fn solve(input: &str) {
        let field: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|l| l.trim().chars().collect())
            .collect();
        let mut visited: Vec<Vec<bool>> = field
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();

        let mut regions = Vec::new();

        for y in 0..field.len() {
            for x in 0..field[y].len() {
                if visited[y][x] {
                    continue;
                }
                regions.push(Region::new(&field, &mut visited, x, y))
            }
        }

        let mut sum = 0;
        for region in &regions {
            let area = region.area();
            let perimeter = region.perimeter();
            //println!("{}, a: {}, p: {}", region.parent, area, perimeter);
            sum += area * perimeter;
        }

        println!("Solution 2: {sum}");
    }
}

fn run(input: &str) {
    let input = input.trim();
    println!("{input}");
    solution_1::solve(input);
    solution_2::solve(input);
    println!();
}

fn main() {
    let input = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
    run(input);

    let input = "
AAAA
BBCD
BBCC
EEEC
";
    run(input);

    let input = "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    run(input);

    let input = "
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
    run(input);

    let input = "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
    run(input);
    let input = &std::fs::read_to_string("input").unwrap();
    let input = input.trim();
    solution_1::solve(input);
    solution_2::solve(input);
}
