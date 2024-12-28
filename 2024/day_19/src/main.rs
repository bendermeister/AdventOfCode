use std::collections::HashMap;

fn chop<'a>(pattern: &'a str, towel: &str) -> Option<&'a str> {
    if pattern.len() < towel.len() {
        return None;
    }
    if &pattern[..towel.len()] == towel {
        return Some(&pattern[towel.len()..]);
    }
    return None;
}

fn build<'a>(cache: &mut HashMap<&'a str, usize>, pattern: &'a str, towels: &[&str]) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(value) = cache.get(pattern) {
        return *value;
    }
    let mut sum = 0;

    for towel in towels {
        if let Some(pattern) = chop(pattern, towel) {
            sum += build(cache, pattern, towels);
        }
    }
    cache.insert(pattern, sum);
    return sum;
}

fn main() {
    #[allow(unused)]
    let input = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    let input = &std::fs::read_to_string("input").unwrap();

    let mut input = input.trim().split("\n\n");
    let towels = input.next().unwrap();
    let patterns = input.next().unwrap();

    let towels: Vec<&str> = towels.trim().split(", ").map(|t| t.trim()).collect();
    let patterns = patterns.trim().lines().map(|l| l.trim());

    let mut cache = HashMap::new();

    let mut builds = Vec::new();

    for pattern in patterns {
        let build = build(&mut cache, pattern, &towels);
        builds.push(build);
    }
    println!("Solution 1: {}", builds.iter().filter(|n| **n > 0).count());
    println!("Solution 2: {}", builds.iter().map(|n| *n).sum::<usize>());
}
