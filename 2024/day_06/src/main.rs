#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn to_vec(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

#[derive(Clone)]
struct Guard {
    x: i32,
    y: i32,
    d: Direction,
}

impl Guard {
    fn next_pos(&self) -> (i32, i32) {
        let (dx, dy) = self.d.to_vec();
        (self.x + dx, self.y + dy)
    }

    fn new(field: &Vec<Vec<char>>) -> Guard {
        for (y, row) in field.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let x = x as i32;
                let y = y as i32;
                match cell {
                    '^' => {
                        return Guard {
                            x,
                            y,
                            d: Direction::Up,
                        }
                    }
                    'v' => {
                        return Guard {
                            x,
                            y,
                            d: Direction::Down,
                        }
                    }
                    '>' => {
                        return Guard {
                            x,
                            y,
                            d: Direction::Right,
                        }
                    }
                    '<' => {
                        return Guard {
                            x,
                            y,
                            d: Direction::Left,
                        }
                    }
                    _ => (),
                }
            }
        }
        unreachable!();
    }
}

enum Field {
    Free,
    Visited,
    Obstacle,
}

fn solution_1(field: &str) {
    let field = String::from(field);
    let field = field
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut guard = Guard::new(&field);

    let mut field = field
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| match c {
                    '.' => Field::Free,
                    '^' => Field::Visited,
                    '>' => Field::Visited,
                    'v' => Field::Visited,
                    '<' => Field::Visited,
                    '#' => Field::Obstacle,
                    _ => panic!("unreachable"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = field[0].len();
    let height = field.len();

    let is_outside =
        |(x, y): (i32, i32)| x < 0 || y < 0 || x as usize >= width || y as usize >= height;

    loop {
        let (x, y) = guard.next_pos();
        if is_outside((x, y)) {
            break;
        }

        match field[y as usize][x as usize] {
            Field::Free => {
                guard.x = x;
                guard.y = y;
                field[y as usize][x as usize] = Field::Visited;
            }
            Field::Visited => {
                guard.x = x;
                guard.y = y;
            }
            Field::Obstacle => {
                guard.d = guard.d.rotate();
            }
        }
    }

    let solution_1: usize = field
        .iter()
        .map(|row| {
            row.iter()
                .filter(|c| match c {
                    Field::Visited => true,
                    _ => false,
                })
                .count()
        })
        .sum();
    println!("Solution 1: {}", solution_1);
}

#[derive(Clone)]
enum Field2 {
    Free,
    Visited(Direction),
    Obstacle,
}

// TODO this solution is very shit fix this

fn has_loop(field: &mut Vec<Vec<Field2>>, guard: &Guard) -> bool {
    let mut guard = guard.clone();

    let width = field[0].len();
    let height = field.len();
    let is_outside =
        |(x, y): (i32, i32)| x < 0 || y < 0 || x as usize >= width || y as usize >= height;

    loop {
        let (x, y) = guard.next_pos();
        if is_outside((x, y)) {
            break;
        }

        match field[y as usize][x as usize] {
            Field2::Free => {
                guard.x = x;
                guard.y = y;
                field[y as usize][x as usize] = Field2::Visited(guard.d.clone());
            }
            Field2::Visited(dir) => match (dir, guard.d) {
                (Direction::Up, Direction::Up) => return true,
                (Direction::Right, Direction::Right) => return true,
                (Direction::Down, Direction::Down) => return true,
                (Direction::Left, Direction::Left) => return true,
                _ => {
                    guard.x = x;
                    guard.y = y;
                }
            },
            Field2::Obstacle => {
                guard.d = guard.d.rotate();
            }
        }
    }

    false
}

fn solution_2(field: &str) {
    let field = field
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let guard = Guard::new(&field);

    let field = field
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| match c {
                    '.' => Field2::Free,
                    '^' => Field2::Visited(Direction::Up),
                    '>' => Field2::Visited(Direction::Right),
                    'v' => Field2::Visited(Direction::Down),
                    '<' => Field2::Visited(Direction::Left),
                    '#' => Field2::Obstacle,
                    _ => panic!("unreachable"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    for y in 0..field.len() {
        for x in 0..field[0].len() {
            if x == guard.x as usize && y == guard.y as usize {
                continue;
            }
            let mut field = field.clone();
            field[y][x] = Field2::Obstacle;
            if has_loop(&mut field, &guard) {
                count += 1;
            }
        }
    }

    println!("Solution 2: {}", count);
}

fn main() {
    // let field = "
    // ....#.....
    // .........#
    // ..........
    // ..#.......
    // .......#..
    // ..........
    // .#..^.....
    // ........#.
    // #.........
    // ......#...
    // ";

    let field = std::fs::read_to_string("input").expect("could not read file");
    solution_1(&field);
    solution_2(&field);
}
