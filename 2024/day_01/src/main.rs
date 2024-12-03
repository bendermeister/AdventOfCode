use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("input").expect("Could not open File 'input'");
    let reader = BufReader::new(file);

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    let mut rmap: HashMap<i64, i64> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("could not read line");
        let nums: Vec<&str> = line.split("   ").collect();

        let l = match nums.get(0) {
            Some(x) => x.trim().parse().expect("could not parse nubmer"),
            None => continue,
        };
        let r = match nums.get(1) {
            Some(x) => x.trim().parse().expect("could not parse nubmer"),
            None => continue,
        };
        left.push(l);
        right.push(r);

        if let Some(v) = rmap.get_mut(&r) {
            *v += 1;
        } else {
            rmap.insert(r, 1);
        }
    }

    left.sort();
    right.sort();

    let sum: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
    println!("First part: {sum}");

    let sum: i64 = left
        .iter()
        .map(|l| {
            l * match rmap.get(&l) {
                Some(r) => *r,
                None => 0,
            }
        })
        .sum();

    println!("Second part: {sum}");
}
