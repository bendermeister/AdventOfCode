use std::collections::HashMap;

fn solve1(input: &str) -> isize {

    let input = input
        .trim()
        .lines()
        .map(|l| l.trim().split("   ").map(|n| n.parse::<isize>().unwrap()));

    let mut v0 = Vec::new();
    let mut v1 = Vec::new();

    for mut line in input {
        v0.push(line.next().unwrap());
        v1.push(line.next().unwrap());
    }

    v0.sort();
    v1.sort();

    v0.iter().zip(v1.iter()).map(|(n0, n1)| (n0 - n1).abs()).sum()
}

fn solve2(input: &str) -> isize {
    let input = input
        .trim()
        .lines()
        .map(|l| l.trim().split("   ").map(|n| n.parse::<isize>().unwrap()));

    let mut v = Vec::new();
    let mut h: HashMap<isize, isize> = HashMap::new();

    for mut line in input {
        v.push(line.next().unwrap());
        let n1 = line.next().unwrap();
        *h.entry(n1).or_insert(0) += 1;
    }

    v.iter().map(|n| *n * h.get(n).map_or(0, |n| *n)).sum()
}

pub fn solution() -> (isize, isize) {
    let input = std::fs::read_to_string("input/d01.txt").expect("Could not read input file");
    (solve1(&input), (solve2(&input)))
}

#[test]
fn test_solve1() {
    let input = "
3   4
4   3
2   5
1   3
3   9
3   3
";
    let solution = solve1(input);
    assert_eq!(11, solution);
}

#[test]
fn test_solve2() {
    let input = "
3   4
4   3
2   5
1   3
3   9
3   3
";
    let solution = solve2(input);
    assert_eq!(31, solution);
}
