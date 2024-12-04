use regex::Regex;
use std::fs;

fn solution_1(s: &str) -> i32 {
    let mul_regex = Regex::new("mul\\([0-9][0-9]*,[0-9][0-9]*\\)").expect("to dumb to write regex");
    let num_regex = Regex::new("[0-9][0-9]*").expect("to dumb to write regex");

    mul_regex
        .find_iter(s)
        .map(|m| m.as_str())
        .map(|m| {
            num_regex
                .find_iter(m)
                .map(|m| m.as_str())
                .map(|m| m.parse::<i32>().expect("could not parse"))
                .product::<i32>()
        })
        .sum()
}

fn solution_2(s: &str) -> i32 {
    s.split("do()")
        .map(|m| m.split("don't()").next())
        .map(|m| match m {
            Some(x) => x,
            None => "", // this branch should not be possible
        })
        .map(|m| solution_1(m))
        .sum()
}

fn main() {
    let text = fs::read_to_string("input").expect("could not read file");
    //let text = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    println!("Solution 1: {}", solution_1(&text));
    println!("Solution 2: {}", solution_2(&text));
}
