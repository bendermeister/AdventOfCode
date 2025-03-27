use regex::Regex;

fn solve1(input: &str) -> isize {
    let re = Regex::new(r#"mul\(\d+,\d+\)"#).unwrap();
    re.find_iter(input)
        .map(|m| m.as_str())
        .map(|s| &s[4..s.len() - 1])
        .map(|s| {
            s.split(",")
                .map(|n| n.parse::<isize>().unwrap())
                .product::<isize>()
        })
        .sum()
}

fn solve2(input: &str) -> isize {
    input
        .split("do()")
        .map(|s| s.split("don't()").next().unwrap())
        .map(|s| solve1(s))
        .sum()
}

#[test]
fn test_solve1() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let solution = solve1(input);
    assert_eq!(161, solution);
}

#[test]
fn test_solve2() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let solution = solve2(input);
    assert_eq!(48, solution);
}

pub fn solution() -> (isize, isize) {
    let input = std::fs::read_to_string("input/d03.txt").expect("Could not read file");
    (solve1(&input), solve2(&input))
}
