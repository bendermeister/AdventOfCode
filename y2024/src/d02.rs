fn is_safe(line: &[isize]) -> bool {
    let (dist, asc, desc) = line
        .windows(2)
        .map(|w| ((w[0] - w[1]).abs(), w[0] < w[1], w[0] > w[1]))
        .fold((0, true, true), |acc, n| {
            (acc.0.max(n.0), acc.1 && n.1, acc.2 && n.2)
        });

    return dist <= 3 && (asc || desc);
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve1(input: &str) -> isize {
    parse_input(input).iter().filter(|l| is_safe(l)).count() as isize
}

fn solve2(input: &str) -> isize {
    let lines = parse_input(input);

    let mut solution = 0;
    for line in lines.iter() {
        for i in 0..line.len() {
            let mut line = line.clone();
            line.remove(i);
            if is_safe(&line) {
                solution += 1;
                break;
            }
        }
    }
    return solution;
}

#[test]
fn test_solve1() {
    let input = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let solution = solve1(input);
    assert_eq!(2, solution);
}

#[test]
fn test_solve2() {
    let input = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let solution = solve2(input);
    assert_eq!(4, solution);
}

pub fn solution() -> (isize, isize) {
    let input = std::fs::read_to_string("input/d02.txt").expect("Could not read file");
    (solve1(&input), (solve2(&input)))
}
