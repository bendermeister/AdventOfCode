use std::ops::Add;
use std::ops::Sub;
enum Cell {
    Empty(bool),
    Antenna(char, bool),
}

impl Cell {
    fn has_antidote(&self) -> bool {
        match self {
            Cell::Antenna(_, b) => *b,
            Cell::Empty(b) => *b,
        }
    }

    fn set_antidote(&mut self) {
        *self = match self {
            Cell::Antenna(c, _) => Cell::Antenna(*c, true),
            Cell::Empty(_) => Cell::Empty(true),
        };
    }

    fn from(c: char) -> Cell {
        match c {
            '.' => Cell::Empty(false),
            c => Cell::Antenna(c, false),
        }
    }
}

struct Field {
    field: Vec<Vec<Cell>>,
}

impl Field {
    fn is_in_bounce(&self, v: V2d) -> bool {
        if v.x < 0 || v.y < 0 {
            return false;
        }
        let (x, y) = v.to_coords();
        if y >= self.field.len() {
            return false;
        }
        if x >= self.field[y].len() {
            return false;
        }
        true
    }

    fn set_antidote(&mut self, v: V2d) {
        if !self.is_in_bounce(v) {
            return;
        }
        let (x, y) = v.to_coords();
        self.field[y][x].set_antidote();
    }

    fn count_antidote(&self) -> usize {
        self.field
            .iter()
            .map(|row| row.iter().filter(|c| c.has_antidote()).count())
            .sum()
    }
}

#[derive(Copy, Debug, Clone, PartialEq)]
struct V2d {
    x: isize,
    y: isize,
}

impl Add for V2d {
    type Output = Self;

    fn add(self, other: V2d) -> Self {
        V2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for V2d {
    type Output = Self;
    fn sub(self, other: V2d) -> Self {
        V2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl V2d {
    fn to_coords(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
    fn new(x: usize, y: usize) -> V2d {
        V2d {
            x: x as isize,
            y: y as isize,
        }
    }
}

struct Problem1<'a> {
    input: &'a str,
}

impl<'a> Problem1<'a> {
    fn new(input: &'a str) -> Problem1<'a> {
        Problem1 { input }
    }

    fn solve(&self) {
        let field: Vec<Vec<Cell>> = self
            .input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| Cell::from(c)).collect())
            .collect();
        let mut field = Field { field };

        for y in 0..field.field.len() {
            for x in 0..field.field[y].len() {
                let c = match field.field[y][x] {
                    Cell::Empty(_) => continue,
                    Cell::Antenna(c, _) => c,
                };
                self.set_antidotes(&mut field, V2d::new(x, y), c);
            }
        }

        println!("Solution 1: {}", field.count_antidote());
    }

    fn set_antidotes(&self, field: &mut Field, antenna: V2d, ac: char) {
        for y in 0..field.field.len() {
            for x in 0..field.field.len() {
                match field.field[y][x] {
                    Cell::Antenna(c, _) if c == ac => (),
                    _ => continue,
                }
                let v = V2d::new(x, y);
                if v == antenna {
                    continue;
                }
                field.set_antidote(v + (v - antenna));
            }
        }
    }
}

struct Problem2<'a> {
    input: &'a str,
}

impl<'a> Problem2<'a> {
    fn new(input: &'a str) -> Problem2<'a> {
        Problem2 { input }
    }

    fn solve(&self) {
        let field: Vec<Vec<Cell>> = self
            .input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| Cell::from(c)).collect())
            .collect();
        let mut field = Field { field };

        for y in 0..field.field.len() {
            for x in 0..field.field[y].len() {
                let c = match field.field[y][x] {
                    Cell::Empty(_) => continue,
                    Cell::Antenna(c, _) => c,
                };
                self.set_antidotes(&mut field, V2d::new(x, y), c);
            }
        }

        println!("Solution 2: {}", field.count_antidote());
    }

    fn set_antidotes(&self, field: &mut Field, antenna: V2d, ac: char) {
        for y in 0..field.field.len() {
            for x in 0..field.field[y].len() {
                let mut v = V2d::new(x, y);
                if antenna == v {
                    continue;
                }
                match field.field[y][x] {
                    Cell::Antenna(c, _) if c == ac => (),
                    _ => continue,
                }
                let dv = v - antenna;
                while field.is_in_bounce(v) {
                    field.set_antidote(v);
                    v = v + dv;
                }
            }
        }
    }
}

fn main() {
//     let input = "
// ............
// ........0...
// .....0......
// .......0....
// ....0.......
// ......A.....
// ............
// ............
// ........A...
// .........A..
// ............
// ............
// ";

    let input = &std::fs::read_to_string("input").unwrap();
    Problem1::new(input).solve();
    Problem2::new(input).solve();
}
