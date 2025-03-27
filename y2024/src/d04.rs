fn solve1(input: &str) -> isize {
    todo!()
}

fn solve2(input: &str) -> isize {
    todo!()
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
    todo!()
}

pub fn solution() -> (isize, isize) {
    let input = std::fs::read_to_string("input/d04.txt").expect("could not read file");
    (solve1(&input), solve2(&input))
}
