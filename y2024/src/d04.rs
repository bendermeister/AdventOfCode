use share::point::Point;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect()
}

fn directions() -> [Point; 8] {
    [
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(1, 0),
        Point::new(1, -1),
        Point::new(0, -1),
        Point::new(-1, -1),
        Point::new(-1, 0),
        Point::new(-1, 1),
    ]
}

fn solve1(input: &str) -> isize {
    let puzzle = parse_input(input);

    let height = puzzle.len();
    let width = puzzle[0].len();

    let positions = (0..height)
        .map(|y| (0..width).map(move |x| (x, y)))
        .flatten()
        .map(|(x, y)| Point::new(x as isize, y as isize));

    let pget = |p: Point| match p.as_coords_with_bound(width, height) {
        Some((x, y)) => Some(puzzle[y][x]),
        None => None,
    };

    let directions = directions();

    let mut count = 0;
    for orig in positions {
        for dir in directions.iter() {
            count += match (
                pget(orig + dir.scale(0)),
                pget(orig + dir.scale(1)),
                pget(orig + dir.scale(2)),
                pget(orig + dir.scale(3)),
            ) {
                (Some('X'), Some('M'), Some('A'), Some('S')) => 1,
                _ => 0,
            }
        }
    }
    return count;
}

fn solve2(input: &str) -> isize {
    let puzzle = parse_input(input);

    let height = puzzle.len();
    let width = puzzle[0].len();

    let positions = (1..height - 1)
        .map(|y| (1..width - 1).map(move |x| (x, y)))
        .flatten()
        .filter(|(x, y)| puzzle[*y][*x] == 'A');

    let mut count = 0;
    for (x, y) in positions {
        count += match (
            puzzle[y - 1][x - 1],
            puzzle[y + 1][x + 1],
            puzzle[y - 1][x + 1],
            puzzle[y + 1][x - 1],
        ) {
            ('M', 'S', 'M', 'S') => 1,
            ('S', 'M', 'M', 'S') => 1,
            ('M', 'S', 'S', 'M') => 1,
            ('S', 'M', 'S', 'M') => 1,
            _ => 0,
        };
    }

    return count;
}

#[test]
fn test_solve1() {
    let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    let solution = solve1(input);
    assert_eq!(18, solution);
}

#[test]
fn test_solve2() {
    let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    let solution = solve2(input);
    assert_eq!(9, solution);
}

pub fn solution() -> (isize, isize) {
    let input = std::fs::read_to_string("input/d04.txt").expect("could not read file");
    (solve1(&input), solve2(&input))
}
