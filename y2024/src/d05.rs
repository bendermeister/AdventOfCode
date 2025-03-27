use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(input: &str) -> (HashMap<isize, HashSet<isize>>, Vec<Vec<isize>>) {
    let mut input = input.trim().split("\n\n");
    let input_rules = input.next().unwrap();
    let input_updates = input.next().unwrap();

    let mut rules: HashMap<isize, HashSet<isize>> = HashMap::new();

    for rule in input_rules.trim().lines() {
        let mut rule = rule.split("|").map(|n| n.parse().unwrap());
        let l = rule.next().unwrap();
        let r = rule.next().unwrap();
        rules.entry(l).or_default().insert(r);
    }

    let updates: Vec<Vec<isize>> = input_updates
        .trim()
        .lines()
        .map(|l| l.trim().split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn is_valid_ordering(rules: &HashMap<isize, HashSet<isize>>, update: &[isize]) -> bool {
    let mut acc = HashSet::new();

    for page in update.iter() {
        if let Some(rule) = rules.get(page) {
            if rule.intersection(&acc).next().is_some() {
                return false;
            }
        }
        acc.insert(*page);
    }

    true
}

fn solve1(input: &str) -> isize {
    let (rules, updates) = parse_input(input);
    updates
        .iter()
        .filter(|update| is_valid_ordering(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn solve2(input: &str) -> isize {
    let (rules, mut updates) = parse_input(input);
    updates
        .iter_mut()
        .filter(|update| !is_valid_ordering(&rules, update))
        .map(|update| {
            update.sort_by(|a, b| {
                if let Some(rule) = rules.get(a) {
                    if rule.contains(b) {
                        return Ordering::Less;
                    }
                }
                if let Some(rule) = rules.get(b) {
                    if rule.contains(a) {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            });
            update
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

#[test]
fn test_solve2() {
    let input = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    let solution = solve2(input);
    assert_eq!(123, solution);
}

#[test]
fn test_solve1() {
    let input = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    let solution = solve1(input);
    assert_eq!(143, solution);
}

pub fn solution() -> (isize, isize) {
    let input = std::fs::read_to_string("input/d05.txt").unwrap();
    (solve1(&input), solve2(&input))
}
