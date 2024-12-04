struct Direction {
    x: isize,
    y: isize,
}

impl Direction {
    fn new(x: isize, y: isize) -> Direction {
        Direction { x, y }
    }
}

fn is_xmas(field: &[&[u8]], x: isize, y: isize, dir: &Direction) -> bool {
    let word: Vec<u8> = (0..4)
        .map(|i| (x + dir.x * i, y + dir.y * i))
        .filter(|(x, y)| *x >= 0 && *y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .filter(|(_, y)| *y < field.len())
        .filter(|(x, y)| *x < field[*y].len())
        .map(|(x, y)| field[y][x])
        .collect();

    if word.len() != 4 {
        return false;
    }

    let word = String::from_utf8(word).expect("could not create utf8 string");

    word == "XMAS"
}

fn solution_1(field: &[&[u8]]) -> usize {
    let directions = [
        Direction::new(1, 0),
        Direction::new(1, 1),
        Direction::new(0, 1),
        Direction::new(-1, 1),
        Direction::new(-1, 0),
        Direction::new(-1, -1),
        Direction::new(0, -1),
        Direction::new(1, -1),
    ];

    let mut sum = 0;
    for y in 0..field.len() {
        for x in 0..field[y].len() {
            sum += directions
                .iter()
                .filter(|d| is_xmas(&field, x as isize, y as isize, d))
                .count();
        }
    }
    sum
}

fn solution_2(field: &[&[u8]]) -> usize {
    assert!(field.len() > 0);
    let height = field.len();
    let width = field[0].len();

    let mut sum = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if field[y][x] != 'A' as u8 {
                continue;
            }
            let cross = (
                field[y - 1][x - 1] as char,
                field[y + 1][x + 1] as char,
                field[y - 1][x + 1] as char,
                field[y + 1][x - 1] as char,
            );

            sum += match cross {
                ('M', 'S', 'M', 'S') => 1,
                ('M', 'S', 'S', 'M') => 1,
                ('S', 'M', 'S', 'M') => 1,
                ('S', 'M', 'M', 'S') => 1,
                _ => 0,
            }
        }
    }

    return sum;
}

fn main() {
    // let input = "
    // MMMSXXMASM
    // MSAMXMSMSA
    // AMXSXMAAMM
    // MSAMASMSMX
    // XMASAMXAMM
    // XXAMMXXAMA
    // SMSMSASXSS
    // SAXAMASAAA
    // MAMMMXMMMM
    // MXMXAXMASX
    // "
    // .trim();

    let input = std::fs::read_to_string("input").expect("could not read file");

    let field: Vec<&[u8]> = input
        .trim()
        .split("\n")
        .map(|n| n.trim())
        .map(|n| n.as_bytes())
        .collect();

    println!("Solution 1: {}", solution_1(&field));
    println!("Solution 2: {}", solution_2(&field));
}
