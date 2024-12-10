use std::collections::HashMap;
use std::collections::HashSet;
fn main() {
    // let rules = "
    // 47|53
    // 97|13
    // 97|61
    // 97|47
    // 75|29
    // 61|13
    // 75|53
    // 29|13
    // 97|29
    // 53|29
    // 61|53
    // 97|53
    // 61|29
    // 47|13
    // 75|47
    // 97|75
    // 47|61
    // 75|61
    // 47|29
    // 75|13
    // 53|13
    // ";

    // let updates = "
    // 75,47,61,53,29
    // 97,61,53,29,13
    // 75,29,13
    // 75,97,47,61,53
    // 61,13,29
    // 97,13,75,29,47
    // ";

    let rules = &std::fs::read_to_string("input/rules").expect("could not read file")[..];
    let updates = &std::fs::read_to_string("input/updates").expect("could not read file")[..];

    let mut map = HashMap::<i32, HashSet<i32>>::new();

    for line in rules.trim().lines() {
        let line: Vec<i32> = line.trim().split("|").map(|n| n.parse().unwrap()).collect();
        map.entry(line[0]).or_default().insert(line[1]);
    }
    // removing mut from map
    let rules = map;

    let mut updates: Vec<Vec<i32>> = updates
        .trim()
        .lines()
        .map(|l| l.trim().split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    /** Checks whether or not a given update is correct */
    fn is_correct(rules: &HashMap<i32, HashSet<i32>>, update: &[i32]) -> bool {
        let mut set = HashSet::new();

        for page in update.iter() {
            let rules = match rules.get(&page) {
                Some(r) => r,
                None => &HashSet::new(),
            };
            if !rules.is_disjoint(&set) {
                return false;
            }
            set.insert(*page);
        }
        true
    }

    let solution_1: i32 = updates
        .iter()
        .filter(|u| is_correct(&rules, u))
        .map(|u| u[u.len() / 2])
        .sum();

    println!("Solution 1: {}", solution_1);

    // solution 2
    let is_before = |a: &i32, b: &i32| match rules.get(a) {
        None => std::cmp::Ordering::Equal,
        Some(set) => {
            if set.contains(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }
    };

    let solution_2: i32 = updates
        .iter_mut()
        .filter(|u| !is_correct(&rules, u))
        .map(|u| {
            u.sort_by(is_before);
            u
        })
        .map(|u| u[u.len() / 2])
        .sum();

    println!("Solution 2: {}", solution_2);
}
