mod problem {
    use std::collections::HashMap;
    fn stones_dump(stones: &Vec<usize>) {
        for s in stones.iter() {
            print!("{s} ");
        }
        println!();
    }

    enum RuleResult {
        One(usize),
        Two(usize, usize),
    }
    struct Blinker {
        cash: HashMap<usize, RuleResult>,
    }

    impl Blinker {
        fn apply_rules(&mut self, stone: usize, result: &mut Vec<usize>) {
            match self.cash.get(&stone) {
                Some(r) => {
                    match r {
                        RuleResult::One(v) => {
                            result.push(*v);
                        }
                        RuleResult::Two(a, b) => {
                            result.push(*a);
                            result.push(*b);
                        }
                    }
                    return;
                }
                None => (),
            }

            // first rule
            if stone == 0 {
                self.cash.insert(0, RuleResult::One(1));
                result.push(1);
                return;
            }
            // second rule
            let num_digits = count_digits(stone);
            if is_even(num_digits) {
                let (upper, lower) = split_n_digits(stone, num_digits);
                self.cash.insert(stone, RuleResult::Two(upper, lower));

                result.push(upper);
                result.push(lower);
                return;
            }
            // third rule
            result.push(stone * 2024);
            self.cash.insert(stone, RuleResult::One(stone * 2024));
        }

        fn blink(&mut self, stones: &Vec<usize>, result: &mut Vec<usize>) {
            for stone in stones {
                self.apply_rules(*stone, result);
            }
        }

        fn new() -> Blinker {
            Blinker {
                cash: HashMap::new(),
            }
        }
    }

    fn count_digits(u: usize) -> usize {
        let mut n = 1;
        let mut d = 10;
        while u / d != 0 {
            d *= 10;
            n += 1;
        }
        n
    }

    /// Returns (upper_half, lower_half)
    fn split_n_digits(num: usize, n: usize) -> (usize, usize) {
        let mut d = 10;
        for _ in 1..n / 2 {
            d *= 10;
        }
        (num / d, num % d)
    }

    fn is_even(u: usize) -> bool {
        (u & 1) == 0
    }

    pub fn solve(input: &str, n_blinks: usize) {
        let mut stones: Vec<usize> = input
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let mut result = vec![];
        result.reserve(stones.len());
        let mut blinker = Blinker::new();

        for i in 0..n_blinks {
            println!("Iteration: {i}\t\tstones.len(): {}", stones.len());
            blinker.blink(&stones, &mut result);
            let tmp = result;
            result = stones;
            stones = tmp;
            result.clear();
        }
        println!("Solution 1: {}", stones.len());
    }
}

fn main() {
    //let input = "125 17";
    let input = &std::fs::read_to_string("input").unwrap();
    let input = input.trim();
    problem::solve(input, 25);
    problem::solve(input, 75);
}
