use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn is_safe(v: &[i32]) -> bool {
    let (dist, asc, desc) = v
        .windows(2)
        .map(|w| ((w[0] - w[1]).abs(), w[0] <= w[1], w[0] >= w[1]))
        .map(|(x, y, z)| (1 <= x && x <= 3, y, z))
        .fold((true, true, true), |(ax, ay, az), (x, y, z)| {
            (ax && x, ay && y, az && z)
        });
    dist && (asc || desc)
}

fn is_safe_with_damp(v: &Vec<i32>) -> bool {
    for i in 0..v.len() {
        let mut v = v.clone();
        v.remove(i);
        if is_safe(&v) {
            return true;
        }
    }
    false
}

fn to_vec(s: String) -> Vec<i32> {
    s.trim()
        .split(" ")
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

fn main() {
    // Solution 1
    {
        let file = File::open("input").expect("could not open file");
        let reader = BufReader::new(file);

        let solution = reader
            .lines()
            .map(|l| l.unwrap())
            .map(|l| to_vec(l))
            .filter(|v| is_safe(v))
            .count();

        println!("Solution 1: {solution}");
    }

    // Solution 2
    {
        let file = File::open("input").expect("could not open file");
        let reader = BufReader::new(file);

        let solution = reader
            .lines()
            .map(|l| l.unwrap())
            .map(|l| to_vec(l))
            .filter(|v| is_safe(v) || is_safe_with_damp(v))
            .count();

        println!("Solution 2: {solution}");
    }

    {}
}
